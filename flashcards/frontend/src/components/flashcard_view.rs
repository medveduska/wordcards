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
        return html! { <p>{"No unknown flashcards remaining."}</p> };
    };

    html! {
        <>
            <div
                onclick={props.on_card_click.clone()}
                style="
                    margin: 50px auto;
                    width: 280px;
                    height: 180px;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    border-radius: 12px;
                    box-shadow: 0 4px 8px rgba(0,0,0,0.2);
                    font-size: 32px;
                    cursor: pointer;
                    user-select: none;
                "
            >
                { card_text }
            </div>

            <div style="margin-top: 10px;">
                <button onclick={props.on_prev.clone()}>{"<- Prev"}</button>
                <button onclick={props.on_mark_known.clone()} style="margin: 0 10px;">{"Mark as Known"}</button>
                <button onclick={props.on_delete.clone()} style="margin: 0 10px; background-color: #ffebee; color: #d32f2f;">{"Delete"}</button>
                <button onclick={props.on_next.clone()}>{"Next ->"}</button>
            </div>
        </>
    }
}
