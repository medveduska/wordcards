//! Read the color information that Calamine deliberately leaves out of cell values.
//! Only direct font colors, rich-text colors, and solid fills are imported.
use std::collections::HashMap;
use std::io::{Cursor, Read};

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use zip::ZipArchive;

use crate::formatting::{CellFormatting, RgbColor, TextRun};
use crate::xlsx_text::decode_text;

pub type SheetFormatting = HashMap<(u32, u16), CellFormatting>;

#[derive(Default)]
struct Element {
    name: String,
    attrs: HashMap<String, String>,
    children: Vec<Element>,
    text: String,
}

impl Element {
    fn attr(&self, name: &str) -> Option<&str> {
        self.attrs.get(name).map(String::as_str)
    }

    fn child(&self, name: &str) -> Option<&Self> {
        self.children.iter().find(|child| child.name == name)
    }

    fn children_named<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a Self> {
        self.children.iter().filter(move |child| child.name == name)
    }

    fn index(&self, name: &str) -> usize {
        self.attr(name)
            .and_then(|value| value.parse().ok())
            .unwrap_or(0)
    }
}

fn element(
    reader: &mut Reader<&[u8]>,
    start: BytesStart<'_>,
    depth: usize,
) -> Result<Element, String> {
    if depth > 64 {
        return Err("Excel formatting contains excessively nested XML.".into());
    }
    let mut node = Element {
        name: String::from_utf8_lossy(start.local_name().as_ref()).into_owned(),
        ..Default::default()
    };
    for attr in start.attributes() {
        let attr = attr.map_err(|error| error.to_string())?;
        node.attrs.insert(
            String::from_utf8_lossy(attr.key.as_ref()).into_owned(),
            attr.decoded_and_normalized_value(quick_xml::XmlVersion::Implicit1_0, reader.decoder())
                .map_err(|error| error.to_string())?
                .into_owned(),
        );
    }
    loop {
        match reader.read_event().map_err(|error| error.to_string())? {
            Event::Start(start) => node.children.push(element(reader, start, depth + 1)?),
            Event::Text(text) => node
                .text
                .push_str(&text.xml10_content().map_err(|error| error.to_string())?),
            Event::CData(text) => node
                .text
                .push_str(&text.xml10_content().map_err(|error| error.to_string())?),
            Event::GeneralRef(reference) => {
                if let Some(ch) = reference
                    .resolve_char_ref()
                    .map_err(|error| error.to_string())?
                {
                    node.text.push(ch);
                } else {
                    let name = reference.decode().map_err(|error| error.to_string())?;
                    node.text.push_str(
                        quick_xml::escape::resolve_predefined_entity(&name)
                            .ok_or_else(|| format!("Unknown XML entity: {name}"))?,
                    );
                }
            }
            Event::End(_) => return Ok(node),
            Event::Eof => return Err("Incomplete Excel formatting XML.".into()),
            _ => {}
        }
    }
}

// Visit one shared string or worksheet cell at a time, keeping sparse sheets sparse.
fn visit(
    xml: &str,
    name: &str,
    mut visitor: impl FnMut(Element) -> Result<(), String>,
) -> Result<(), String> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().expand_empty_elements = true;
    loop {
        match reader.read_event().map_err(|error| error.to_string())? {
            Event::Start(start) if start.local_name().as_ref() == name.as_bytes() => {
                visitor(element(&mut reader, start, 0)?)?;
            }
            Event::Eof => return Ok(()),
            _ => {}
        }
    }
}

fn document(xml: &str) -> Result<Element, String> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().expand_empty_elements = true;
    loop {
        match reader.read_event().map_err(|error| error.to_string())? {
            Event::Start(start) => return element(&mut reader, start, 0),
            Event::Eof => return Err("Missing Excel XML document.".into()),
            _ => {}
        }
    }
}

fn read_part(zip: &mut ZipArchive<Cursor<&[u8]>>, path: &str) -> Result<Option<String>, String> {
    let mut file = match zip.by_name(path) {
        Ok(file) => file,
        Err(zip::result::ZipError::FileNotFound) => return Ok(None),
        Err(error) => return Err(error.to_string()),
    };
    let mut xml = String::new();
    file.read_to_string(&mut xml)
        .map_err(|error| error.to_string())?;
    Ok(Some(xml))
}

