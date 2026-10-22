use rust_xlsxwriter::{Color, Format, FormatAlign, Workbook, Worksheet, XlsxError};

use crate::formatting::CellFormatting;
use crate::model::Flashcard;
use crate::xlsx_text::escape_rich_text_literals;

pub fn export_flashcards_xlsx<'a>(
    cards: impl Iterator<Item = &'a Flashcard>,
) -> Result<Vec<u8>, XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.set_name("Flashcards")?;
    worksheet.set_freeze_panes(1, 0)?;
    for (column, width) in [18.0, 24.0, 40.0, 60.0, 12.0].into_iter().enumerate() {
        worksheet.set_column_width(column as u16, width)?;
    }
    let header_format = Format::new().set_bold();
    for (column, header) in ["word", "pinyin", "translation", "example_sentence", "known"]
        .into_iter()
        .enumerate()
    {
        worksheet.write_string_with_format(0, column as u16, header, &header_format)?;
    }
    let text_format = Format::new().set_text_wrap().set_align(FormatAlign::Top);
    let mut last_row = 0;
    for (index, card) in cards.enumerate() {
        let row = u32::try_from(index + 1).map_err(|_| XlsxError::RowColumnLimitError)?;
        // Explicit strings preserve Unicode, leading zeros, and formula-like text.
        for (column, (value, formatting)) in [
            (card.word.as_str(), &card.formatting.word),
            (
                card.pinyin.as_deref().unwrap_or_default(),
                &card.formatting.pinyin,
            ),
            (card.translation.as_str(), &card.formatting.translation),
            (
                card.example_sentence.as_deref().unwrap_or_default(),
                &card.formatting.example_sentence,
            ),
        ]
        .into_iter()
        .enumerate()
        {
            write_colored_text(
                worksheet,
                row,
                column as u16,
                value,
                formatting,
                &text_format,
            )?;
        }
        worksheet.write_boolean(row, 4, card.known)?;
        last_row = row;
    }
    worksheet.autofilter(0, 0, last_row, 4)?;
    workbook.save_to_buffer()
}

fn write_colored_text(
    worksheet: &mut Worksheet,
    row: u32,
    column: u16,
    text: &str,
    formatting: &CellFormatting,
    base_format: &Format,
) -> Result<(), XlsxError> {
    let mut format = base_format.clone();
    if let Some(color) = formatting.text_color {
        format = format.set_font_color(Color::RGB(color.into()));
    }
    if let Some(color) = formatting.background_color {
        format = format.set_background_color(Color::RGB(color.into()));
    }
    if formatting.valid_runs(text) {
        let formats: Vec<_> = formatting
            .runs
            .iter()
            .map(|run| match run.color.or(formatting.text_color) {
                Some(color) => Format::new().set_font_color(Color::RGB(color.into())),
                None => Format::new().set_font_color(Color::Automatic),
            })
            .collect();
        let strings: Vec<_> = formatting
            .runs
            .iter()
            .map(|run| escape_rich_text_literals(&run.text))
            .collect();
        let runs: Vec<_> = formats
            .iter()
            .zip(&strings)
            .map(|(format, text)| (format, text.as_str()))
            .collect();
        worksheet.write_rich_string_with_format(row, column, &runs, &format)?;
    } else {
        worksheet.write_string_with_format(row, column, text, &format)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{export_io::ExportFormat, xlsx_io::parse_flashcards_from_xlsx};
    use calamine::{Data, Reader, Xlsx};
    use std::io::Cursor;

    fn sample_cards() -> Vec<Flashcard> {
        vec![
            Flashcard {
                word: "你好".into(),
                pinyin: Some("nǐ hǎo".into()),
                translation: "Hello, friend & <welcome>".into(),
                example_sentence: Some("你好！\n\"很高兴认识你。\"".into()),
                formatting: Default::default(),
                known: false,
            },
            Flashcard {
                word: "00123".into(),
                pinyin: Some("".into()),
                translation: "=1+1".into(),
                example_sentence: Some("true".into()),
                formatting: Default::default(),
                known: true,
            },
        ]
    }

    #[test]
    fn xlsx_export_round_trips_all_fields_and_known_states() {
        let cards = sample_cards();
        let bytes = ExportFormat::Xlsx.export(cards.iter()).unwrap();
        assert_eq!(parse_flashcards_from_xlsx(&bytes).unwrap(), cards);
        let mut workbook = Xlsx::new(Cursor::new(bytes)).unwrap();
        let sheet = workbook.worksheet_range("Flashcards").unwrap();
        assert_eq!(
            sheet.get_value((0, 3)),
            Some(&Data::String("example_sentence".into()))
        );
        assert_eq!(sheet.get_value((2, 0)), Some(&Data::String("00123".into())));
        assert_eq!(sheet.get_value((2, 2)), Some(&Data::String("=1+1".into())));
        assert_eq!(sheet.get_value((2, 4)), Some(&Data::Bool(true)));
        let formulas = workbook.worksheet_formula("Flashcards").unwrap();
        assert!(formulas
            .used_cells()
            .all(|(_, _, formula)| formula.is_empty()));
    }

    #[test]
    fn exports_empty_wordsets_with_headers_and_handles_absent_optional_fields() {
        let bytes = export_flashcards_xlsx(std::iter::empty()).unwrap();
        let mut workbook = Xlsx::new(Cursor::new(bytes)).unwrap();
        let sheet = workbook.worksheet_range("Flashcards").unwrap();
        assert_eq!(sheet.get_size(), (1, 5));
        let mut card = sample_cards().remove(0);
        card.pinyin = None;
        card.example_sentence = None;
        let bytes = export_flashcards_xlsx(std::iter::once(&card)).unwrap();
        let restored = parse_flashcards_from_xlsx(&bytes).unwrap();
        assert_eq!(restored[0].word, card.word);
        assert_eq!(restored[0].pinyin.as_deref(), Some(""));
        assert_eq!(restored[0].example_sentence, None);
    }

    #[test]
    fn reports_excel_cell_length_errors_instead_of_truncating() {
        let mut card = sample_cards().remove(0);
        card.example_sentence = Some("a".repeat(32_768));
        assert!(ExportFormat::Xlsx
            .export(std::iter::once(&card))
            .unwrap_err()
            .contains("Could not create the XLSX file"));
    }
}
