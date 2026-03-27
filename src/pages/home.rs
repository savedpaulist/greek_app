use dioxus::prelude::*;

use crate::i18n::{t, UiKey};
use crate::models::FilterParams;
use crate::router::Route;
use crate::state::AppState;

#[component]
pub fn HomePage() -> Element {
    let mut state = use_context::<AppState>();
    let stats = state.stats();
    let lang = state.settings.read().language.clone();

    let filter = state.filter.read();
    let has_filter = !filter.pos.is_empty()
        || !filter.lemma_ids.is_empty()
        || !filter.tenses.is_empty()
        || !filter.cases.is_empty()
        || !filter.voices.is_empty()
        || !filter.moods.is_empty()
        || !filter.persons.is_empty()
        || !filter.numbers.is_empty()
        || !filter.genders.is_empty();
    drop(filter);

    rsx! {
        div { class: "home-page",
            // Hero header
            div { class: "home-hero",
                h1 { class: "home-hero__title greek-text", "σφόδρα" }
                p { class: "home-hero__subtitle", "{t(UiKey::HomeSubtitle, lang.clone())}" }
            }

            // Quick stats
            div { class: "home-stats",
                StatBadge { label: t(UiKey::HomeStatForms, lang.clone()).to_string(), value: stats.total.to_string() }
                StatBadge { label: t(UiKey::HomeStatSeen, lang.clone()).to_string(), value: stats.seen.to_string() }
                StatBadge { label: t(UiKey::HomeStatLearned, lang.clone()).to_string(), value: stats.learned.to_string() }
                StatBadge {
                    label: t(UiKey::HomeStatAccuracy, lang.clone()).to_string(),
                    value: format!("{:.0}%", stats.accuracy * 100.0),
                }
            }

            // Study mode cards
            h2 { class: "home-section-title", "{t(UiKey::HomeModeSelect, lang.clone())}" }
            div { class: "mode-grid",
                Link { to: Route::Flashcard {}, class: "mode-card",
                    span { class: "mode-card__icon",
                        svg { xmlns: "http://www.w3.org/2000/svg", width: "24", height: "24", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                            rect { x: "2", y: "5", width: "20", height: "14", rx: "2" }
                            line { x1: "2", y1: "10", x2: "22", y2: "10" }
                        }
                    }
                    div { class: "mode-card__text",
                        strong { class: "mode-card__title",
                            "{t(UiKey::ModeFlashcard, lang.clone())}"
                            if has_filter { span { class: "mode-card__filter-badge", " ({t(UiKey::HomeFilterBadge, lang.clone())})" } }
                        }
                        p { class: "mode-card__desc", "{t(UiKey::ModeFlashcardDesc, lang.clone())}" }
                    }
                }
                Link { to: Route::FlashcardReverse {}, class: "mode-card",
                    span { class: "mode-card__icon",
                        svg { xmlns: "http://www.w3.org/2000/svg", width: "24", height: "24", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                            polyline { points: "1 4 1 10 7 10" }
                            path { d: "M3.51 15a9 9 0 1 0 .49-3" }
                        }
                    }
                    div { class: "mode-card__text",
                        strong { class: "mode-card__title",
                            "{t(UiKey::ModeFlashcardRev, lang.clone())}"
                            if has_filter { span { class: "mode-card__filter-badge", " ({t(UiKey::HomeFilterBadge, lang.clone())})" } }
                        }
                        p { class: "mode-card__desc", "{t(UiKey::ModeFlashcardRevDesc, lang.clone())}" }
                    }
                }
                Link { to: Route::FillIn {}, class: "mode-card",
                    span { class: "mode-card__icon",
                        svg { xmlns: "http://www.w3.org/2000/svg", width: "24", height: "24", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                            path { d: "M12 20h9" }
                            path { d: "M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z" }
                        }
                    }
                    div { class: "mode-card__text",
                        strong { class: "mode-card__title",
                            "{t(UiKey::ModeFillIn, lang.clone())}"
                            if has_filter { span { class: "mode-card__filter-badge", " ({t(UiKey::HomeFilterBadge, lang.clone())})" } }
                        }
                        p { class: "mode-card__desc", "{t(UiKey::ModeFillInDesc, lang.clone())}" }
                    }
                }
                Link { to: Route::ParadigmView {}, class: "mode-card",
                    span { class: "mode-card__icon",
                        svg { xmlns: "http://www.w3.org/2000/svg", width: "24", height: "24", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                            path { d: "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" }
                            polyline { points: "14 2 14 8 20 8" }
                            line { x1: "16", y1: "13", x2: "8", y2: "13" }
                            line { x1: "16", y1: "17", x2: "8", y2: "17" }
                            polyline { points: "10 9 9 9 8 9" }
                        }
                    }
                    div { class: "mode-card__text",
                        strong { class: "mode-card__title", "{t(UiKey::ModeParadigm, lang.clone())}" }
                        p { class: "mode-card__desc", "{t(UiKey::ModeParadigmDesc, lang.clone())}" }
                    }
                }
                Link { to: Route::ParadigmBuilder {}, class: "mode-card",
                    span { class: "mode-card__icon",
                        svg { xmlns: "http://www.w3.org/2000/svg", width: "24", height: "24", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                            line { x1: "12", y1: "5", x2: "12", y2: "19" }
                            line { x1: "5", y1: "12", x2: "19", y2: "12" }
                        }
                    }
                    div { class: "mode-card__text",
                        strong { class: "mode-card__title", "{t(UiKey::ModeBuilderTitle, lang.clone())}" }
                        p { class: "mode-card__desc", "{t(UiKey::ModeBuilderDesc, lang.clone())}" }
                    }
                }
            }
            // Filter hint
            if has_filter {
                div { class: "home-tip home-tip--filter",
                    p { "{t(UiKey::HomeFilterActive, lang.clone())}" }
                    button {
                        class: "btn btn--ghost btn--sm",
                        onclick: move |_| *state.filter.write() = FilterParams::default(),
                        "{t(UiKey::FiltersReset, lang.clone())}"
                    }
                }
            }
            div { class: "home-tip",
                p { "{t(UiKey::HomeTipFilter, lang.clone())}" }
            }
        }
    }
}

#[component]
fn StatBadge(label: String, value: String) -> Element {
    rsx! {
        div { class: "stat-badge",
            span { class: "stat-badge__value", "{value}" }
            span { class: "stat-badge__label", "{label}" }
        }
    }
}
