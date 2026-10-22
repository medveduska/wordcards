use serde::{Deserialize, Serialize};

/// Store RGB values rather than CSS so imported and restored colors are safe to render.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "u32", into = "u32")]
pub struct RgbColor(u32);

impl RgbColor {
    pub fn from_hex(value: &str) -> Option<Self> {
        let value = value.strip_prefix('#').unwrap_or(value);
        if !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return None;
        }
        let rgb = match value.len() {
            6 => value,
            8 => &value[2..], // Excel stores ARGB; cell colors are opaque.
            _ => return None,
        };
        u32::from_str_radix(rgb, 16).ok()?.try_into().ok()
    }

    pub fn css(self) -> String {
        format!("#{:06X}", self.0)
    }
}

impl TryFrom<u32> for RgbColor {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        (value <= 0xFFFFFF)
            .then_some(Self(value))
            .ok_or("Invalid RGB color")
    }
}

impl From<RgbColor> for u32 {
    fn from(value: RgbColor) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TextRun {
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<RgbColor>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CellFormatting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_color: Option<RgbColor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background_color: Option<RgbColor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub runs: Vec<TextRun>,
}

impl CellFormatting {
    pub fn is_empty(&self) -> bool {
        self.text_color.is_none() && self.background_color.is_none() && self.runs.is_empty()
    }

    pub fn valid_runs(&self, text: &str) -> bool {
        !self.runs.is_empty()
            && self.runs.iter().all(|run| !run.text.is_empty())
            && self
                .runs
                .iter()
                .map(|run| run.text.as_str())
                .collect::<String>()
                == text
    }

    pub fn css_variables(&self) -> String {
        let mut style = String::new();
        if let Some(color) = self.text_color {
            style.push_str(&format!("--imported-text-color:{};", color.css()));
        }
        if let Some(color) = self.background_color {
            style.push_str(&format!("--imported-background:{};", color.css()));
        }
        style
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CardFormatting {
    #[serde(default, skip_serializing_if = "CellFormatting::is_empty")]
    pub word: CellFormatting,
    #[serde(default, skip_serializing_if = "CellFormatting::is_empty")]
    pub pinyin: CellFormatting,
    #[serde(default, skip_serializing_if = "CellFormatting::is_empty")]
    pub translation: CellFormatting,
    #[serde(default, skip_serializing_if = "CellFormatting::is_empty")]
    pub example_sentence: CellFormatting,
}

impl CardFormatting {
    pub fn is_empty(&self) -> bool {
        [
            &self.word,
            &self.pinyin,
            &self.translation,
            &self.example_sentence,
        ]
        .iter()
        .all(|cell| cell.is_empty())
    }

    pub fn cell_mut(&mut self, column: u16) -> Option<&mut CellFormatting> {
        match column {
            0 => Some(&mut self.word),
            1 => Some(&mut self.pinyin),
            2 => Some(&mut self.translation),
            3 => Some(&mut self.example_sentence),
            _ => None,
        }
    }
}
