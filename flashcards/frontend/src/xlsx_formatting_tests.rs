use std::io::{Cursor, Read, Write};

use rust_xlsxwriter::{Color, Format, Workbook};
use zip::{write::SimpleFileOptions, ZipArchive, ZipWriter};

use crate::formatting::{CellFormatting, RgbColor, TextRun};
use crate::xlsx_export::export_flashcards_xlsx;
use crate::xlsx_io::parse_flashcards_from_xlsx;

fn color(hex: &str) -> Option<RgbColor> {
    RgbColor::from_hex(hex)
}

fn colored_workbook() -> Vec<u8> {
    let mut book = Workbook::new();
    let sheet = book.add_worksheet();
    for (column, header) in ["word", "pinyin", "translation", "example_sentence", "known"]
        .iter()
        .enumerate()
    {
        sheet.write_string(1, column as u16, *header).unwrap();
    }
    sheet
        .write_string_with_format(
            6,
            0,
            "你好",
            &Format::new()
                .set_font_color(Color::Red)
                .set_background_color(Color::RGB(0xFFF2CC)),
        )
        .unwrap();
    sheet
        .write_string_with_format(
            6,
            1,
            "nǐ hǎo",
            &Format::new().set_font_color(Color::RGB(0x0000FF)),
        )
        .unwrap();
    sheet
        .write_string_with_format(
            6,
            2,
            "Hello",
            &Format::new()
                .set_font_color(Color::RGB(0xFFFFFF))
                .set_background_color(Color::RGB(0x203864)),
        )
        .unwrap();
    let default = Format::new().set_font_color(Color::Automatic);
    let red = Format::new().set_font_color(Color::Red);
    let blue = Format::new().set_font_color(Color::RGB(0x0000FF));
    sheet
        .write_rich_string_with_format(
            6,
            3,
            &[
                (&default, "Example: "),
                (&red, "你好"),
                (&default, " & <hello>\n"),
                (&blue, "世界！"),
            ],
            &Format::new().set_background_color(Color::RGB(0xE2F0D9)),
        )
        .unwrap();
    sheet.write_boolean(6, 4, true).unwrap();
    // The second worksheet must never influence the imported colors.
    book.add_worksheet()
        .write_string_with_format(
            6,
            0,
            "ignored",
            &Format::new().set_background_color(Color::Red),
        )
        .unwrap();
    book.save_to_buffer().unwrap()
}

fn rewrite_parts(bytes: Vec<u8>, replace: impl Fn(&str, String) -> String) -> Vec<u8> {
    let mut input = ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut output = ZipWriter::new(Cursor::new(Vec::new()));
    for index in 0..input.len() {
        let mut file = input.by_index(index).unwrap();
        let name = file.name().to_string();
        let mut xml = String::new();
        file.read_to_string(&mut xml).unwrap();
        output
            .start_file(&name, SimpleFileOptions::default())
            .unwrap();
        output.write_all(replace(&name, xml).as_bytes()).unwrap();
    }
    output.finish().unwrap().into_inner()
}

#[test]
fn imports_font_colors_solid_fills_and_rich_shared_strings_on_sparse_rows() {
    let cards = parse_flashcards_from_xlsx(&colored_workbook()).unwrap();
    assert_eq!(cards.len(), 1);
    let card = &cards[0];
    assert!(card.known);
    assert_eq!(card.formatting.word.text_color, color("FF0000"));
    assert_eq!(card.formatting.word.background_color, color("FFF2CC"));
    assert_eq!(card.formatting.pinyin.text_color, color("0000FF"));
    assert_eq!(card.formatting.translation.text_color, color("FFFFFF"));
    assert_eq!(
        card.formatting.translation.background_color,
        color("203864")
    );
    assert_eq!(
        card.formatting.example_sentence.background_color,
        color("E2F0D9")
    );
    let sentence = card.example_sentence.as_deref().unwrap();
    assert_eq!(sentence, "Example: 你好 & <hello>\n世界！");
    assert!(card.formatting.example_sentence.valid_runs(sentence));
    assert_eq!(
        card.formatting.example_sentence.runs[1].color,
        color("FF0000")
    );
    assert_eq!(
        card.formatting.example_sentence.runs[3].color,
        color("0000FF")
    );
}

#[test]
fn colors_survive_excel_export_reimport_and_json_storage() {
    let original = parse_flashcards_from_xlsx(&colored_workbook()).unwrap();
    let bytes = export_flashcards_xlsx(original.iter()).unwrap();
    assert_eq!(parse_flashcards_from_xlsx(&bytes).unwrap(), original);
    let saved = serde_json::to_string(&original).unwrap();
    let loaded: Vec<crate::model::Flashcard> = serde_json::from_str(&saved).unwrap();
    assert_eq!(loaded, original);
    // Inherited rich-text colors also survive without turning into separate overrides.
    let mut cards = original;
    cards[0].formatting.example_sentence.text_color = color("008000");
    let bytes = export_flashcards_xlsx(cards.iter()).unwrap();
    assert_eq!(parse_flashcards_from_xlsx(&bytes).unwrap(), cards);
}

