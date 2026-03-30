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
        <div style="margin: 20px 0; padding: 12px; background-color: #f5f5f5; border-radius: 8px;">
            <h3 style="margin-top: 0;">{"File Management"}</h3>
            <div style="margin-bottom: 12px;">
                <input type="file" accept=".csv" onchange={props.on_file_select.clone()} />
            </div>
            <button onclick={props.on_download.clone()}>
                {"Download Flashcards"}
            </button>
        </div>
    }
}
