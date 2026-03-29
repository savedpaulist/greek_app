#![allow(non_snake_case)]

mod components;
mod db;
mod i18n;
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
const SWIPE_JS: Asset = asset!("/assets/swipe.js");
const GOOGLE_FONTS: &str = "https://fonts.googleapis.com/css2?family=Crimson+Pro:wght@400;600;700&family=Literata:wght@400;600;700&family=Lora:wght@400;600;700&family=Noto+Sans:wght@400;600;700&family=Noto+Serif:wght@400;600;700&family=PT+Serif:ital,wght@0,400;0,700;1,400&family=Source+Serif+4:ital,opsz,wght@0,8..60,400;0,8..60,600;1,8..60,400&display=swap";

const SW_REGISTER: &str = r#"
if ('serviceWorker' in navigator) {
  window.addEventListener('load', function() {
    navigator.serviceWorker.register('/greek_app/sw.js', { scope: '/greek_app/' })
      .catch(function(e) { console.warn('SW registration failed:', e); });
  });
}
"#;

fn main() {
    // #[cfg(debug_assertions)]
    // dioxus_devtools::init(); 
    
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
        document::Link { rel: "manifest", href: "/greek_app/manifest.json" }
        document::Meta { name: "theme-color", content: "#1a1a2e" }
        document::Meta { name: "apple-mobile-web-app-capable", content: "yes" }
        document::Meta { name: "apple-mobile-web-app-status-bar-style", content: "black-translucent" }
        document::Link { rel: "apple-touch-icon", href: "/greek_app/icon-192.png" }
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com", crossorigin: "anonymous" }
        document::Link { rel: "stylesheet", href: GOOGLE_FONTS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: THEMES_CSS }
        document::Script { dangerous_inner_html: SW_REGISTER }
        document::Script { src: SWIPE_JS }
        Router::<Route> {}
    }
}
