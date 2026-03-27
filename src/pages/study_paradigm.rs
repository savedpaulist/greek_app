use dioxus::prelude::*;

use crate::components::paradigm_view::LemmaPicker;

#[component]
pub fn ParadigmViewPage() -> Element {
    rsx! {
        div { class: "study-page study-page--paradigm",
            h2 { class: "study-page__title", "📖 Просмотр парадигм" }
            LemmaPicker {}
        }
    }
}