fn resolve_part(base: &str, target: &str) -> String {
    let parent = base
        .rsplit_once('/')
        .map(|(parent, _)| parent)
        .unwrap_or("");
    let joined = if target.starts_with('/') {
        target.to_string()
    } else {
        format!("{parent}/{target}")
    };
    let mut parts = Vec::new();
    for part in joined.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            _ => parts.push(part),
        }
    }
    parts.join("/")
}

fn relationships(zip: &mut ZipArchive<Cursor<&[u8]>>, base: &str) -> Result<Vec<Element>, String> {
    let path = match base.rsplit_once('/') {
        Some((parent, file)) => format!("{parent}/_rels/{file}.rels"),
        None => format!("_rels/{base}.rels"),
    };
    Ok(read_part(zip, &path)?
        .map(|xml| document(&xml))
        .transpose()?
        .unwrap_or_default()
        .children)
}

const DEFAULT_THEME: [&str; 12] = [
    "FFFFFF", "000000", "E7E6E6", "44546A", "4472C4", "ED7D31", "A5A5A5", "FFC000", "5B9BD5",
    "70AD47", "0563C1", "954F72",
];

const INDEXED_COLORS: [&str; 64] = [
    "000000", "FFFFFF", "FF0000", "00FF00", "0000FF", "FFFF00", "FF00FF", "00FFFF", "000000",
    "FFFFFF", "FF0000", "00FF00", "0000FF", "FFFF00", "FF00FF", "00FFFF", "800000", "008000",
    "000080", "808000", "800080", "008080", "C0C0C0", "808080", "9999FF", "993366", "FFFFCC",
    "CCFFFF", "660066", "FF8080", "0066CC", "CCCCFF", "000080", "FF00FF", "FFFF00", "00FFFF",
    "800080", "800000", "008080", "0000FF", "00CCFF", "CCFFFF", "CCFFCC", "FFFF99", "99CCFF",
    "FF99CC", "CC99FF", "FFCC99", "3366FF", "33CCCC", "99CC00", "FFCC00", "FF9900", "FF6600",
    "666699", "969696", "003366", "339966", "003300", "333300", "993300", "993366", "333399",
    "333333",
];

struct Palette {
    theme: Vec<RgbColor>,
    indexed: Vec<RgbColor>,
}

impl Palette {
    fn color(&self, node: &Element) -> Option<RgbColor> {
        if node.attr("auto") == Some("1") {
            return None;
        }
        let color = if let Some(rgb) = node.attr("rgb") {
            RgbColor::from_hex(rgb)
        } else if let Some(index) = node
            .attr("theme")
            .and_then(|index| index.parse::<usize>().ok())
        {
            self.theme.get(index).copied()
        } else {
            node.attr("indexed")
                .and_then(|index| index.parse::<usize>().ok())
                .and_then(|index| self.indexed.get(index).copied())
        }?;
        let tint = node
            .attr("tint")
            .and_then(|tint| tint.parse::<f64>().ok())
            .filter(|tint| tint.is_finite())
            .unwrap_or(0.0)
            .clamp(-1.0, 1.0);
        Some(tint_color(color, tint))
    }

    fn font_color(&self, node: &Element) -> Option<RgbColor> {
        let color = node.child("color")?;
        // Automatic/default font color should follow the app's normal text color.
        if color.attr("theme") == Some("1") && self.color(color) == RgbColor::from_hex("000000") {
            return None;
        }
        self.color(color)
    }
}

