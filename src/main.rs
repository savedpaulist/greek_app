#![allow(non_snake_case)]

mod components;
mod db;
mod logic;
mod models;
mod pages;
mod router;
mod state;

use dioxus::prelude::*;

use crate::router::Route;
use crate::state::AppState;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const THEMES_CSS: Asset = asset!("/assets/styles/themes.css");
const GOOGLE_FONTS: &str = "https://fonts.googleapis.com/css2?family=Crimson+Pro:wght@400;600;700&family=Literata:wght@400;600;700&family=Lora:wght@400;600;700&family=Noto+Sans:wght@400;600;700&family=Noto+Serif:wght@400;600;700&display=swap";

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Load all data from embedded JSON (built by build.rs).
    let data = db::load();

    // Initialise AppState and provide it to the component tree.
    let app_state = AppState::init(data.forms, data.lemmas, data.categories, data.tags);
    use_context_provider(|| app_state);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com", crossorigin: "anonymous" }
        document::Link { rel: "stylesheet", href: GOOGLE_FONTS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: THEMES_CSS }
        Router::<Route> {}
    }
}
