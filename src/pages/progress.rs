use dioxus::prelude::*;

use crate::i18n::{t, UiKey};
use crate::state::AppState;

#[component]
pub fn ProgressPage() -> Element {
    let mut state = use_context::<AppState>();
    let stats = state.stats();
    let settings_snap = state.settings.read().clone();
    let lang = settings_snap.language.clone();
    let morph_lang = settings_snap.morph_language.clone();

    rsx! {
        div { class: "progress-page",
            h2 { class: "progress-page__title", "{t(UiKey::ProgressTitle, lang.clone())}" }

            div { class: "progress-overview",
                StatRow { label: t(UiKey::ProgressStatForms, lang.clone()).to_string(), value: stats.total.to_string() }
                StatRow {
                    label: t(UiKey::ProgressStatSeen, lang.clone()).to_string(),
                    value: format!("{} ({:.0}%)", stats.seen, stats.seen as f32 / stats.total as f32 * 100.0),
                }
                StatRow { label: t(UiKey::ProgressStatLearned, lang.clone()).to_string(), value: stats.learned.to_string() }
                StatRow { label: t(UiKey::ProgressStatSessions, lang.clone()).to_string(), value: stats.sessions.to_string() }
                StatRow {
                    label: t(UiKey::ProgressStatAccuracy, lang.clone()).to_string(),
                    value: format!("{:.0}%", stats.accuracy * 100.0),
                }
            }

            div { class: "progress-by-pos",
                h3 { "{t(UiKey::ProgressByPos, lang.clone())}" }
                for (pos, key) in [
                    ("noun",    UiKey::FilterPosNoun),
                    ("verb",    UiKey::FilterPosVerb),
                    ("adj",     UiKey::FilterPosAdj),
                    ("pronoun", UiKey::FilterPosPronoun),
                    ("article", UiKey::FilterPosArticle),
                ]
                {
                    PosProgressBar { pos: pos.to_string(), label: t(key, morph_lang.clone()).to_string() }
                }
            }

            div { class: "progress-actions",
                button {
                    class: "btn btn--danger",
                    onclick: move |_| {
                        state.reset_progress();
                    },
                    "{t(UiKey::ProgressResetBtn, lang.clone())}"
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
