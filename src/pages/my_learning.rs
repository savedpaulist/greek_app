use std::collections::HashSet;

use dioxus::prelude::*;

use crate::i18n::{t, UiKey};
use crate::logic::diacritics::normalize;
use crate::logic::paradigm::{build_verb_paradigms, mood_label, tense_label, voice_label};
use crate::models::{FilterParams, MyLearningItem};
use crate::router::Route;
use crate::state::AppState;

#[component]
pub fn MyLearningPage() -> Element {
    let mut state = use_context::<AppState>();
    let lang = state.settings.read().language.clone();
    let morph_lang = state.settings.read().morph_language.clone();
    let include_dual = state.settings.read().include_dual;

    let mut add_panel_open = use_signal(|| false);
    let mut search = use_signal(|| String::new());
    let mut configuring_lemma: Signal<Option<i64>> = use_signal(|| None);
    // Which table indices are selected in the table-picker
    let mut sel_tables: Signal<Vec<usize>> = use_signal(Vec::new);

    let my_learning = state.my_learning.read().clone();
    let lemmas_snap = state.lemmas.read().clone();

    // Filtered lemma search results
    let q = search.read().clone();
    let q_norm = normalize(q.trim(), true).to_lowercase();
    let q_lower = q.to_lowercase();
    let mut shown_lemmas: Vec<_> = lemmas_snap
        .iter()
        .filter(|l| {
            if !state.lemma_has_paradigm(l.id) {
                return false;
            }
            if my_learning.iter().any(|item| item.lemma_id == l.id) {
                return false;
            }
            if q.trim().is_empty() {
                return true;
            }
            normalize(&l.greek, true)
                .to_lowercase()
                .contains(&q_norm)
                || l.russian
                    .as_deref()
                    .unwrap_or("")
                    .to_lowercase()
                    .contains(&q_lower)
                || l.english
                    .as_deref()
                    .unwrap_or("")
                    .to_lowercase()
                    .contains(&q_lower)
        })
        .collect();
    shown_lemmas.sort_by(|a, b| normalize(&a.greek, true).cmp(&normalize(&b.greek, true)));

    let is_verb_like = |pos: &str| matches!(pos, "verb" | "participle");

    rsx! {
        div { class: "my-learning-page",

            // ── Header ──────────────────────────────────────────────────────
            div { class: "my-learning-page__header",
                Link { to: Route::Home {}, class: "btn btn--ghost btn--sm",
                    "← {t(UiKey::Back, lang.clone())}"
                }
                h2 { class: "my-learning-page__title",
                    "{t(UiKey::MyLearningPageTitle, lang.clone())}"
                }
            }

            // ── Add word button / panel ──────────────────────────────────────
            if !*add_panel_open.read() {
                button {
                    class: "btn btn--primary my-learning-page__add-btn",
                    onclick: move |_| {
                        *add_panel_open.write() = true;
                        *search.write() = String::new();
                        *configuring_lemma.write() = None;
                    },
                    "+ {t(UiKey::MyLearningAddWord, lang.clone())}"
                }
            } else {
                div { class: "my-learning__add-panel",
                    // Search input
                    input {
                        class: "my-learning__search",
                        r#type: "search",
                        placeholder: t(UiKey::ParadigmSearch, lang.clone()),
                        value: "{search.read()}",
                        oninput: move |e| {
                            *search.write() = e.value();
                            *configuring_lemma.write() = None;
                            *sel_tables.write() = Vec::new();
                        },
                        autofocus: true,
                    }

                    // Lemma results
                    if configuring_lemma.read().is_none() && !shown_lemmas.is_empty() {
                        ul { class: "my-learning__results",
                            for lemma in &shown_lemmas {
                                {
                                    let lid = lemma.id;
                                    let greek = lemma.greek.clone();
                                    let trans = lemma.translation(&lang).to_string();
                                    let pos = lemma.part_of_speech.clone().unwrap_or_default();
                                    let pos2 = pos.clone();
                                    let verb_like = is_verb_like(&pos);
                                    rsx! {
                                        li {
                                            button {
                                                class: "my-learning__pick",
                                                onclick: move |_| {
                                                    if verb_like {
                                                        *configuring_lemma.write() = Some(lid);
                                                        *sel_tables.write() = Vec::new();
                                                    } else {
                                                        state.add_to_my_learning(MyLearningItem {
                                                            lemma_id: lid,
                                                            tenses: vec![],
                                                            voices: vec![],
                                                            moods: vec![],
                                                        });
                                                        *add_panel_open.write() = false;
                                                        *search.write() = String::new();
                                                    }
                                                },
                                                span { class: "my-learning__pick__greek greek-text", "{greek}" }
                                                span { class: "my-learning__pick__pos", "[{pos2}]" }
                                                if !trans.is_empty() {
                                                    span { class: "my-learning__pick__trans", " — {trans}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Table-picker for verb / participle
                    if let Some(cfg_lemma_id) = *configuring_lemma.read() {
                        {
                            let cfg_lemma = lemmas_snap.iter().find(|l| l.id == cfg_lemma_id).cloned();
                            let forms_for_cfg = state.paradigm_forms_for_lemma(cfg_lemma_id);
                            let cfg_greek = cfg_lemma.as_ref().map(|l| l.greek.clone()).unwrap_or_default();
                            let tables = match cfg_lemma.clone() {
                                Some(lem) => build_verb_paradigms(lem, &forms_for_cfg, include_dual, &morph_lang),
                                None => vec![],
                            };
                            let ml2 = morph_lang.clone();
                            rsx! {
                                div { class: "my-learning__verb-config",
                                    p { class: "my-learning__verb-config__heading greek-text", "{cfg_greek}" }
                                    p { class: "my-learning__verb-config__hint",
                                        "{t(UiKey::MyLearningSelectForms, lang.clone())}"
                                    }

                                    // Table picker chips
                                    div { class: "my-learning__table-picker",
                                        for (idx, table) in tables.iter().enumerate() {
                                            {
                                                let title = table.title.clone().unwrap_or_else(|| {
                                                    let mut parts = Vec::new();
                                                    if let Some(k) = &table.tense_key { parts.push(tense_label(k, &ml2).to_string()); }
                                                    if let Some(k) = &table.voice_key { parts.push(voice_label(k, &ml2).to_string()); }
                                                    if let Some(k) = &table.mood_key  { parts.push(mood_label(k,  &ml2).to_string()); }
                                                    parts.join(" · ")
                                                });
                                                let is_sel = sel_tables.read().contains(&idx);
                                                rsx! {
                                                    button {
                                                        class: if is_sel { "filter-chip filter-chip--active" } else { "filter-chip" },
                                                        onclick: move |_| {
                                                            let mut st = sel_tables.write();
                                                            if st.contains(&idx) { st.retain(|x| *x != idx); } else { st.push(idx); }
                                                        },
                                                        "{title}"
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    div { class: "my-learning__verb-config__actions",
                                        button {
                                            class: "btn btn--primary btn--sm",
                                            onclick: {
                                                let tables2 = tables.clone();
                                                move |_| {
                                                    let selected = sel_tables.read().clone();
                                                    if selected.is_empty() {
                                                        // Add all forms
                                                        state.add_to_my_learning(MyLearningItem {
                                                            lemma_id: cfg_lemma_id,
                                                            tenses: vec![],
                                                            voices: vec![],
                                                            moods:  vec![],
                                                        });
                                                    } else {
                                                        for &ti in &selected {
                                                            if let Some(tbl) = tables2.get(ti) {
                                                                state.add_to_my_learning(MyLearningItem {
                                                                    lemma_id: cfg_lemma_id,
                                                                    tenses: tbl.tense_key.iter().cloned().collect(),
                                                                    voices: tbl.voice_key.iter().cloned().collect(),
                                                                    moods:  tbl.mood_key.iter().cloned().collect(),
                                                                });
                                                            }
                                                        }
                                                    }
                                                    *add_panel_open.write() = false;
                                                    *search.write() = String::new();
                                                    *configuring_lemma.write() = None;
                                                    *sel_tables.write() = Vec::new();
                                                }
                                            },
                                            "{t(UiKey::MyLearningAdd, lang.clone())}"
                                        }
                                        button {
                                            class: "btn btn--ghost btn--sm",
                                            onclick: move |_| { *configuring_lemma.write() = None; },
                                            "←"
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Close panel
                    button {
                        class: "btn btn--ghost btn--sm",
                        onclick: move |_| {
                            *add_panel_open.write() = false;
                            *search.write() = String::new();
                            *configuring_lemma.write() = None;
                        },
                        "✕"
                    }
                }
            }

            // ── Current items list ────────────────────────────────────────────
            if my_learning.is_empty() {
                p { class: "my-learning__desc", "{t(UiKey::HomeMyLearningDesc, lang.clone())}" }
            } else {
                {
                    let mut seen_ids: Vec<i64> = Vec::new();
                    for item in my_learning.iter() {
                        if !seen_ids.contains(&item.lemma_id) {
                            seen_ids.push(item.lemma_id);
                        }
                    }
                    let ml = morph_lang.clone();
                    rsx! {
                        ul { class: "my-learning__list",
                            for lemma_id_g in seen_ids {
                                {
                                    let group: Vec<_> = my_learning
                                        .iter()
                                        .filter(|i| i.lemma_id == lemma_id_g)
                                        .cloned()
                                        .collect();
                                    let lemma_greek = lemmas_snap
                                        .iter()
                                        .find(|l| l.id == lemma_id_g)
                                        .map(|l| l.greek.clone())
                                        .unwrap_or_default();
                                    let lemma_pos = lemmas_snap
                                        .iter()
                                        .find(|l| l.id == lemma_id_g)
                                        .and_then(|l| l.part_of_speech.clone())
                                        .unwrap_or_default();
                                    let has_tags = group.iter().any(|i| {
                                        !i.tenses.is_empty() || !i.voices.is_empty() || !i.moods.is_empty()
                                    });
                                    let grouped = group.len() > 1 || has_tags;
                                    let ml2 = ml.clone();
                                    if grouped {
                                        rsx! {
                                            li { class: "my-learning__group",
                                                div { class: "my-learning__group-header",
                                                    span { class: "my-learning__lemma greek-text", "{lemma_greek}" }
                                                    span { class: "my-learning__pos", "[{lemma_pos}]" }
                                                }
                                                ul { class: "my-learning__sub-list",
                                                    for sub_item in group {
                                                        {
                                                            let ml3 = ml2.clone();
                                                            let tags_text = {
                                                                let mut parts: Vec<String> = Vec::new();
                                                                if !sub_item.tenses.is_empty() {
                                                                    parts.push(sub_item.tenses.iter().map(|s| tense_label(s, &ml3)).collect::<Vec<_>>().join(" · "));
                                                                }
                                                                if !sub_item.voices.is_empty() {
                                                                    parts.push(sub_item.voices.iter().map(|s| voice_label(s, &ml3)).collect::<Vec<_>>().join(" · "));
                                                                }
                                                                if !sub_item.moods.is_empty() {
                                                                    parts.push(sub_item.moods.iter().map(|s| mood_label(s, &ml3)).collect::<Vec<_>>().join(" · "));
                                                                }
                                                                if parts.is_empty() {
                                                                    t(UiKey::MyLearningAllForms, lang.clone()).to_string()
                                                                } else {
                                                                    parts.join(" · ")
                                                                }
                                                            };
                                                            rsx! {
                                                                li { class: "my-learning__sub-item",
                                                                    span { class: "my-learning__tags", "{tags_text}" }
                                                                    button {
                                                                        class: "my-learning__remove",
                                                                        onclick: move |_| { state.remove_learning_item(&sub_item); },
                                                                        "×"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        rsx! {
                                            li { class: "my-learning__item",
                                                span { class: "my-learning__lemma greek-text", "{lemma_greek}" }
                                                span { class: "my-learning__pos", "[{lemma_pos}]" }
                                                button {
                                                    class: "my-learning__remove",
                                                    onclick: move |_| { state.remove_from_my_learning(lemma_id_g); },
                                                    "×"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
