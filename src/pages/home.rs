use std::collections::HashSet;

use dioxus::prelude::*;

use crate::i18n::{t, UiKey};
use crate::logic::diacritics::normalize;
use crate::logic::paradigm::{mood_label, tense_label, voice_label};
use crate::models::{FilterParams, MyLearningItem};
use crate::router::Route;
use crate::state::AppState;

#[component]
pub fn HomePage() -> Element {
    let mut state = use_context::<AppState>();
    let stats = state.stats();
    let lang = state.settings.read().language.clone();

    let filter = state.filter.read();
    let my_learning_active = *state.my_learning_active.read();
    let my_learning_empty = state.my_learning.read().is_empty();

    let has_filter = !my_learning_active
        || !filter.pos.is_empty()
        || !filter.lemma_ids.is_empty()
        || !filter.tenses.is_empty()
        || !filter.cases.is_empty()
        || !filter.voices.is_empty()
        || !filter.moods.is_empty()
        || !filter.persons.is_empty()
        || !filter.numbers.is_empty()
        || !filter.genders.is_empty();
    drop(filter);

    let forms_in_learning = state.my_learning_forms_count();

    rsx! {
        div { class: "home-page",
            // Hero header
            div { class: "home-hero",
                h1 { class: "home-hero__title greek-text", "σφόδρα" }
                p { class: "home-hero__subtitle", "{t(UiKey::HomeSubtitle, lang.clone())}" }
            }

            // Quick stats
            div { class: "home-stats",
                StatBadge {
                    label: t(UiKey::HomeStatFormsLearning, lang.clone()).to_string(),
                    value: forms_in_learning.to_string(),
                }
                StatBadge { label: t(UiKey::HomeStatSeen, lang.clone()).to_string(), value: stats.seen.to_string() }
                StatBadge { label: t(UiKey::HomeStatLearned, lang.clone()).to_string(), value: stats.learned.to_string() }
                StatBadge {
                    label: t(UiKey::HomeStatAccuracy, lang.clone()).to_string(),
                    value: format!("{:.0}%", stats.accuracy * 100.0),
                }
            }

            // My Learning compact card
            {
                let word_count = state.my_learning.read().iter()
                    .map(|i| i.lemma_id)
                    .collect::<std::collections::HashSet<_>>()
                    .len();
                let form_count = forms_in_learning;
                rsx! {
                    div { class: "my-learning-card",
                        div { class: "my-learning-card__info",
                            h3 { class: "my-learning-card__title",
                                "{t(UiKey::HomeMyLearning, lang.clone())}"
                            }
                            div { class: "my-learning-card__stats",
                                span { "{word_count} {t(UiKey::MyLearningWords, lang.clone())}" }
                                span { class: "my-learning-card__dot", "·" }
                                span { "{form_count} {t(UiKey::HomeStatFormsLearning, lang.clone())}" }
                            }
                        }
                        Link {
                            to: Route::MyLearning {},
                            class: "btn btn--ghost btn--sm",
                            "{t(UiKey::MyLearningEdit, lang.clone())}"
                        }
                    }
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
                        onclick: move |_| {
                            *state.filter.write() = FilterParams::default();
                            *state.my_learning_active.write() = true;
                        },
                        "{t(UiKey::FiltersReset, lang.clone())}"
                    }
                }
            }
            if my_learning_empty {
                div { class: "home-tip",
                    p { "{t(UiKey::HomeTipFilter, lang.clone())}" }
                }
            }
        }
    }
}

// ── My Learning Container ─────────────────────────────────────────────────────

