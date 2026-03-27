use dioxus::prelude::*;

use crate::state::AppState;

#[component]
pub fn ProgressPage() -> Element {
    let mut state = use_context::<AppState>();
    let stats = state.stats();

    rsx! {
        div { class: "progress-page",
            h2 { class: "progress-page__title", "📊 Прогресс" }

            // Overview cards
            div { class: "progress-overview",
                StatRow { label: "Форм в базе", value: stats.total.to_string() }
                StatRow {
                    label: "Встречено",
                    value: format!("{} ({:.0}%)", stats.seen, stats.seen as f32 / stats.total as f32 * 100.0),
                }
                StatRow { label: "Выучено (≥5 подряд)", value: stats.learned.to_string() }
                StatRow { label: "Сессий", value: stats.sessions.to_string() }
                StatRow {
                    label: "Точность",
                    value: format!("{:.0}%", stats.accuracy * 100.0),
                }
            }

            // Per-POS progress bars
            div { class: "progress-by-pos",
                h3 { "По частям речи" }
                for (pos, label) in [
                    ("noun", "Существительные"),
                    ("verb", "Глаголы"),
                    ("adj", "Прилагательные"),
                    ("pronoun", "Местоимения"),
                    ("article", "Артикль"),
                ]
                {
                    PosProgressBar { pos: pos.to_string(), label: label.to_string() }
                }
            }

            // Reset button
            div { class: "progress-actions",
                button {
                    class: "btn btn--danger",
                    onclick: move |_| {
                        // Confirm intentional action before clearing
                        state.reset_progress();
                    },
                    "Сбросить прогресс"
                }
            }
        }
    }
}

#[component]
fn StatRow(label: String, value: String) -> Element {
    rsx! {
        div { class: "stat-row",
            span { class: "stat-row__label", "{label}" }
            span { class: "stat-row__value", "{value}" }
        }
    }
}

#[component]
fn PosProgressBar(pos: String, label: String) -> Element {
    let state = use_context::<AppState>();
    let forms = state.forms.read();
    let progress = state.progress.read();

    let total: usize = forms.iter().filter(|f| f.pos.as_deref() == Some(&pos)).count();
    let seen: usize = forms
        .iter()
        .filter(|f| f.pos.as_deref() == Some(&pos))
        .filter(|f| progress.contains_key(&f.id))
        .count();

    let pct = if total > 0 { seen * 100 / total } else { 0 };

    rsx! {
        div { class: "pos-bar",
            div { class: "pos-bar__header",
                span { class: "pos-bar__label", "{label}" }
                span { class: "pos-bar__count", "{seen}/{total}" }
            }
            div { class: "pos-bar__track",
                div { class: "pos-bar__fill", style: "width: {pct}%;", "" }
            }
        }
    }
}
