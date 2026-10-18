use csv::{IntoInnerError, Writer};

use crate::import_io::{header_layout, require_cards, validate_row, ImportLayout};
use crate::model::Flashcard;

pub fn parse_flashcards_from_csv(csv_data: &str) -> Result<Vec<Flashcard>, String> {
    let trimmed_data = csv_data.trim_start_matches('\u{FEFF}');
    validate_csv_quotes(trimmed_data)?;
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(trimmed_data.as_bytes());

    let mut cards = Vec::new();
    let mut first_row = true;
    let mut layout = ImportLayout::Auto;
    for record in reader.records() {
        let record = record.map_err(|error| format!("Could not read CSV data: {error}"))?;
        let row = record
            .position()
            .map(|pos| pos.line() as usize)
            .unwrap_or(1);
        let fields: Vec<_> = record.iter().collect();
        if fields.iter().all(|field| field.trim().is_empty()) {
            continue;
        }
        if first_row {
            if let Some(header) = header_layout(&fields) {
                layout = header;
                first_row = false;
                continue;
            }
        }
        first_row = false;
        cards.push(validate_row(&fields, row, layout)?);
    }
    require_cards(cards)
}

// The csv crate intentionally tolerates broken quotes; reject them for imports.
fn validate_csv_quotes(text: &str) -> Result<(), String> {
    #[derive(Clone, Copy, PartialEq)]
    enum State {
        Start,
        Text,
        Quoted,
        Closed,
    }
    let mut state = State::Start;
    let mut line = 1;
    for ch in text.chars() {
        state = match (state, ch) {
            (State::Start, '"') => State::Quoted,
            (State::Quoted, '"') => State::Closed,
            (State::Closed, '"') => State::Quoted,
            (State::Quoted, _) => State::Quoted,
            (_, ',' | '\r' | '\n') => State::Start,
            (State::Text, '"') | (State::Closed, _) => {
                return Err(format!("Line {line}: malformed CSV quotes. Enclose fields containing commas or newlines in double quotes and escape quotes as two double quotes."));
            }
            _ => State::Text,
        };
        if ch == '\n' {
            line += 1;
        }
    }
    if state == State::Quoted {
        return Err(format!("Line {line}: CSV has an unclosed quoted field."));
    }
    Ok(())
}

