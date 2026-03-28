use dioxus::prelude::*;

use crate::components::paradigm_view::LemmaPicker;
use crate::i18n::{t, UiKey};
use crate::state::AppState;

#[component]
pub fn ParadigmViewPage() -> Element {
    let state = use_context::<AppState>();
    let title = t(UiKey::ModeParadigm, state.settings.read().language.clone());
    rsx! {
        div { class: "study-page study-page--paradigm",
            h2 { class: "study-page__title", "{title}" }
            LemmaPicker {}
        }
    }
}
