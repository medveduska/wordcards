use web_sys::{Event, HtmlInputElement, MouseEvent};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct WordsetImportProps {
    pub on_file_select: Callback<Event>,
    pub on_add_card: Callback<MouseEvent>,
}

#[function_component(WordsetImport)]
pub fn wordset_import(props: &WordsetImportProps) -> Html {
    let file_input = use_node_ref();
    let choose_file = {
        let file_input = file_input.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(input) = file_input.cast::<HtmlInputElement>() {
                input.click();
            }
        })
    };

    html! {
        <div class="wordset-onboarding">
            <div class="wordset-next-step">
                <span class="wordset-step-number" aria-hidden="true">{"2"}</span>
                <div>
                    <h4>{"Add your first words"}</h4>
                    <p>{"Choose how you’d like to fill this wordset."}</p>
                </div>
            </div>
            <div class="wordset-start-options">
                <div class="wordset-start-option is-recommended">
                    <span class="wordset-option-icon" aria-hidden="true">{"↥"}</span>
                    <h5>{"Import a file"}</h5>
                    <p>{"Already have a word list? Bring it in from CSV or Excel."}</p>
                    <button class="btn btn-primary" onclick={choose_file}>
                        {"Choose CSV or XLSX"}
                    </button>
                    <input
                        ref={file_input}
                        id="import-flashcards-input"
                        class="visually-hidden-input"
                        type="file"
                        tabindex="-1"
                        aria-label="Import a CSV or XLSX word list"
                        accept=".csv,.xlsx"
                        onchange={props.on_file_select.clone()}
                    />
                </div>
                <div class="wordset-start-option">
                    <span class="wordset-option-icon" aria-hidden="true">{"＋"}</span>
                    <h5>{"Add a card"}</h5>
                    <p>{"Start with one word. Add more whenever you’re ready."}</p>
                    <button class="btn btn-secondary" onclick={props.on_add_card.clone()}>
                        {"Add your first card"}
                    </button>
                </div>
            </div>
            <details class="wordset-file-guide">
                <summary>{"How should I arrange my file?"}</summary>
                <p>{"Put one word on each row, in this column order:"}</p>
                <ol class="wordset-column-guide">
                    <li><strong>{"Word"}</strong><span>{"Required"}</span></li>
                    <li><strong>{"Pinyin"}</strong><span>{"May be blank"}</span></li>
                    <li><strong>{"Translation"}</strong><span>{"Required"}</span></li>
                    <li><strong>{"Example sentence"}</strong><span>{"Optional"}</span></li>
                    <li><strong>{"Known"}</strong><span>{"Optional: true / false"}</span></li>
                </ol>
                <p>{"CSV files must use UTF-8. For Excel, we read the first worksheet. A header row is optional; older files with known in column 4 also work."}</p>
            </details>
        </div>
    }
}
