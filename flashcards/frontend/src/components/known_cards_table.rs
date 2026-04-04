use yew::prelude::*;

use crate::model::Flashcard;

#[derive(Properties, PartialEq)]
pub struct KnownCardsTableProps {
    pub known_cards: Vec<Flashcard>,
    pub total: usize,
    pub on_restore: Callback<usize>,
    pub on_delete: Callback<usize>,
}

#[function_component(KnownCardsTable)]
pub fn known_cards_table(props: &KnownCardsTableProps) -> Html {
    let counter = if props.total > 0 {
        html! { <p class="status-chip">{ format!("Known: {} / {}", props.known_cards.len(), props.total) }</p> }
    } else {
        html! {}
    };
    html! {
        <section class="known-panel">
            <h3 class="panel-title known-title">{"Known Words"}</h3>
            { counter }
            <table class="known-table">
                <tr>
                    <th>{"Word"}</th>
                    <th>{"Pinyin"}</th>
                    <th>{"Translation"}</th>
                    <th>{"Action"}</th>
                </tr>
                { for props.known_cards.iter().enumerate().map(|(index, card)| {
                    let on_restore = props.on_restore.clone();
                    let on_delete = props.on_delete.clone();

                    html! {
                        <tr>
                            <td>{ &card.word }</td>
                            <td>{ card.pinyin.as_deref().unwrap_or_default() }</td>
                            <td>{ &card.translation }</td>
                            <td class="known-actions-cell">
                                <div class="known-actions-group">
                                    <button class="btn btn-secondary" onclick={Callback::from(move |_| on_restore.emit(index))}>{"Restore"}</button>
                                    <button class="btn btn-danger" onclick={Callback::from(move |_| on_delete.emit(index))}>{"Delete"}</button>
                                </div>
                            </td>
                        </tr>
                    }
                }) }
            </table>
        </section>
    }
}