// Excel applies tint to HSL luminance, preserving hue and saturation.
fn tint_color(color: RgbColor, tint: f64) -> RgbColor {
    if tint == 0.0 {
        return color;
    }
    let rgb = u32::from(color);
    let [r, g, b] = [16, 8, 0].map(|shift| ((rgb >> shift) & 255) as f64 / 255.0);
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;
    let lightness = (max + min) / 2.0;
    let saturation = if delta == 0.0 {
        0.0
    } else {
        delta / (1.0 - (2.0 * lightness - 1.0).abs())
    };
    let hue = if delta == 0.0 {
        0.0
    } else if max == r {
        ((g - b) / delta).rem_euclid(6.0)
    } else if max == g {
        (b - r) / delta + 2.0
    } else {
        (r - g) / delta + 4.0
    };
    let lightness = if tint < 0.0 {
        lightness * (1.0 + tint)
    } else {
        lightness * (1.0 - tint) + tint
    };
    let chroma = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
    let x = chroma * (1.0 - (hue.rem_euclid(2.0) - 1.0).abs());
    let channels = match hue as u32 {
        0 => [chroma, x, 0.0],
        1 => [x, chroma, 0.0],
        2 => [0.0, chroma, x],
        3 => [0.0, x, chroma],
        4 => [x, 0.0, chroma],
        _ => [chroma, 0.0, x],
    };
    let rgb = channels.into_iter().fold(0, |rgb, channel| {
        (rgb << 8)
            | ((channel + lightness - chroma / 2.0) * 255.0)
                .round()
                .clamp(0.0, 255.0) as u32
    });
    rgb.try_into().expect("RGB channels are in range")
}

fn rich_runs(node: &Element, palette: &Palette) -> Vec<TextRun> {
    let runs: Vec<_> = node
        .children
        .iter()
        .filter_map(|child| {
            let (text, color) = match child.name.as_str() {
                "t" => (
                    decode_text(&child.text, child.attr("xml:space") == Some("preserve")),
                    None,
                ),
                "r" => (
                    {
                        let text = child.child("t")?;
                        decode_text(&text.text, text.attr("xml:space") == Some("preserve"))
                    },
                    child
                        .child("rPr")
                        .and_then(|props| props.child("color"))
                        .and_then(|color| palette.color(color)),
                ),
                _ => return None, // Exclude phonetic annotations.
            };
            (!text.is_empty()).then_some(TextRun { text, color })
        })
        .collect();
    if runs.iter().any(|run| run.color.is_some()) {
        runs
    } else {
        Vec::new()
    }
}

