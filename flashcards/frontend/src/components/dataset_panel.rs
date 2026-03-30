use web_sys::{InputEvent, MouseEvent};
use yew::prelude::*;

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
    pub on_create_dataset: Callback<MouseEvent>,
}

#[function_component(DatasetPanel)]
pub fn dataset_panel(props: &DatasetPanelProps) -> Html {
    let dataset_list = if props.datasets.is_empty() {
        html! { <p style="color: #999;">{"No datasets yet. Create one below."}</p> }
    } else {
        html! {
            <div>
                { for props.datasets.iter().map(|dataset| {
                    let name = dataset.name.clone();
                    let name_for_delete = dataset.name.clone();
                    let is_selected = props.current_dataset == dataset.name;
                    let on_select_dataset = props.on_select_dataset.clone();
                    let on_delete_dataset = props.on_delete_dataset.clone();

                    html! {
                        <div key={dataset.name.clone()} style="display: inline-block; margin: 4px;">
                            <button
                                onclick={Callback::from(move |_| on_select_dataset.emit(name.clone()))}
                                style={format!(
                                    "padding: 8px 12px; border-radius: 4px 0 0 4px; border: 2px solid {}; background-color: {}; cursor: pointer; font-weight: {};",
                                    if is_selected { "#2196F3" } else { "#ccc" },
                                    if is_selected { "#e3f2fd" } else { "white" },
                                    if is_selected { "bold" } else { "normal" }
                                )}
                            >
                                { &dataset.name }
                            </button>
                            <button
                                onclick={Callback::from(move |_| on_delete_dataset.emit(name_for_delete.clone()))}
                                style="padding: 8px 8px; border-radius: 0 4px 4px 0; border: 2px solid #ccc; background-color: #ffebee; cursor: pointer; color: #d32f2f; font-weight: bold; margin-left: -2px;"
                                title="Delete this dataset"
                            >
                                { "Delete" }
                            </button>
                        </div>
                    }
                }) }
            </div>
        }
    };

    html! {
        <div style="margin-bottom: 20px; padding: 12px; background-color: #f5f5f5; border-radius: 8px;">
            <h3 style="margin-top: 0;">{"Datasets"}</h3>
            <div style="margin-bottom: 10px;">
                { dataset_list }
            </div>
            <div style="margin-bottom: 10px;">
                <button onclick={props.on_toggle_input.clone()}>
                    { if props.show_dataset_input { "Cancel" } else { "New Dataset" } }
                </button>
            </div>
            { if props.show_dataset_input {
                html! {
                    <div style="margin-top: 10px; text-align: center;">
                        <input
                            type="text"
                            placeholder="Dataset name (e.g., 'HSK 1', 'Business Terms')"
                            value={props.new_dataset_name.clone()}
                            oninput={props.on_dataset_name_input.clone()}
                            style="padding: 8px; width: 250px; margin-right: 8px; border-radius: 4px; border: 1px solid #ccc;"
                        />
                        <button onclick={props.on_create_dataset.clone()}>{"Create"}</button>
                    </div>
                }
            } else {
                html! {}
            } }
        </div>
    }
}
