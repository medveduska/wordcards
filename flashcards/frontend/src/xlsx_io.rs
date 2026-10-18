use std::io::Cursor;

use calamine::{DataRef, Reader, Xlsx};

use crate::import_io::{header_layout, require_cards, validate_row, ImportLayout};
use crate::model::Flashcard;

pub fn parse_flashcards_from_xlsx(bytes: &[u8]) -> Result<Vec<Flashcard>, String> {
    let mut workbook = Xlsx::new(Cursor::new(bytes))
        .map_err(|_| "Could not open this XLSX file. It may be damaged, password-protected, or a different file type. Save it as an Excel Workbook (.xlsx) and try again.".to_string())?;
    let sheet = workbook
        .sheet_names()
        .first()
        .cloned()
        .ok_or_else(|| "The XLSX file has no worksheets.".to_string())?;
    let mut reader = workbook
        .worksheet_cells_reader(&sheet)
        .map_err(|_| "Could not read the first worksheet in this XLSX file.".to_string())?;
    let mut cards = Vec::new();
    let mut fields: [String; 5] = Default::default();
    let mut current_row = None;
    let mut first_row = true;
    let mut layout = ImportLayout::Auto;

    // Read sparse cells directly so distant cells cannot create a huge dense grid.
    while let Some(cell) = reader.next_cell().map_err(|_| {
        "The first XLSX worksheet contains unreadable data. Correct it in Excel and save again."
            .to_string()
    })? {
        let (row, column) = cell.get_position();
        if current_row.is_some_and(|previous| previous != row) {
            append_row(
                &mut cards,
                &fields,
                current_row.unwrap() as usize + 1,
                &mut first_row,
                &mut layout,
            )?;
            fields = Default::default();
        }
        current_row = Some(row);
        let value = match cell.get_value() {
            DataRef::Empty => continue,
            DataRef::String(value) => value.clone(),
            DataRef::SharedString(value) => value.to_string(),
            DataRef::Bool(value) if column == 4 || (column == 3 && !matches!(layout, ImportLayout::Examples)) => value.to_string(),
            _ => return Err(format!("Row {}, column {}: expected text (or true/false in known). Correct numeric, date, or error cells before importing.", row + 1, column + 1)),
        };
        if column >= 5 {
            if value.trim().is_empty() {
                continue;
            }
            return Err(format!("Row {}: unexpected data after column E. Use columns A–E for word, pinyin, translation, optional example sentence, and optional known.", row + 1));
        }
        fields[column as usize] = value;
    }
    if let Some(row) = current_row {
        append_row(
            &mut cards,
            &fields,
            row as usize + 1,
            &mut first_row,
            &mut layout,
        )?;
    }
    require_cards(cards)
}

