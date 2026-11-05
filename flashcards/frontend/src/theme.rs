use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;

const THEME_KEY: &str = "flashcards_theme";

#[function_component(ThemeToggle)]
pub fn theme_toggle() -> Html {
    let dark = use_state(|| LocalStorage::get::<String>(THEME_KEY).unwrap_or_default() == "dark");
    let onclick = {
        let dark = dark.clone();
        Callback::from(move |_| {
            let next = !*dark;
            let theme = if next { "dark" } else { "light" };
            if let Some(root) = web_sys::window()
                .and_then(|window| window.document())
                .and_then(|document| document.document_element())
            {
                let _ = root.set_attribute("data-theme", theme);
            }
            let _ = LocalStorage::set(THEME_KEY, theme);
            dark.set(next);
        })
    };

    html! {
        <button class="btn btn-secondary btn-small" {onclick}
            aria-label="Dark mode" aria-pressed={dark.to_string()}
            title={if *dark { "Switch to light mode" } else { "Switch to dark mode" }}>
            {if *dark { "☀ Light mode" } else { "☾ Dark mode" }}
        </button>
    }
}