#[test]
fn imports_inline_rich_text_and_excludes_phonetic_annotations() {
    let bytes = rewrite_parts(colored_workbook(), |name, xml| {
        if name != "xl/worksheets/sheet1.xml" {
            return xml;
        }
        r#"<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheetData><row r="7"><c r="A7" t="inlineStr"><is><r><rPr><color rgb="FFFF0000"/></rPr><t>你</t></r><r><t>好</t></r><rPh sb="0" eb="2"><t>nǐ hǎo</t></rPh></is></c><c r="C7" t="inlineStr"><is><t>Hello</t></is></c><c r="D7" t="inlineStr"><is><r><t xml:space="preserve">Hi &amp; </t></r><r><rPr><color rgb="FF0000FF"/></rPr><t>&lt;你好&gt;&#33;</t></r></is></c></row></sheetData></worksheet>"#.into()
    });
    let cards = parse_flashcards_from_xlsx(&bytes).unwrap();
    let card = &cards[0];
    assert_eq!(card.word, "你好");
    assert!(card.formatting.word.valid_runs(&card.word));
    assert_eq!(card.formatting.word.runs[0].color, color("FF0000"));
    assert_eq!(card.example_sentence.as_deref(), Some("Hi & <你好>!"));
    assert!(card.formatting.example_sentence.valid_runs("Hi & <你好>!"));
    assert_eq!(
        parse_flashcards_from_xlsx(&export_flashcards_xlsx(cards.iter()).unwrap()).unwrap(),
        cards
    );
}

#[test]
fn anquan_multiline_example_keeps_colors_with_excel_carriage_return_escapes() {
    for inline in [false, true] {
        let rich = r#"<r><rPr><color rgb="FFFF0000"/></rPr><t>Adj.</t></r><r><t>这里很</t></r><r><rPr><color rgb="FF6D9EEB"/></rPr><t>安全</t></r><r><t>。_x000D_
N.请注意</t></r><r><rPr><color indexed="10"/></rPr><t>安全</t></r><r><t>。</t></r>"#;
        let bytes = example_workbook(rich, inline);
        let cards = parse_flashcards_from_xlsx(&bytes).unwrap();
        let card = &cards[0];
        let text = card.example_sentence.as_deref().unwrap();
        assert_eq!(text, "Adj.这里很安全。\r\nN.请注意安全。");
        assert!(
            card.formatting.example_sentence.valid_runs(text),
            "Colored text differs from the imported example: {:?}",
            card.formatting.example_sentence.runs
        );
        assert_eq!(
            card.formatting.example_sentence.runs[0].color,
            color("FF0000")
        );
        assert_eq!(
            card.formatting.example_sentence.runs[2].color,
            color("6D9EEB")
        );
        assert_eq!(
            card.formatting.example_sentence.runs[4].color,
            color("FF0000")
        );
        let exported = export_flashcards_xlsx(cards.iter()).unwrap();
        assert_eq!(parse_flashcards_from_xlsx(&exported).unwrap(), cards);
    }
}

#[test]
fn anquan_example_preserves_rgb_and_theme_colors_with_windows_line_breaks() {
    // Reproduces the formatting and CRLF line ending in the reported D4 cell.
    let rich = "<r><rPr><color rgb=\"FFFF0000\"/></rPr><t>Adj</t></r><r><rPr><color theme=\"1\"/></rPr><t>.这里很安</t></r><r><rPr><color theme=\"3\" tint=\"0.59999389629810485\"/></rPr><t>全</t></r><r><rPr><color theme=\"1\"/></rPr><t xml:space=\"preserve\">。\r\n</t></r><r><rPr><color theme=\"3\" tint=\"0.59999389629810485\"/></rPr><t>N</t></r><r><rPr><color theme=\"1\"/></rPr><t>.请注意</t></r><r><rPr><color theme=\"5\" tint=\"-0.249977111117893\"/></rPr><t>安全</t></r><r><rPr><color theme=\"1\"/></rPr><t>。</t></r>";
    for inline in [false, true] {
        let cards = parse_flashcards_from_xlsx(&example_workbook(rich, inline)).unwrap();
        let card = &cards[0];
        let text = card.example_sentence.as_deref().unwrap();
        assert_eq!(text, "Adj.这里很安全。\nN.请注意安全。");
        assert!(card.formatting.example_sentence.valid_runs(text));
        let runs = &card.formatting.example_sentence.runs;
        assert_eq!(runs[0].color, color("FF0000"));
        assert_eq!(runs[1].color, color("000000"));
        assert_eq!(runs[2].color, color("8EB4E3"));
        assert_eq!(runs[4].color, color("8EB4E3"));
        assert_eq!(runs[6].color, color("953735"));
        let exported = export_flashcards_xlsx(cards.iter()).unwrap();
        assert_eq!(parse_flashcards_from_xlsx(&exported).unwrap(), cards);
    }
}

