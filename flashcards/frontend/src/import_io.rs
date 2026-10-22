use crate::{
    csv_io::parse_flashcards_from_csv, model::Flashcard, xlsx_io::parse_flashcards_from_xlsx,
};

#[derive(Clone, Copy)]
pub enum ImportFormat {
    Csv,
    Xlsx,
}

impl ImportFormat {
    pub fn from_filename(name: &str) -> Result<Self, String> {
        match name
            .rsplit_once('.')
            .map(|(_, extension)| extension)
            .unwrap_or_default()
            .to_ascii_lowercase()
            .as_str()
        {
            "csv" => Ok(Self::Csv),
            "xlsx" => Ok(Self::Xlsx),
            _ => Err(
                "Unsupported file type. Choose a UTF-8 CSV (.csv) or Excel (.xlsx) file.".into(),
            ),
        }
    }

    pub fn parse(self, bytes: &[u8]) -> Result<Vec<Flashcard>, String> {
        match self {
            Self::Csv => {
                let text = std::str::from_utf8(bytes).map_err(|_| {
                    "The CSV file is not valid UTF-8. Save it as CSV UTF-8 or use an XLSX file."
                        .to_string()
                })?;
                parse_flashcards_from_csv(text)
            }
            Self::Xlsx => parse_flashcards_from_xlsx(bytes),
        }
    }
}

fn has_invalid_text(value: &str) -> bool {
    let invalid_symbol = value.chars().any(|ch| {
        let code = ch as u32;
        (ch.is_control() && !matches!(ch, '\t' | '\n' | '\r'))
            || matches!(code, 0xFFFD | 0xFEFF | 0xE000..=0xF8FF | 0xF0000..=0xFFFFD | 0x100000..=0x10FFFD | 0xFDD0..=0xFDEF)
            || code & 0xFFFF >= 0xFFFE
    });
    // Common UTF-8 bytes mistakenly decoded as Latin-1/Windows-1252.
    // Do not reject accented letters, combining tone marks, or other scripts.
    let mojibake = value.chars().zip(value.chars().skip(1)).any(|(a, b)| {
        matches!(a, 'Ã' | 'Â' | 'Ä' | 'Å')
            && (matches!(b, '\u{80}'..='\u{BF}') || "€‚ƒ„…†‡ˆ‰Š‹ŒŽ‘’“”•–—˜™š›œžŸ".contains(b))
    }) || ["â€", "ï¿½", "ðŸ"]
        .iter()
        .any(|pattern| value.contains(pattern));
    invalid_symbol || mojibake
}

#[derive(Clone, Copy, Default)]
pub enum ImportLayout {
    #[default]
    Auto,
    Examples,
    LegacyKnown,
}

pub fn header_layout(fields: &[&str]) -> Option<ImportLayout> {
    if !(3..=5).contains(&fields.len())
        || !fields
            .iter()
            .take(3)
            .zip(["word", "pinyin", "translation"])
            .all(|(field, expected)| field.trim().eq_ignore_ascii_case(expected))
    {
        return None;
    }
    let fourth = fields
        .get(3)
        .copied()
        .unwrap_or_default()
        .trim()
        .to_ascii_lowercase();
    let fifth = fields.get(4).copied().unwrap_or_default().trim();
    match fourth.as_str() {
        "" if fifth.is_empty() => Some(ImportLayout::Auto),
        "known" if fifth.is_empty() => Some(ImportLayout::LegacyKnown),
        "example_sentence" | "example sentence" | "example"
            if fifth.is_empty() || fifth.eq_ignore_ascii_case("known") =>
        {
            Some(ImportLayout::Examples)
        }
        _ => None,
    }
}

