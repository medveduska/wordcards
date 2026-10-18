use web_sys::{Event, InputEvent, MouseEvent, SubmitEvent};
use yew::prelude::*;

use crate::components::wordset_import::WordsetImport;
use crate::export_io::ExportFormat;
use crate::model::Dataset;

#[derive(Properties, PartialEq)]
pub struct DatasetPanelProps {
    pub datasets: Vec<Dataset>,
    pub current_dataset: String,
    pub show_dataset_input: bool,
    pub new_dataset_name: String,
    pub on_select_dataset: Callback<String>,
    pub on_delete_dataset: Callback<String>,
    pub on_toggle_input: Callback<MouseEvent>,
    pub on_dataset_name_input: Callback<InputEvent>,
    pub on_create_dataset: Callback<()>,
    pub on_add_card: Callback<MouseEvent>,
    pub on_save_all: Callback<MouseEvent>,
    pub on_open_backup_picker: Callback<MouseEvent>,
    pub on_backup_file_select: Callback<Event>,
    pub backup_status_message: Option<String>,
    pub backup_status_is_error: bool,
    pub show_import: bool,
    pub import_status_message: Option<String>,
    pub import_status_is_error: bool,
    pub show_export: bool,
    pub on_file_select: Callback<Event>,
    pub on_download: Callback<ExportFormat>,
    pub export_status_message: Option<String>,
    pub export_status_is_error: bool,
    pub renaming_dataset: Option<String>,
    pub rename_input: String,
    pub on_start_rename: Callback<String>,
    pub on_rename_input: Callback<InputEvent>,
    pub on_confirm_rename: Callback<MouseEvent>,
    pub on_cancel_rename: Callback<MouseEvent>,
}

fn status_message(message: &Option<String>, is_error: bool, class: &'static str) -> Html {
    message
        .as_ref()
        .map(|message| {
            html! {
                <p class={classes!(class, is_error.then_some("is-error"))}
                    role={if is_error { "alert" } else { "status" }}>{message}</p>
            }
        })
        .unwrap_or_default()
}

