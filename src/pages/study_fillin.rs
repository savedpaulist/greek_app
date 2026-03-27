use dioxus::prelude::*;

use crate::components::fillin::FillInView;

#[component]
pub fn FillInPage() -> Element {
    rsx! {
        div { class: "study-page",
            h2 { class: "study-page__title", "✏️ Вписать форму" }
            FillInView {}
        }
    }
}