pub fn read_sheet_formatting(bytes: &[u8]) -> Result<SheetFormatting, String> {
    let mut zip = ZipArchive::new(Cursor::new(bytes)).map_err(|error| error.to_string())?;
    let workbook = read_part(&mut zip, "_rels/.rels")?
        .map(|xml| document(&xml))
        .transpose()?
        .and_then(|root| {
            root.children.into_iter().find(|rel| {
                rel.attr("Type")
                    .is_some_and(|kind| kind.ends_with("/officeDocument"))
            })
        })
        .and_then(|rel| rel.attr("Target").map(|target| resolve_part("", target)))
        .unwrap_or_else(|| "xl/workbook.xml".into());
    let rels = relationships(&mut zip, &workbook)?;
    let related = |kind: &str| {
        rels.iter()
            .find(|rel| rel.attr("Type").is_some_and(|value| value.ends_with(kind)))
            .and_then(|rel| rel.attr("Target"))
            .map(|target| resolve_part(&workbook, target))
    };
    let mut palette = Palette {
        theme: DEFAULT_THEME
            .iter()
            .filter_map(|color| RgbColor::from_hex(color))
            .collect(),
        indexed: INDEXED_COLORS
            .iter()
            .filter_map(|color| RgbColor::from_hex(color))
            .collect(),
    };
    if let Some(path) = related("/theme") {
        if let Some(xml) = read_part(&mut zip, &path)? {
            visit(&xml, "clrScheme", |scheme| {
                for (index, name) in [
                    "lt1", "dk1", "lt2", "dk2", "accent1", "accent2", "accent3", "accent4",
                    "accent5", "accent6", "hlink", "folHlink",
                ]
                .iter()
                .enumerate()
                {
                    if let Some(color) = scheme
                        .child(name)
                        .and_then(|node| node.children.first())
                        .and_then(|node| node.attr("lastClr").or_else(|| node.attr("val")))
                        .and_then(RgbColor::from_hex)
                    {
                        palette.theme[index] = color;
                    }
                }
                Ok(())
            })?;
        }
    }
    let mut styles = Vec::new();
    let style_path = related("/styles").unwrap_or_else(|| resolve_part(&workbook, "styles.xml"));
    if let Some(xml) = read_part(&mut zip, &style_path)? {
        let root = document(&xml)?;
        if let Some(indexed) = root
            .child("colors")
            .and_then(|node| node.child("indexedColors"))
        {
            for (index, node) in indexed.children.iter().enumerate() {
                if let Some(color) = node.attr("rgb").and_then(RgbColor::from_hex) {
                    if let Some(entry) = palette.indexed.get_mut(index) {
                        *entry = color;
                    }
                }
            }
        }
        let fonts: Vec<_> = root
            .child("fonts")
            .map(|node| {
                node.children
                    .iter()
                    .map(|font| palette.font_color(font))
                    .collect()
            })
            .unwrap_or_default();
        let fills: Vec<_> = root
            .child("fills")
            .map(|node| {
                node.children
                    .iter()
                    .map(|fill| {
                        let pattern = fill.child("patternFill")?;
                        if pattern.attr("patternType") != Some("solid") {
                            return None;
                        }
                        palette.color(pattern.child("fgColor")?)
                    })
                    .collect()
            })
            .unwrap_or_default();
        if let Some(xfs) = root.child("cellXfs") {
            for xf in &xfs.children {
                styles.push(CellFormatting {
                    text_color: fonts.get(xf.index("fontId")).copied().flatten(),
                    background_color: fills.get(xf.index("fillId")).copied().flatten(),
                    ..Default::default()
                });
            }
        }
    }
    let mut shared = Vec::new();
    let shared_path =
        related("/sharedStrings").unwrap_or_else(|| resolve_part(&workbook, "sharedStrings.xml"));
    if let Some(xml) = read_part(&mut zip, &shared_path)? {
        visit(&xml, "si", |node| {
            shared.push(rich_runs(&node, &palette));
            Ok(())
        })?;
    }
    let root = document(&read_part(&mut zip, &workbook)?.ok_or("Missing workbook XML.")?)?;
    let sheet = root
        .child("sheets")
        .and_then(|sheets| sheets.children_named("sheet").next())
        .ok_or("Missing first worksheet.")?;
    let relation_id = sheet
        .attrs
        .iter()
        .find(|(key, _)| key.ends_with(":id"))
        .map(|(_, value)| value.as_str());
    let path = rels
        .iter()
        .find(|rel| rel.attr("Id") == relation_id)
        .and_then(|rel| rel.attr("Target"))
        .map(|target| resolve_part(&workbook, target))
        .ok_or("Missing worksheet relationship.")?;
    let xml = read_part(&mut zip, &path)?.ok_or("Missing worksheet XML.")?;
    let mut cells = HashMap::new();
    visit(&xml, "c", |cell| {
        let Some((row, column)) = cell.attr("r").and_then(cell_position) else {
            return Ok(());
        };
        if column > 3 {
            return Ok(());
        }
        let mut style = styles.get(cell.index("s")).cloned().unwrap_or_default();
        style.runs = match cell.attr("t") {
            Some("s") => cell
                .child("v")
                .and_then(|node| node.text.parse::<usize>().ok())
                .and_then(|index| shared.get(index))
                .cloned()
                .unwrap_or_default(),
            Some("inlineStr") => cell
                .child("is")
                .map(|node| rich_runs(node, &palette))
                .unwrap_or_default(),
            _ => Vec::new(),
        };
        for run in &mut style.runs {
            if run.color == style.text_color {
                run.color = None;
            }
        }
        if style.runs.iter().all(|run| run.color.is_none()) {
            style.runs.clear();
        }
        if !style.is_empty() {
            cells.insert((row, column), style);
        }
        Ok(())
    })?;
    Ok(cells)
}

fn cell_position(address: &str) -> Option<(u32, u16)> {
    let mut column = 0u16;
    let mut split = 0;
    for byte in address.bytes().take_while(u8::is_ascii_uppercase) {
        column = column
            .checked_mul(26)?
            .checked_add(u16::from(byte - b'A' + 1))?;
        split += 1;
    }
    Some((
        address[split..].parse::<u32>().ok()?.checked_sub(1)?,
        column.checked_sub(1)?,
    ))
}
