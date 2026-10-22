//! Keep rich-text fragments consistent with Calamine's imported cell text.
//! XML line endings are normalized by the XML reader before calling this helper.

pub fn decode_text(value: &str, preserve_space: bool) -> String {
    let value = if preserve_space {
        value
    } else {
        value.trim_matches([' ', '\t', '\r', '\n'])
    };
    let mut decoded = String::with_capacity(value.len());
    let mut remaining = value;
    while !remaining.is_empty() {
        let bytes = remaining.as_bytes();
        // Match the _x00HH_ decoding performed by the current Calamine reader.
        // Decode once: _x005F_x000D_ is literal "_x000D_", not a line break.
        if bytes.len() >= 7
            && bytes.starts_with(b"_x00")
            && bytes[6] == b'_'
            && bytes[4..6].iter().all(u8::is_ascii_hexdigit)
        {
            let code = u8::from_str_radix(&remaining[4..6], 16).expect("Validated hex digits");
            decoded.push(char::from(code));
            remaining = &remaining[7..];
        } else {
            let ch = remaining.chars().next().expect("Nonempty text");
            decoded.push(ch);
            remaining = &remaining[ch.len_utf8()..];
        }
    }
    decoded
}

/// The XLSX writer escapes literal _xHHHH_ sequences for plain strings, but
/// rich strings need this protection before their XML is assembled.
pub fn escape_rich_text_literals(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for (index, ch) in value.char_indices() {
        let remaining = &value.as_bytes()[index..];
        if remaining.len() >= 7
            && remaining.starts_with(b"_x")
            && remaining[6] == b'_'
            && remaining[2..6].iter().all(u8::is_ascii_hexdigit)
        {
            escaped.push_str("_x005F");
        }
        escaped.push(ch);
    }
    escaped
}
