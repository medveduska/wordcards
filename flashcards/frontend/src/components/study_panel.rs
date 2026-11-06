use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{Element, Event, KeyboardEvent};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct StudyPanelProps {
    pub children: Children,
}

#[function_component(StudyPanel)]
pub fn study_panel(props: &StudyPanelProps) -> Html {
    let panel = use_node_ref();
    let toggle = use_node_ref();
    let expanded = use_state(|| false);

    {
        let panel = panel.clone();
        let toggle = toggle.clone();
        let expanded = expanded.clone();
        use_effect_with(*expanded, move |_| {
            let document = web_sys::window().unwrap().document().unwrap();
            let changed_document = document.clone();
            let changed_expanded = expanded.clone();
            let changed_toggle = toggle.clone();
            let key_panel = panel.clone();
            let on_change = Closure::<dyn Fn(Event)>::new(move |_| {
                let active = changed_document.fullscreen_element().is_some()
                    && changed_document.fullscreen_element() == panel.cast::<Element>();
                changed_expanded.set(active);
                if !active {
                    if let Some(button) = changed_toggle.cast::<web_sys::HtmlElement>() {
                        let _ = button.focus();
                    }
                }
            });
            let key_document = document.clone();
            let on_key = Closure::<dyn Fn(KeyboardEvent)>::new(move |event: KeyboardEvent| {
                if !*expanded {
                    return;
                }
                if event.key() == "Tab" {
                    if let Some(element) = key_panel.cast::<Element>() {
                        if let Ok(controls) = element.query_selector_all("button, input, summary") {
                            let first = controls.item(0);
                            let last = controls.item(controls.length().saturating_sub(1));
                            let active = key_document.active_element().map(web_sys::Node::from);
                            let target = if event.shift_key() && active == first {
                                last
                            } else if !event.shift_key() && active == last {
                                first
                            } else {
                                None
                            };
                            if let Some(control) =
                                target.and_then(|node| node.dyn_into::<web_sys::HtmlElement>().ok())
                            {
                                event.prevent_default();
                                let _ = control.focus();
                            }
                        }
                    }
                }
                if event.key() == "Escape" {
                    if key_document.fullscreen_element().is_some() {
                        key_document.exit_fullscreen();
                    }
                    expanded.set(false);
                    if let Some(button) = toggle.cast::<web_sys::HtmlElement>() {
                        let _ = button.focus();
                    }
                }
            });
            let _ = document.add_event_listener_with_callback(
                "fullscreenchange",
                on_change.as_ref().unchecked_ref(),
            );
            let _ = document
                .add_event_listener_with_callback("keydown", on_key.as_ref().unchecked_ref());
            move || {
                let _ = document.remove_event_listener_with_callback(
                    "fullscreenchange",
                    on_change.as_ref().unchecked_ref(),
                );
                let _ = document.remove_event_listener_with_callback(
                    "keydown",
                    on_key.as_ref().unchecked_ref(),
                );
            }
        });
    }

    let on_toggle = {
        let panel = panel.clone();
        let expanded = expanded.clone();
        Callback::from(move |_| {
            let document = web_sys::window().unwrap().document().unwrap();
            if *expanded {
                if document.fullscreen_element().is_some() {
                    document.exit_fullscreen();
                }
                expanded.set(false);
            } else if let Some(element) = panel.cast::<Element>() {
                // Keep the expanded layout if the browser cannot enter native fullscreen.
                expanded.set(true);
                let _ = element.request_fullscreen();
            }
        })
    };

    html! {
        <section ref={panel} class={classes!("unknown-panel", "panel", expanded.then_some("study-expanded"))}
            aria-label="Flashcards">
            <div class="study-panel-heading">
                <h3 class="panel-title">{"Flashcards"}</h3>
                <button ref={toggle} type="button" class="btn btn-secondary btn-small"
                    onclick={on_toggle} aria-pressed={expanded.to_string()}
                    title={if *expanded { "Exit fullscreen (Esc)" } else { "Study in fullscreen" }}>
                    {if *expanded { "Exit fullscreen" } else { "⛶ Fullscreen" }}
                </button>
            </div>
            { for props.children.iter() }
        </section>
    }
}
