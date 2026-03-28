use dioxus::prelude::*;

use crate::components::flashcard::FlashcardView;
use crate::i18n::{t, UiKey};
use crate::state::AppState;

#[component]
pub fn FlashcardPage() -> Element {
    let state = use_context::<AppState>();
    let title = t(UiKey::ModeFlashcard, state.settings.read().language.clone());
    rsx! {
        div { class: "study-page",
            h2 { class: "study-page__title", "{title}" }
            FlashcardView { reverse: false }
        }
    }
}
