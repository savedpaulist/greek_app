use dioxus::prelude::*;

use crate::pages::{
    home::HomePage as Home,
    progress::ProgressPage as Progress,
    settings::SettingsPage as Settings,
    study_fillin::FillInPage as FillIn,
    study_flashcard::FlashcardPage as Flashcard,
    study_flashcard_rev::FlashcardRevPage as FlashcardReverse,
    study_paradigm::ParadigmViewPage as ParadigmView,
};

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[layout(AppShell)]
    #[route("/")]
    Home {},
    #[route("/study/paradigm")]
    ParadigmView {},
    #[route("/study/flashcard")]
    Flashcard {},
    #[route("/study/flashcard-reverse")]
    FlashcardReverse {},
    #[route("/study/fillin")]
    FillIn {},
    #[route("/progress")]
    Progress {},
    #[route("/settings")]
    Settings {},
}

// ── Layout shell (wraps all pages) ─────────────────────────────────────────

#[component]
fn AppShell() -> Element {
    rsx! {
        crate::components::shell::Shell {}
    }
}
