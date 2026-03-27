use dioxus::prelude::*;

use crate::components::flashcard::FlashcardView;

#[component]
pub fn FlashcardPage() -> Element {
    rsx! {
        div { class: "study-page",
            h2 { class: "study-page__title", "🃏 Карточки" }
            FlashcardView { reverse: false }
        }
    }
}
