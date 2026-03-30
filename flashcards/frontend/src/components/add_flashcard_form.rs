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
        <div class="panel add-form-panel">
            <h3 class="panel-title">{"Add New Flashcard"}</h3>
            <div class="field-row">
                <input class="text-input" placeholder="Chinese character" value={props.new_word.clone()} oninput={props.on_word_input.clone()} />
            </div>
            <div class="field-row">
                <input class="text-input" placeholder="Pinyin" value={props.new_pinyin.clone()} oninput={props.on_pinyin_input.clone()} />
            </div>
            <div class="field-row">
                <input class="text-input" placeholder="Translation" value={props.new_translation.clone()} oninput={props.on_translation_input.clone()} />
            </div>
            <div class="form-actions">
                <button class="btn btn-primary" onclick={props.on_save.clone()}>{"Save"}</button>
                <button class="btn btn-muted" onclick={props.on_cancel.clone()}>{"Cancel"}</button>
            </div>
        </div>
    }
}
