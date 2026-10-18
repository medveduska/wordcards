use js_sys::{Array, Uint8Array};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Blob, BlobPropertyBag, Url};

use crate::{csv_io::export_flashcards_csv, model::Flashcard, xlsx_export::export_flashcards_xlsx};

#[derive(Clone, Copy, PartialEq)]
pub enum ExportFormat {
    Csv,
    Xlsx,
}

impl ExportFormat {
    pub fn label(self) -> &'static str {
        match self {
            Self::Csv => "CSV",
            Self::Xlsx => "XLSX",
        }
    }

    pub fn export<'a>(self, cards: impl Iterator<Item = &'a Flashcard>) -> Result<Vec<u8>, String> {
        match self {
            Self::Csv => export_flashcards_csv(cards)
                .map_err(|error| format!("Could not create the CSV file: {error}")),
            Self::Xlsx => export_flashcards_xlsx(cards)
                .map_err(|error| format!("Could not create the XLSX file: {error}")),
        }
    }

    pub fn start_download(self, bytes: &[u8]) -> Result<(), JsValue> {
        let (file_name, content_type) = match self {
            Self::Csv => ("updated_flashcards.csv", "text/csv;charset=utf-8"),
            Self::Xlsx => (
                "updated_flashcards.xlsx",
                "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            ),
        };
        let array = Uint8Array::from(bytes);
        let blob_parts = Array::new();
        blob_parts.push(&array.buffer());
        let options = BlobPropertyBag::new();
        options.set_type(content_type);
        let blob = Blob::new_with_u8_array_sequence_and_options(&blob_parts, &options)?;

        let window = web_sys::window().ok_or_else(|| JsValue::from_str("window unavailable"))?;
        let document = window
            .document()
            .ok_or_else(|| JsValue::from_str("document unavailable"))?;
        let anchor: web_sys::HtmlElement = document.create_element("a")?.dyn_into()?;
        anchor.set_attribute("download", file_name)?;
        let url = Url::create_object_url_with_blob(&blob)?;
        let result = anchor.set_attribute("href", &url).map(|_| anchor.click());
        Url::revoke_object_url(&url)?;
        result
    }
}
