use dioxus::prelude::*;

use crate::components::flashcard::FlashcardView;

#[component]
pub fn FlashcardRevPage() -> Element {
    rsx! {
        div { class: "study-page",
            h2 { class: "study-page__title", "Обратные карточки" }
            FlashcardView { reverse: true }
        }
    }
}