#[test]
fn rich_text_matches_the_cell_reader_whitespace_and_literal_escape_rules() {
    for inline in [false, true] {
        let rich = "<r><rPr><color rgb=\"FFFF0000\"/></rPr><t>  Adj.  </t></r><r><t xml:space=\"preserve\"> 这里很安全。\r\nN. </t></r><r><rPr><color rgb=\"FF0000FF\"/></rPr><t>请注意安全。_x005F_x000D_</t></r><r><t> \t </t></r>";
        let cards = parse_flashcards_from_xlsx(&example_workbook(rich, inline)).unwrap();
        let card = &cards[0];
        let text = card.example_sentence.as_deref().unwrap();
        assert_eq!(text, "Adj. 这里很安全。\nN. 请注意安全。_x000D_");
        assert!(card.formatting.example_sentence.valid_runs(text));
        let exported = export_flashcards_xlsx(cards.iter()).unwrap();
        assert_eq!(parse_flashcards_from_xlsx(&exported).unwrap(), cards);
    }
}

fn example_workbook(rich: &str, inline: bool) -> Vec<u8> {
    rewrite_parts(colored_workbook(), |name, xml| match name {
        "xl/worksheets/sheet1.xml" => {
            let example = if inline {
                format!(r#"<c r="D7" t="inlineStr"><is>{rich}</is></c>"#)
            } else {
                r#"<c r="D7" t="s"><v>0</v></c>"#.into()
            };
            format!(
                r#"<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheetData><row r="7"><c r="A7" t="inlineStr"><is><t>安全</t></is></c><c r="B7" t="inlineStr"><is><t>ānquán</t></is></c><c r="C7" t="inlineStr"><is><t>security</t></is></c>{example}</row></sheetData></worksheet>"#
            )
        }
        "xl/sharedStrings.xml" => format!(
            r#"<sst xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" count="1" uniqueCount="1"><si>{rich}</si></sst>"#
        ),
        _ => xml,
    })
}

#[test]
fn resolves_theme_tints_indexed_colors_and_absolute_relationship_targets() {
    let bytes = rewrite_parts(colored_workbook(), |name, xml| match name {
        "xl/styles.xml" => xml
            .replace("rgb=\"FFFF0000\"", "indexed=\"10\"")
            .replace("rgb=\"FFFFF2CC\"", "theme=\"4\" tint=\"0.4\""),
        "xl/_rels/workbook.xml.rels" => xml.replace(
            "Target=\"worksheets/sheet1.xml\"",
            "Target=\"/xl/worksheets/sheet1.xml\"",
        ),
        _ => xml,
    });
    let cards = parse_flashcards_from_xlsx(&bytes).unwrap();
    assert_eq!(cards[0].formatting.word.text_color, color("FF0000"));
    // rust_xlsxwriter uses the Office theme's accent1 4F81BD; a 40% tint is 95B3D7.
    assert_eq!(cards[0].formatting.word.background_color, color("95B3D7"));
}

#[test]
fn invalid_rich_text_metadata_falls_back_to_the_real_text_on_export() {
    let mut cards = parse_flashcards_from_xlsx(&colored_workbook()).unwrap();
    cards[0].formatting.word.runs = vec![TextRun {
        text: "wrong text".into(),
        color: color("0000FF"),
    }];
    let exported = export_flashcards_xlsx(cards.iter()).unwrap();
    let restored = parse_flashcards_from_xlsx(&exported).unwrap();
    assert_eq!(restored[0].word, "你好");
    assert!(restored[0].formatting.word.runs.is_empty());
    assert_eq!(restored[0].formatting.word.text_color, color("FF0000"));
}

#[test]
fn plain_csv_drops_colors_but_keeps_text() {
    let cards = parse_flashcards_from_xlsx(&colored_workbook()).unwrap();
    let bytes = crate::csv_io::export_flashcards_csv(cards.iter()).unwrap();
    let plain =
        crate::csv_io::parse_flashcards_from_csv(std::str::from_utf8(&bytes).unwrap()).unwrap();
    assert_eq!(plain[0].word, cards[0].word);
    assert_eq!(plain[0].example_sentence, cards[0].example_sentence);
    assert!(plain[0].formatting.is_empty());
}

#[test]
fn rejects_invalid_rgb_and_ignores_malformed_or_stale_text_runs() {
    for hex in [
        "not-a-color",
        "#FF0000;color:red",
        "éééé",
        "12345",
        "123456789",
    ] {
        assert!(RgbColor::from_hex(hex).is_none());
    }
    assert!(serde_json::from_str::<CellFormatting>(r#"{"text_color":4294967295}"#).is_err());
    let invalid = CellFormatting {
        runs: vec![TextRun {
            text: "你好".into(),
            color: color("FF0000"),
        }],
        ..Default::default()
    };
    assert!(!invalid.valid_runs("再见"));
}
