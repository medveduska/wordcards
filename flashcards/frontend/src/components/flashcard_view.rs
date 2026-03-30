use web_sys::MouseEvent;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FlashcardViewProps {
    pub card_text: Option<String>,
    pub on_card_click: Callback<MouseEvent>,
    pub on_prev: Callback<MouseEvent>,
    pub on_mark_known: Callback<MouseEvent>,
    pub on_delete: Callback<MouseEvent>,
    pub on_next: Callback<MouseEvent>,
}

#[function_component(FlashcardView)]
pub fn flashcard_view(props: &FlashcardViewProps) -> Html {
    let Some(card_text) = props.card_text.clone() else {
        return html! { <p class="empty-note">{"No unknown flashcards remaining."}</p> };
    };

    html! {
        <>
            <div
                onclick={props.on_card_click.clone()}
                class="flashcard"
            >
                { card_text }
            </div>

            <div class="flashcard-actions">
                <button class="btn btn-secondary" onclick={props.on_prev.clone()}>{"<- Prev"}</button>
                <button class="btn btn-primary" onclick={props.on_mark_known.clone()}>{"Mark as Known"}</button>
                <button class="btn btn-danger" onclick={props.on_delete.clone()}>{"Delete"}</button>
                <button class="btn btn-secondary" onclick={props.on_next.clone()}>{"Next ->"}</button>
            </div>
        </>
    }
}
