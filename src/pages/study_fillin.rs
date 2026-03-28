use dioxus::prelude::*;

use crate::components::fillin::FillInView;
use crate::i18n::{t, UiKey};
use crate::state::AppState;

#[component]
pub fn FillInPage() -> Element {
    let state = use_context::<AppState>();
    let title = t(UiKey::ModeFillIn, state.settings.read().language.clone());
    rsx! {
        div { class: "study-page",
            h2 { class: "study-page__title", "{title}" }
            FillInView {}
        }
    }
}
