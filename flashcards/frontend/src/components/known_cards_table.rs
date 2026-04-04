use web_sys::MouseEvent;
use yew::prelude::*;

use crate::model::Flashcard;

#[derive(Properties, PartialEq)]
pub struct KnownCardsTableProps {
    pub known_cards: Vec<Flashcard>,
    pub unknown_cards: Vec<Flashcard>,
    pub total: usize,
    pub show_unknown: bool,
    pub on_restore: Callback<usize>,
    pub on_delete: Callback<usize>,
    pub on_toggle_unknown: Callback<MouseEvent>,
    pub on_mark_known_from_table: Callback<usize>,
    pub on_delete_unknown: Callback<usize>,
}

#[function_component(KnownCardsTable)]
pub fn known_cards_table(props: &KnownCardsTableProps) -> Html {
    let counter = if props.total > 0 {
        html! { <p class="status-chip">{ format!("Known: {} / {}", props.known_cards.len(), props.total) }</p> }
    } else {
        html! {}
    };

    let toggle_label = if props.show_unknown {
        "Hide Unknown Words"
    } else {
        "Show Unknown Words"
    };

    let status_column_header = if props.show_unknown {
        html! { <th>{"Status"}</th> }
    } else {
        html! {}
    };

    let known_rows = props.known_cards.iter().enumerate().map(|(index, card)| {
        let on_restore = props.on_restore.clone();
        let on_delete = props.on_delete.clone();
        let status_cell = if props.show_unknown {
            html! { <td><span class="status-badge-known">{"Known"}</span></td> }
        } else {
            html! {}
        };
        html! {
            <tr>
                <td>{ &card.word }</td>
                <td>{ card.pinyin.as_deref().unwrap_or_default() }</td>
                <td>{ &card.translation }</td>
                { status_cell }
                <td class="known-actions-cell">
                    <div class="known-actions-group">
                        <button class="btn btn-secondary" onclick={Callback::from(move |_| on_restore.emit(index))}>{"Restore"}</button>
                        <button class="btn btn-danger" onclick={Callback::from(move |_| on_delete.emit(index))}>{"Delete"}</button>
                    </div>
                </td>
            </tr>
        }
    });

    let unknown_rows = if props.show_unknown {
        props.unknown_cards.iter().enumerate().map(|(index, card)| {
            let on_mark = props.on_mark_known_from_table.clone();
            let on_delete = props.on_delete_unknown.clone();
            html! {
                <tr>
                    <td>{ &card.word }</td>
                    <td>{ card.pinyin.as_deref().unwrap_or_default() }</td>
                    <td>{ &card.translation }</td>
                    <td><span class="status-badge-unknown">{"Unknown"}</span></td>
                    <td class="known-actions-cell">
                        <div class="known-actions-group">
                            <button class="btn btn-primary" onclick={Callback::from(move |_| on_mark.emit(index))}>{"Mark as Known"}</button>
                            <button class="btn btn-danger" onclick={Callback::from(move |_| on_delete.emit(index))}>{"Delete"}</button>
                        </div>
                    </td>
                </tr>
            }
        }).collect::<Html>()
    } else {
        html! {}
    };

    html! {
        <section class="known-panel">
            <h3 class="panel-title known-title">{"Word Review"}</h3>
            { counter }
            <div class="panel-actions">
                <button class="btn btn-secondary" onclick={props.on_toggle_unknown.clone()}>
                    { toggle_label }
                </button>
            </div>
            <table class="known-table">
                <tr>
                    <th>{"Word"}</th>
                    <th>{"Pinyin"}</th>
                    <th>{"Translation"}</th>
                    { status_column_header }
                    <th>{"Action"}</th>
                </tr>
                { for known_rows }
                { unknown_rows }
            </table>
        </section>
    }
}