pub fn export_flashcards_csv<'a>(
    cards: impl Iterator<Item = &'a Flashcard>,
) -> Result<Vec<u8>, IntoInnerError<Writer<Vec<u8>>>> {
    let mut writer = csv::Writer::from_writer(Vec::new());
    // Explicit headers keep example text such as "true" distinct from legacy flags.
    let _ = writer.write_record(["word", "pinyin", "translation", "example_sentence", "known"]);

    for card in cards {
        let record = [
            card.word.as_str(),
            card.pinyin.as_deref().unwrap_or_default(),
            card.translation.as_str(),
            card.example_sentence.as_deref().unwrap_or_default(),
            if card.known { "true" } else { "false" },
        ];
        let _ = writer.write_record(record);
    }

    let mut csv_bytes = writer.into_inner()?;
    let mut result = Vec::with_capacity(3 + csv_bytes.len());
    result.extend_from_slice(b"\xef\xbb\xbf");
    result.append(&mut csv_bytes);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::{export_flashcards_csv, parse_flashcards_from_csv};

    #[test]
    fn parses_known_column_from_csv() {
        let cards = parse_flashcards_from_csv("阿姨,āyí,aunt,true\n啊,a,ah,false\n").unwrap();

        assert_eq!(cards.len(), 2);
        assert!(cards[0].known);
        assert!(!cards[1].known);
        assert_eq!(cards[0].translation, "aunt");
    }

    #[test]
    fn exports_known_state_to_csv() {
        let cards = parse_flashcards_from_csv("阿姨,āyí,aunt,true\n").unwrap();
        let bytes = export_flashcards_csv(cards.iter()).expect("csv export should succeed");
        let csv = String::from_utf8(bytes).expect("csv should be utf-8");

        assert!(csv.contains("阿姨,āyí,aunt,,true"));
        assert_eq!(parse_flashcards_from_csv(&csv).unwrap(), cards);
    }

    #[test]
    fn accepts_bom_optional_header_optional_known_and_quoted_fields() {
        let cards = parse_flashcards_from_csv("\u{FEFF}word,pinyin,translation,known\r\n\"你好\",nǐ hǎo,\"Hello, friend\"\r\n再见,,\"Say \"\"bye\"\"\nnow\",TRUE\r\n").unwrap();
        assert_eq!(cards.len(), 2);
        assert_eq!(cards[0].translation, "Hello, friend");
        assert_eq!(cards[1].translation, "Say \"bye\"\nnow");
        assert!(cards[1].known);
    }

    #[test]
    fn rejects_invalid_rows_instead_of_importing_partially() {
        for invalid in [
            "word,pinyin",
            ",,meaning",
            "word,pinyin,",
            "a,b,c,true,extra",
            "a,b,c,example,yes",
            "\"a,b,c",
            "a\"b,c,d",
            "\"a\"oops,b,c",
        ] {
            assert!(
                parse_flashcards_from_csv(&format!("你好,nǐ hǎo,hello\n{invalid}")).is_err(),
                "{invalid}"
            );
        }
        for empty in ["", "\u{FEFF}\n", ",,\n", "word,pinyin,translation\n"] {
            assert!(parse_flashcards_from_csv(empty)
                .unwrap_err()
                .contains("No flashcards"));
        }
    }

    #[test]
    fn preserves_pinyin_tone_marks() {
        let cards = parse_flashcards_from_csv("流血,liúxiě,v. to bleed\n擦,ca\u{304},v. to wipe\n")
            .unwrap();
        assert_eq!(cards[0].pinyin.as_deref(), Some("liúxiě"));
        assert_eq!(cards[1].pinyin.as_deref(), Some("ca\u{304}"));
    }

    #[test]
    fn imports_optional_examples_and_preserves_them_on_export() {
        let cards = parse_flashcards_from_csv("你好,nǐ hǎo,Hello,\"你好！Hello, friend.\",true\n再见,,Goodbye\n谢谢,xièxie,Thanks,谢谢你的帮助！\n").unwrap();
        assert_eq!(
            cards[0].example_sentence.as_deref(),
            Some("你好！Hello, friend.")
        );
        assert!(cards[0].known);
        assert_eq!(cards[1].example_sentence, None);
        assert_eq!(cards[2].example_sentence.as_deref(), Some("谢谢你的帮助！"));
        let bytes = export_flashcards_csv(cards.iter()).unwrap();
        assert_eq!(
            parse_flashcards_from_csv(std::str::from_utf8(&bytes).unwrap()).unwrap(),
            cards
        );
    }

    #[test]
    fn distinguishes_explicit_example_headers_from_legacy_known_flags() {
        let cards = parse_flashcards_from_csv(
            "word,pinyin,translation,example_sentence,known\n真,zhēn,true,true,false\n",
        )
        .unwrap();
        assert_eq!(cards[0].example_sentence.as_deref(), Some("true"));
        assert!(!cards[0].known);
        let legacy =
            parse_flashcards_from_csv("word,pinyin,translation,known\n真,zhēn,true,TRUE\n")
                .unwrap();
        assert!(legacy[0].known);
        assert_eq!(legacy[0].example_sentence, None);
        assert!(
            parse_flashcards_from_csv("word,pinyin,translation,known\n真,zhēn,true,yes\n").is_err()
        );
    }

    #[test]
    fn validates_example_encoding_and_fifth_column() {
        for invalid in [
            "a,b,c,cafÃ©",
            "a,b,c,bad\u{FFFD}",
            "a,b,c,example,yes",
            "a,b,c,example,false,extra",
        ] {
            assert!(parse_flashcards_from_csv(invalid).is_err(), "{invalid}");
        }
    }
}