#[function_component(DatasetPanel)]
pub fn dataset_panel(props: &DatasetPanelProps) -> Html {
    let show_create = props.show_dataset_input || props.datasets.is_empty();
    let new_name = props.new_dataset_name.trim();
    let duplicate_name = props
        .datasets
        .iter()
        .any(|dataset| dataset.name == new_name);
    let can_create = !new_name.is_empty() && !duplicate_name;
    let on_create = {
        let on_create_dataset = props.on_create_dataset.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            if can_create {
                on_create_dataset.emit(());
            }
        })
    };
    let active = props
        .datasets
        .iter()
        .find(|dataset| dataset.name == props.current_dataset);

    html! {
        <section class="panel wordsets-panel" aria-labelledby="wordsets-title">
            <div class="wordsets-header">
                <div>
                    <h3 id="wordsets-title" class="panel-title">{"Wordsets"}</h3>
                    <p class="wordsets-subtitle">{"Choose a wordset to study, or start a new collection."}</p>
                </div>
                if !show_create {
                    <button class="btn btn-primary" onclick={props.on_toggle_input.clone()}>
                        <span aria-hidden="true">{"＋ "}</span>{"New wordset"}
                    </button>
                }
            </div>

            if show_create {
                <form class="wordset-create" onsubmit={on_create}>
                    <div class="wordset-next-step">
                        <span class="wordset-step-number" aria-hidden="true">{"1"}</span>
                        <div>
                            <h4>{if props.datasets.is_empty() { "Create your first wordset" } else { "Create a new wordset" }}</h4>
                            <p>{"Give it a name. Next, import a word list or add cards yourself."}</p>
                        </div>
                    </div>
                    <label class="input-label" for="new-wordset-name">{"Wordset name"}</label>
                    <div class="wordset-create-actions">
                        <input id="new-wordset-name" class="text-input" type="text"
                            placeholder="e.g. HSK 1 or This week’s words"
                            value={props.new_dataset_name.clone()}
                            oninput={props.on_dataset_name_input.clone()}
                            aria-describedby="wordset-name-feedback"
                            aria-invalid={duplicate_name.to_string()}
                            required={true} autofocus={true} />
                        <button type="submit" class="btn btn-primary" disabled={!can_create}>{"Create wordset →"}</button>
                        if !props.datasets.is_empty() {
                            <button type="button" class="btn wordset-text-button" onclick={props.on_toggle_input.clone()}>{"Cancel"}</button>
                        }
                    </div>
                    <p id="wordset-name-feedback" class={classes!("wordset-name-feedback", duplicate_name.then_some("is-error"))}>
                        {if duplicate_name { "This name is already taken. Try another." } else { "You can rename it later." }}
                    </p>
                </form>
            }

            if !props.datasets.is_empty() {
                <div class="wordsets-layout">
                    <nav class="wordset-library" aria-label="Choose a wordset">
                        <p class="wordset-section-label">{format!("Your wordsets · {}", props.datasets.len())}</p>
                        <div class="dataset-list">
                            { for props.datasets.iter().map(|dataset| {
                                let selected = props.current_dataset == dataset.name;
                                let name = dataset.name.clone();
                                let on_select = props.on_select_dataset.clone();
                                let count = dataset.flashcards.len() + dataset.known_cards.len();
                                html! {
                                    <button key={dataset.name.clone()}
                                        class={classes!("dataset-btn", selected.then_some("is-selected"))}
                                        aria-pressed={selected.to_string()}
                                        onclick={Callback::from(move |_| on_select.emit(name.clone()))}>
                                        <span class="wordset-selection-dot" aria-hidden="true">{if selected { "✓" } else { "" }}</span>
                                        <span class="wordset-list-label">
                                            <span class="wordset-list-name">{&dataset.name}</span>
                                            <span class="wordset-list-count">{format!("{count} {}", if count == 1 { "card" } else { "cards" })}</span>
                                        </span>
                                    </button>
                                }
                            }) }
                        </div>
                    </nav>
                    <div class="wordset-workspace">
                        if let Some(dataset) = active {
                            <div class="wordset-active-header">
                                <div class="wordset-active-heading">
                                    <p class="wordset-section-label">{"Current wordset"}</p>
                                    <h4>{&dataset.name}</h4>
                                    <p class="wordset-card-count">{format!("{} to study · {} known", dataset.flashcards.len(), dataset.known_cards.len())}</p>
                                </div>
                                <details key={format!("options-{}-{}", dataset.name, props.renaming_dataset.is_some())} class="wordset-menu">
                                    <summary class="wordset-menu-trigger">{"Options"}</summary>
                                    <div class="wordset-menu-content">
                                        <button class="btn wordset-text-button" onclick={{
                                            let name = dataset.name.clone();
                                            let on_rename = props.on_start_rename.clone();
                                            Callback::from(move |_| on_rename.emit(name.clone()))
                                        }}>{"Rename wordset"}</button>
                                        <button class="btn btn-danger" onclick={{
                                            let name = dataset.name.clone();
                                            let on_delete = props.on_delete_dataset.clone();
                                            Callback::from(move |_| on_delete.emit(name.clone()))
                                        }}>{"Delete wordset"}</button>
                                    </div>
                                </details>
                            </div>
                            if props.renaming_dataset.as_deref() == Some(dataset.name.as_str()) {
                                <div class="wordset-rename">
                                    <label class="input-label" for="rename-wordset-name">{"New name"}</label>
                                    <div class="wordset-create-actions">
                                        <input id="rename-wordset-name" type="text" class="text-input"
                                            value={props.rename_input.clone()} oninput={props.on_rename_input.clone()} autofocus={true} />
                                        <button class="btn btn-primary" onclick={props.on_confirm_rename.clone()}>{"Save name"}</button>
                                        <button class="btn wordset-text-button" onclick={props.on_cancel_rename.clone()}>{"Cancel"}</button>
                                    </div>
                                </div>
                            }

                            { status_message(&props.import_status_message, props.import_status_is_error, "import-status") }
                            if props.show_import {
                                <WordsetImport on_file_select={props.on_file_select.clone()} on_add_card={props.on_add_card.clone()} />
                            } else {
                                <div class="wordset-ready">
                                    <span class="wordset-ready-icon" aria-hidden="true">{"✓"}</span>
                                    <div>
                                        <h4>{if dataset.flashcards.is_empty() { "All caught up" } else { "Ready when you are" }}</h4>
                                        <p>{if dataset.flashcards.is_empty() {
                                            "You know every card in this wordset. Add more words, or revisit them in Word Review below."
                                        } else {
                                            "Study your cards below, or keep growing this wordset."
                                        }}</p>
                                    </div>
                                </div>
                            }
                            <div class="wordset-workspace-actions">
                                if !props.show_import {
                                    <button class="btn btn-primary" onclick={props.on_add_card.clone()}>{"＋ Add a card"}</button>
                                    <p class="wordset-import-note">{"Want to import another list? Create a new wordset."}</p>
                                }
                                if props.show_export {
                                    <details key={format!("export-{}", dataset.name)} class="wordset-menu wordset-export-menu">
                                        <summary class="wordset-menu-trigger">{"Export wordset"}</summary>
                                        <div class="wordset-menu-content">
                                            <button class="btn btn-secondary" onclick={props.on_download.reform(|_: MouseEvent| ExportFormat::Csv)}>{"Export CSV"}</button>
                                            <button class="btn btn-secondary" onclick={props.on_download.reform(|_: MouseEvent| ExportFormat::Xlsx)}>{"Export XLSX"}</button>
                                        </div>
                                    </details>
                                }
                            </div>
                            { status_message(&props.export_status_message, props.export_status_is_error, "export-status") }
                        } else {
                            <div class="wordset-select-prompt">
                                <span aria-hidden="true">{"↖"}</span>
                                <h4>{"Choose a wordset to get started"}</h4>
                                <p>{"Select one from your collection, or create a new one above."}</p>
                            </div>
                        }
                    </div>
                </div>
            }

            <div class="wordset-backup">
                <details>
                    <summary>{"Backup & restore"}<span>{"All wordsets and study progress"}</span></summary>
                    <div class="wordset-backup-content">
                        <p>{"Keep a copy of everything, or continue from a saved backup."}</p>
                        <div class="wordset-backup-actions">
                            <button class="btn btn-secondary" onclick={props.on_save_all.clone()}>{"Download full backup"}</button>
                            <button class="btn wordset-text-button" onclick={props.on_open_backup_picker.clone()}>{"Restore a backup"}</button>
                        </div>
                        <p class="wordset-import-note">{"Restoring replaces all wordsets and study progress in this browser."}</p>
                    </div>
                </details>
                <input id="load-progress-input" class="visually-hidden-input" type="file"
                    accept=".json,application/json" onchange={props.on_backup_file_select.clone()} />
                { status_message(&props.backup_status_message, props.backup_status_is_error, "backup-status") }
            </div>
        </section>
    }
}
