use web_sys::MouseEvent;
use yew::prelude::*;

use crate::model::StudyDirection;

#[derive(Properties, PartialEq)]
pub struct StudyToolbarProps {
    pub direction: StudyDirection,
    pub on_toggle_direction: Callback<MouseEvent>,
    pub on_randomize: Callback<MouseEvent>,
    pub on_open_add: Callback<MouseEvent>,
}

#[function_component(StudyToolbar)]
pub fn study_toolbar(props: &StudyToolbarProps) -> Html {
    html! {
        <div style="margin-top: 15px;">
            <button onclick={props.on_toggle_direction.clone()}>
                {
                    match props.direction {
                        StudyDirection::Normal => "Switch to Translation -> Pinyin -> Character",
                        StudyDirection::Reverse => "Switch to Character -> Pinyin -> Translation",
                    }
                }
            </button>
            <button onclick={props.on_randomize.clone()} style="margin-left: 10px;">
                {"Randomize"}
            </button>
            <button onclick={props.on_open_add.clone()} style="margin-left: 10px;">{"Add New Flashcard"}</button>
        </div>
    }
}
