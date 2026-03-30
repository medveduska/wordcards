use web_sys::{Event, MouseEvent};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FileManagementPanelProps {
    pub on_file_select: Callback<Event>,
    pub on_download: Callback<MouseEvent>,
}

#[function_component(FileManagementPanel)]
pub fn file_management_panel(props: &FileManagementPanelProps) -> Html {
    html! {
        <section class="panel">
            <h3 class="panel-title">{"Export / Import"}</h3>
            <div class="panel-content">
                <div class="import-group">
                    <label class="input-label" for="import-flashcards-input">{"Import Flashcards"}</label>
                    <input
                        id="import-flashcards-input"
                        class="file-input"
                        type="file"
                        accept=".csv"
                        onchange={props.on_file_select.clone()}
                    />
                </div>
            </div>
            <button class="btn btn-primary" onclick={props.on_download.clone()}>
                {"Export Flashcards"}
            </button>
        </section>
    }
}
