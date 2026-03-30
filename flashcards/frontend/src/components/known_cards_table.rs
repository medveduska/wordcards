use yew::prelude::*;

use crate::model::Flashcard;

#[derive(Properties, PartialEq)]
pub struct KnownCardsTableProps {
    pub known_cards: Vec<Flashcard>,
    pub on_restore: Callback<usize>,
    pub on_delete: Callback<usize>,
}

#[function_component(KnownCardsTable)]
pub fn known_cards_table(props: &KnownCardsTableProps) -> Html {
    html! {
        <>
            <h3 style="margin-top: 40px;">{"Known Words"}</h3>
            <table style="margin: 0 auto; border-collapse: collapse;">
                <tr>
                    <th style="padding: 5px; border-bottom: 1px solid #ccc;">{"Word"}</th>
                    <th style="padding: 5px; border-bottom: 1px solid #ccc;">{"Pinyin"}</th>
                    <th style="padding: 5px; border-bottom: 1px solid #ccc;">{"Translation"}</th>
                    <th style="padding: 5px; border-bottom: 1px solid #ccc;">{"Action"}</th>
                </tr>
                { for props.known_cards.iter().enumerate().map(|(index, card)| {
                    let on_restore = props.on_restore.clone();
                    let on_delete = props.on_delete.clone();

                    html! {
                        <tr>
                            <td style="padding: 5px;">{ &card.word }</td>
                            <td style="padding: 5px;">{ card.pinyin.as_deref().unwrap_or_default() }</td>
                            <td style="padding: 5px;">{ &card.translation }</td>
                            <td style="padding: 5px;">
                                <button onclick={Callback::from(move |_| on_restore.emit(index))}>{"Restore"}</button>
                                <button onclick={Callback::from(move |_| on_delete.emit(index))} style="margin-left: 5px; background-color: #ffebee; color: #d32f2f;">{"Delete"}</button>
                            </td>
                        </tr>
                    }
                }) }
            </table>
        </>
    }
}