pub fn validate_row(
    fields: &[&str],
    row: usize,
    layout: ImportLayout,
) -> Result<Flashcard, String> {
    if !(3..=5).contains(&fields.len()) {
        return Err(format!("Row {row}: expected 3 to 5 columns: word, pinyin, translation, optional example sentence, and optional known. Leave pinyin blank if unused."));
    }
    let fourth = fields.get(3).copied().unwrap_or_default();
    // Headerless exports from older versions used column 4 for the known flag.
    // An example_sentence header disambiguates literal sentences like "true".
    let legacy_known = matches!(layout, ImportLayout::LegacyKnown)
        || (matches!(layout, ImportLayout::Auto)
            && fields.len() == 4
            && (fourth.trim().eq_ignore_ascii_case("true")
                || fourth.trim().eq_ignore_ascii_case("false")));
    let column_names = if legacy_known {
        [
            "word",
            "pinyin",
            "translation",
            "known",
            "unexpected column",
        ]
    } else {
        ["word", "pinyin", "translation", "example sentence", "known"]
    };
    for (field, name) in fields.iter().zip(column_names) {
        if has_invalid_text(field) {
            return Err(format!("Row {row}, {name}: text contains invalid or garbled characters. Correct the text and save as UTF-8 CSV or XLSX."));
        }
    }
    if fields[0].trim().is_empty() || fields[2].trim().is_empty() {
        return Err(format!(
            "Row {row}: word and translation are required. Pinyin may be blank."
        ));
    }
    if legacy_known && fields.get(4).is_some_and(|value| !value.trim().is_empty()) {
        return Err(format!("Row {row}: unexpected fifth column after a legacy known column. Use word,pinyin,translation,example_sentence,known."));
    }
    let known = if legacy_known {
        fourth
    } else {
        fields.get(4).copied().unwrap_or_default()
    }
    .trim();
    if !known.is_empty()
        && !known.eq_ignore_ascii_case("true")
        && !known.eq_ignore_ascii_case("false")
    {
        return Err(format!(
            "Row {row}, known: use true, false, or leave the cell blank."
        ));
    }
    Ok(Flashcard {
        word: fields[0].to_string(),
        pinyin: Some(fields[1].to_string()),
        translation: fields[2].to_string(),
        example_sentence: (!legacy_known && !fourth.trim().is_empty()).then(|| fourth.to_string()),
        known: known.eq_ignore_ascii_case("true"),
        formatting: Default::default(),
    })
}

pub fn require_cards(cards: Vec<Flashcard>) -> Result<Vec<Flashcard>, String> {
    if cards.is_empty() {
        Err(
            "No flashcards found. Add rows with word, pinyin (may be blank), and translation."
                .into(),
        )
    } else {
        Ok(cards)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_unsupported_files_and_invalid_utf8() {
        for name in [
            "words.xls",
            "words.txt",
            "words.json",
            "words.csv.exe",
            "words",
            "csv",
            "xlsx",
        ] {
            assert!(ImportFormat::from_filename(name).is_err(), "{name}");
        }
        assert!(ImportFormat::from_filename("WORDS.CSV").is_ok());
        assert!(ImportFormat::from_filename("WORDS.XLSX").is_ok());
        assert!(ImportFormat::Csv
            .parse(b"\xff,abc,translation")
            .unwrap_err()
            .contains("UTF-8"));
    }

    #[test]
    fn rejects_damaged_text_without_restricting_languages() {
        for value in [
            "ęµ\u{81}č\u{F8FC}€",
            "liĆŗxiÄ›",
            "cafÃ©",
            "bad\u{FFFD}",
            "bad\0",
            "bad\u{FFFF}",
        ] {
            assert!(
                validate_row(&[value, "", "meaning"], 2, ImportLayout::Auto)
                    .unwrap_err()
                    .contains("Row 2"),
                "{value}"
            );
        }
        for value in [
            "流血",
            "cā",
            "ca\u{304}",
            "你好！",
            "Åland",
            "Äpfel",
            "čeļš",
            "Привіт",
            "مرحبا",
            "👋",
        ] {
            assert!(
                validate_row(&[value, "nǐ hǎo", "meaning"], 1, ImportLayout::Auto).is_ok(),
                "{value}"
            );
        }
    }
}