fn append_row(
    cards: &mut Vec<Flashcard>,
    fields: &[String; 5],
    row: usize,
    first_row: &mut bool,
    layout: &mut ImportLayout,
) -> Result<(), String> {
    if fields.iter().all(|value| value.trim().is_empty()) {
        return Ok(());
    }
    let width = fields
        .iter()
        .rposition(|field| !field.is_empty())
        .map(|index| index + 1)
        .unwrap_or(3)
        .max(3);
    let fields: Vec<_> = fields[..width].iter().map(String::as_str).collect();
    if *first_row {
        if let Some(header) = header_layout(&fields) {
            *layout = header;
            *first_row = false;
            return Ok(());
        }
    }
    cards.push(validate_row(&fields, row, *layout)?);
    *first_row = false;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use zip::{write::SimpleFileOptions, CompressionMethod, ZipWriter};

    fn workbook(sheet_data: &str) -> Vec<u8> {
        let mut zip = ZipWriter::new(Cursor::new(Vec::new()));
        let sheet = format!(
            r#"<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheetData>{sheet_data}</sheetData></worksheet>"#
        );
        let files = [
            (
                "[Content_Types].xml",
                r#"<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="xml" ContentType="application/xml"/><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/><Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/></Types>"#,
            ),
            (
                "_rels/.rels",
                r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="xl/workbook.xml"/></Relationships>"#,
            ),
            (
                "xl/workbook.xml",
                r#"<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets><sheet name="Words" sheetId="1" r:id="rId1"/><sheet name="Ignored" sheetId="2" r:id="rId2"/></sheets></workbook>"#,
            ),
            (
                "xl/_rels/workbook.xml.rels",
                r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/><Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet2.xml"/></Relationships>"#,
            ),
            (
                "xl/sharedStrings.xml",
                r#"<sst xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" count="1" uniqueCount="1"><si><t>流血</t></si></sst>"#,
            ),
            ("xl/worksheets/sheet1.xml", sheet.as_str()),
            (
                "xl/worksheets/sheet2.xml",
                r#"<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheetData><row r="1"><c r="A1" t="inlineStr"><is><t>Invalid second sheet should not be imported</t></is></c></row></sheetData></worksheet>"#,
            ),
        ];
        for (name, xml) in files {
            zip.start_file(
                name,
                SimpleFileOptions::default().compression_method(CompressionMethod::Stored),
            )
            .unwrap();
            zip.write_all(xml.as_bytes()).unwrap();
        }
        zip.finish().unwrap().into_inner()
    }

    const VALID_ROW: &str = r#"<row r="3"><c r="A3" t="s"><v>0</v></c><c r="B3" t="inlineStr"><is><t>liúxiě</t></is></c><c r="C3" t="inlineStr"><is><t>v. to bleed</t></is></c><c r="D3" t="b"><v>1</v></c></row>"#;

    #[test]
    fn imports_unicode_shared_strings_header_and_excel_booleans() {
        let header = r#"<row r="1"><c r="A1" t="inlineStr"><is><t>word</t></is></c><c r="B1" t="inlineStr"><is><t>pinyin</t></is></c><c r="C1" t="inlineStr"><is><t>translation</t></is></c></row>"#;
        let cards = parse_flashcards_from_xlsx(&workbook(&format!("{header}{VALID_ROW}"))).unwrap();
        assert_eq!(cards.len(), 1);
        assert_eq!(cards[0].word, "流血");
        assert_eq!(cards[0].pinyin.as_deref(), Some("liúxiě"));
        assert!(cards[0].known);
    }

    #[test]
    fn accepts_blank_pinyin_and_optional_known() {
        let row = r#"<row r="7"><c r="A7" t="inlineStr"><is><t>你好</t></is></c><c r="C7" t="inlineStr"><is><t>Hello</t></is></c></row>"#;
        let cards = parse_flashcards_from_xlsx(&workbook(row)).unwrap();
        assert_eq!(cards[0].pinyin.as_deref(), Some(""));
        assert!(!cards[0].known);
        assert_eq!(cards[0].example_sentence, None);
    }

    #[test]
    fn imports_examples_in_column_d_and_known_in_column_e() {
        let row = r#"<row r="1"><c r="A1" t="inlineStr"><is><t>你好</t></is></c><c r="C1" t="inlineStr"><is><t>Hello</t></is></c><c r="D1" t="inlineStr"><is><t>你好，很高兴认识你！</t></is></c><c r="E1" t="b"><v>1</v></c></row>"#;
        let cards = parse_flashcards_from_xlsx(&workbook(row)).unwrap();
        assert_eq!(
            cards[0].example_sentence.as_deref(),
            Some("你好，很高兴认识你！")
        );
        assert!(cards[0].known);
        let optional_known = row.replace(r#"<c r="E1" t="b"><v>1</v></c>"#, "");
        let cards = parse_flashcards_from_xlsx(&workbook(&optional_known)).unwrap();
        assert!(cards[0].example_sentence.is_some());
        assert!(!cards[0].known);
    }

    #[test]
    fn example_header_disambiguates_boolean_text_and_rejects_garbled_examples() {
        let header = r#"<row r="1"><c r="A1" t="inlineStr"><is><t>word</t></is></c><c r="B1" t="inlineStr"><is><t>pinyin</t></is></c><c r="C1" t="inlineStr"><is><t>translation</t></is></c><c r="D1" t="inlineStr"><is><t>example_sentence</t></is></c></row>"#;
        let row = r#"<row r="2"><c r="A2" t="inlineStr"><is><t>真</t></is></c><c r="C2" t="inlineStr"><is><t>true</t></is></c><c r="D2" t="inlineStr"><is><t>true</t></is></c></row>"#;
        let cards = parse_flashcards_from_xlsx(&workbook(&format!("{header}{row}"))).unwrap();
        assert_eq!(cards[0].example_sentence.as_deref(), Some("true"));
        assert!(!cards[0].known);
        let garbled = row.replace("<t>true</t></is></c></row>", "<t>cafÃ©</t></is></c></row>");
        let error =
            parse_flashcards_from_xlsx(&workbook(&format!("{header}{garbled}"))).unwrap_err();
        assert!(error.contains("example sentence") && error.contains("garbled"));
    }

    #[test]
    fn rejects_damaged_files_empty_sheets_and_invalid_cells() {
        assert!(parse_flashcards_from_xlsx(b"not an excel workbook")
            .unwrap_err()
            .contains("Could not open"));
        assert!(parse_flashcards_from_xlsx(&workbook(""))
            .unwrap_err()
            .contains("No flashcards"));
        for (cell, expected) in [
            (
                r#"<c r="A4" t="inlineStr"><is><t>word</t></is></c>"#,
                "Row 4: word and translation",
            ),
            (r#"<c r="A4"><v>123</v></c>"#, "Row 4, column 1"),
            (r#"<c r="A4" t="e"><v>#VALUE!</v></c>"#, "Row 4, column 1"),
            (
                r#"<c r="A4" t="inlineStr"><is><t>cafÃ©</t></is></c><c r="C4" t="inlineStr"><is><t>coffee</t></is></c>"#,
                "Row 4, word",
            ),
            (
                r#"<c r="XFD4" t="inlineStr"><is><t>extra</t></is></c>"#,
                "Row 4: unexpected data",
            ),
        ] {
            let bytes = workbook(&format!("{VALID_ROW}<row r=\"4\">{cell}</row>"));
            let error = parse_flashcards_from_xlsx(&bytes).unwrap_err();
            assert!(error.contains(expected), "{error}");
        }
    }
}
