use dioxus::prelude::*;

use crate::router::Route;
use crate::state::AppState;

#[component]
pub fn HomePage() -> Element {
    let state = use_context::<AppState>();
    let stats = state.stats();

    rsx! {
        div { class: "home-page",
            // Hero header
            div { class: "home-hero",
                h1 { class: "home-hero__title greek-text", "σφόδρα" }
                p { class: "home-hero__subtitle", "Тренажёр древнегреческих парадигм" }
            }

            // Quick stats
            div { class: "home-stats",
                StatBadge { label: "Форм в базе", value: stats.total.to_string() }
                StatBadge { label: "Встречено", value: stats.seen.to_string() }
                StatBadge { label: "Выучено", value: stats.learned.to_string() }
                StatBadge {
                    label: "Точность",
                    value: format!("{:.0}%", stats.accuracy * 100.0),
                }
            }

            // Study mode cards
            h2 { class: "home-section-title", "Выберите режим" }
            div { class: "mode-grid",
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
                        strong { class: "mode-card__title", "Просмотр парадигм" }
                        p { class: "mode-card__desc", "Изучайте таблицы склонений и спряжений" }
                    }
                }
                Link { to: Route::Flashcard {}, class: "mode-card",
                    span { class: "mode-card__icon",
                        svg { xmlns: "http://www.w3.org/2000/svg", width: "24", height: "24", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                            rect { x: "2", y: "5", width: "20", height: "14", rx: "2" }
                            line { x1: "2", y1: "10", x2: "22", y2: "10" }
                        }
                    }
                    div { class: "mode-card__text",
                        strong { class: "mode-card__title", "Карточки" }
                        p { class: "mode-card__desc", "Слово → форма: 4 варианта ответа" }
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
                        strong { class: "mode-card__title", "Обратные карточки" }
                        p { class: "mode-card__desc", "Форма → словарное слово" }
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
                        strong { class: "mode-card__title", "Вписать форму" }
                        p { class: "mode-card__desc", "Введите форму по грамматическому описанию" }
                    }
                }
            }

            // Onboarding hint
            div { class: "home-tip",
                p { "Используйте фильтр (меню) для выбора раздела или урока" }
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
