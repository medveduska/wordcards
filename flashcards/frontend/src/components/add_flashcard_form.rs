use web_sys::{InputEvent, MouseEvent};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AddFlashcardFormProps {
    pub visible: bool,
    pub new_word: String,
    pub new_pinyin: String,
    pub new_translation: String,
    pub on_word_input: Callback<InputEvent>,
    pub on_pinyin_input: Callback<InputEvent>,
    pub on_translation_input: Callback<InputEvent>,
    pub on_save: Callback<MouseEvent>,
    pub on_cancel: Callback<MouseEvent>,
}

#[function_component(AddFlashcardForm)]
pub fn add_flashcard_form(props: &AddFlashcardFormProps) -> Html {
    if !props.visible {
        return html! {};
    }

    html! {
        <div style="margin-top:20px; padding:12px; border:1px solid #ddd; display:inline-block; text-align:left; border-radius:8px;">
            <h3 style="margin:0 0 8px 0;">{"Add New Flashcard"}</h3>
            <div style="margin-bottom:8px;">
                <input placeholder="Chinese character" value={props.new_word.clone()} oninput={props.on_word_input.clone()} />
            </div>
            <div style="margin-bottom:8px;">
                <input placeholder="Pinyin" value={props.new_pinyin.clone()} oninput={props.on_pinyin_input.clone()} />
            </div>
            <div style="margin-bottom:8px;">
                <input placeholder="Translation" value={props.new_translation.clone()} oninput={props.on_translation_input.clone()} />
            </div>
            <div>
                <button onclick={props.on_save.clone()}>{"Save"}</button>
                <button onclick={props.on_cancel.clone()} style="margin-left:8px;">{"Cancel"}</button>
            </div>
        </div>
    }
}
