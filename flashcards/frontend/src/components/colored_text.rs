use yew::prelude::*;

use crate::formatting::{CellFormatting, RgbColor};

pub fn colored_text(text: &str, formatting: &CellFormatting) -> Html {
    if !formatting.valid_runs(text) {
        return html! { {text.to_string()} };
    }
    html! {
        <>
            { for formatting.runs.iter().map(|run| {
                let style = run.color.map(|color: RgbColor| format!("--imported-text-color:{};", color.css())).unwrap_or_default();
                html! { <span class="colored-text" {style}>{run.text.clone()}</span> }
            }) }
        </>
    }
}