#[component]
fn MyLearningContainer() -> Element {
    let mut state = use_context::<AppState>();
    let lang = state.settings.read().language.clone();
    let morph_lang = state.settings.read().morph_language.clone();

    let mut add_panel_open = use_signal(|| false);
    let mut search = use_signal(|| String::new());
    // Lemma selected for verb configuration (before final add)
    let mut configuring_lemma: Signal<Option<i64>> = use_signal(|| None);
    let mut sel_tenses: Signal<Vec<String>> = use_signal(Vec::new);
    let mut sel_voices: Signal<Vec<String>> = use_signal(Vec::new);
    let mut sel_moods: Signal<Vec<String>> = use_signal(Vec::new);

    let my_learning = state.my_learning.read().clone();
    let my_learning_active = *state.my_learning_active.read();
    let lemmas_snap = state.lemmas.read().clone();

    // Filtered lemma search results — same logic as sidebar filter
    let q = search.read().clone();
    let q_norm = normalize(q.trim(), true).to_lowercase();
    let q_lower = q.to_lowercase();
    let mut shown_lemmas: Vec<_> = lemmas_snap.iter()
        .filter(|l| {
            if !state.lemma_has_paradigm(l.id) { return false; }
            if my_learning.iter().any(|item| item.lemma_id == l.id) { return false; }
            if q.trim().is_empty() { return true; }
            normalize(&l.greek, true).to_lowercase().contains(&q_norm)
                || l.russian.as_deref().unwrap_or("").to_lowercase().contains(&q_lower)
                || l.english.as_deref().unwrap_or("").to_lowercase().contains(&q_lower)
        })
        .collect();
    shown_lemmas.sort_by(|a, b| normalize(&a.greek, true).cmp(&normalize(&b.greek, true)));

    let is_verb_like = |pos: &str| matches!(pos, "verb" | "participle");

    rsx! {
        div { class: "my-learning",
            // Header row: title + active toggle
            div { class: "my-learning__header",
                h3 { class: "my-learning__title", "{t(UiKey::HomeMyLearning, lang.clone())}" }
                button {
                    class: if my_learning_active { "filter-chip filter-chip--active my-learning__toggle" } else { "filter-chip my-learning__toggle" },
                    onclick: move |_| {
                        let new_val = !*state.my_learning_active.read();
                        *state.my_learning_active.write() = new_val;
                    },
                    if my_learning_active { "✓" } else { "○" }
                }
            }

            if my_learning.is_empty() {
                p { class: "my-learning__desc", "{t(UiKey::HomeMyLearningDesc, lang.clone())}" }
            }

            // Current items list — grouped by lemma
            if !my_learning.is_empty() {
                {
                    // Collect unique lemma_ids in order of first appearance
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
                                    let group: Vec<_> = my_learning.iter()
                                        .filter(|i| i.lemma_id == lemma_id_g)
                                        .cloned()
                                        .collect();
                                    let lemma_greek = lemmas_snap.iter()
                                        .find(|l| l.id == lemma_id_g)
                                        .map(|l| l.greek.clone())
                                        .unwrap_or_default();
                                    let lemma_pos = lemmas_snap.iter()
                                        .find(|l| l.id == lemma_id_g)
                                        .and_then(|l| l.part_of_speech.clone())
                                        .unwrap_or_default();
                                    // Group if multiple items OR single item with tags
                                    let has_tags = group.iter().any(|i| !i.tenses.is_empty() || !i.voices.is_empty() || !i.moods.is_empty());
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
                                                            let tense_tags = sub_item.tenses.clone();
                                                            let voice_tags = sub_item.voices.clone();
                                                            let mood_tags  = sub_item.moods.clone();
                                                            let ml3 = ml2.clone();
                                                            let tags_text = {
                                                                let mut parts: Vec<String> = Vec::new();
                                                                if !tense_tags.is_empty() {
                                                                    parts.push(tense_tags.iter().map(|s| tense_label(s, &ml3)).collect::<Vec<_>>().join("/"));
                                                                }
                                                                if !voice_tags.is_empty() {
                                                                    parts.push(voice_tags.iter().map(|s| voice_label(s, &ml3)).collect::<Vec<_>>().join("/"));
                                                                }
                                                                if !mood_tags.is_empty() {
                                                                    parts.push(mood_tags.iter().map(|s| mood_label(s, &ml3)).collect::<Vec<_>>().join("/"));
                                                                }
                                                                if parts.is_empty() { t(UiKey::MyLearningAllForms, lang.clone()).to_string() } else { parts.join(" · ") }
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
                                        // Flat display for nouns / single-item no-tag entries
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

            // Add word button
            if !*add_panel_open.read() {
                button {
                    class: "btn btn--ghost btn--sm my-learning__add-btn",
                    onclick: move |_| {
                        *add_panel_open.write() = true;
                        *search.write() = String::new();
                        *configuring_lemma.write() = None;
                    },
                    "{t(UiKey::MyLearningAddWord, lang.clone())}"
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
                        },
                        autofocus: true,
                    }

                    // Lemma results (only shown when not configuring a verb)
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
                                                        // Show verb config
                                                        *configuring_lemma.write() = Some(lid);
                                                        *sel_tenses.write() = Vec::new();
                                                        *sel_voices.write() = Vec::new();
                                                        *sel_moods.write() = Vec::new();
                                                    } else {
                                                        // Add immediately for nouns/adj/etc.
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

                    // Verb configuration panel
                    if let Some(cfg_lemma_id) = *configuring_lemma.read() {
                        {
                            let cfg_lemma = lemmas_snap.iter().find(|l| l.id == cfg_lemma_id).cloned();
                            let forms_for_cfg = state.paradigm_forms_for_lemma(cfg_lemma_id);
                            // Collect unique tenses, voices, moods available for this lemma
                            let mut uniq_tenses: Vec<String> = forms_for_cfg.iter()
                                .filter_map(|f| f.tense_tag.clone())
                                .collect::<HashSet<_>>().into_iter().collect();
                            uniq_tenses.sort();
                            let mut uniq_voices: Vec<String> = forms_for_cfg.iter()
                                .filter_map(|f| f.voice_tag.clone())
                                .collect::<HashSet<_>>().into_iter().collect();
                            uniq_voices.sort();
                            let mut uniq_moods: Vec<String> = forms_for_cfg.iter()
                                .filter_map(|f| f.mood_tag.clone())
                                .collect::<HashSet<_>>().into_iter().collect();
                            uniq_moods.sort();
                            let cfg_greek = cfg_lemma.as_ref().map(|l| l.greek.clone()).unwrap_or_default();
                            let ml2 = morph_lang.clone();
                            let ml3 = morph_lang.clone();
                            let ml4 = morph_lang.clone();
                            rsx! {
                                div { class: "my-learning__verb-config",
                                    p { class: "my-learning__verb-config__heading greek-text", "{cfg_greek}" }

                                    // Tense chips
                                    if !uniq_tenses.is_empty() {
                                        div { class: "my-learning__verb-config__section",
                                            span { class: "my-learning__verb-config__label",
                                                "{t(UiKey::FiltersTense, lang.clone())}"
                                            }
                                            for tense in &uniq_tenses {
                                                {
                                                    let tv = tense.clone();
                                                    let label = tense_label(&tense, &ml2).to_string();
                                                    let active = sel_tenses.read().contains(&tv);
                                                    rsx! {
                                                        button {
                                                            class: if active { "filter-chip filter-chip--active" } else { "filter-chip" },
                                                            onclick: move |_| {
                                                                let mut t = sel_tenses.write();
                                                                if t.contains(&tv) { t.retain(|x| x != &tv); } else { t.push(tv.clone()); }
                                                            },
                                                            "{label}"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    // Voice chips
                                    if !uniq_voices.is_empty() {
                                        div { class: "my-learning__verb-config__section",
                                            span { class: "my-learning__verb-config__label",
                                                "{t(UiKey::FiltersVoice, lang.clone())}"
                                            }
                                            for voice in &uniq_voices {
                                                {
                                                    let vv = voice.clone();
                                                    let label = voice_label(&voice, &ml3).to_string();
                                                    let active = sel_voices.read().contains(&vv);
                                                    rsx! {
                                                        button {
                                                            class: if active { "filter-chip filter-chip--active" } else { "filter-chip" },
                                                            onclick: move |_| {
                                                                let mut v = sel_voices.write();
                                                                if v.contains(&vv) { v.retain(|x| x != &vv); } else { v.push(vv.clone()); }
                                                            },
                                                            "{label}"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    // Mood chips
                                    if !uniq_moods.is_empty() {
                                        div { class: "my-learning__verb-config__section",
                                            span { class: "my-learning__verb-config__label",
                                                "{t(UiKey::FiltersMood, lang.clone())}"
                                            }
                                            for mood in &uniq_moods {
                                                {
                                                    let mv = mood.clone();
                                                    let label = mood_label(&mood, &ml4).to_string();
                                                    let active = sel_moods.read().contains(&mv);
                                                    rsx! {
                                                        button {
                                                            class: if active { "filter-chip filter-chip--active" } else { "filter-chip" },
                                                            onclick: move |_| {
                                                                let mut m = sel_moods.write();
                                                                if m.contains(&mv) { m.retain(|x| x != &mv); } else { m.push(mv.clone()); }
                                                            },
                                                            "{label}"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    // Add + Cancel buttons
                                    div { class: "my-learning__verb-config__actions",
                                        button {
                                            class: "btn btn--primary btn--sm",
                                            onclick: move |_| {
                                                state.add_to_my_learning(MyLearningItem {
                                                    lemma_id: cfg_lemma_id,
                                                    tenses: sel_tenses.read().clone(),
                                                    voices: sel_voices.read().clone(),
                                                    moods: sel_moods.read().clone(),
                                                });
                                                *add_panel_open.write() = false;
                                                *search.write() = String::new();
                                                *configuring_lemma.write() = None;
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

                    // Close add panel button
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
