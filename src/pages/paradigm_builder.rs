use std::collections::HashMap;

use dioxus::prelude::*;

use crate::i18n::{t, UiKey};
use crate::models::{Form, Lemma};
use crate::router::Route;
use crate::state::{
    app_state::{
        CustomFormEntry, CustomNominalParadigmDraft, CustomParticipleFormEntry,
        CustomParticipleDraft, CustomVerbFormEntry, CustomVerbParadigmDraft,
    },
    settings::UiLanguage,
    AppState,
};

// ── Init-state helpers ────────────────────────────────────────────────────────

struct InitState {
    greek: String,
    translation: String,
    pos: String,
    gender: String,
    dual: bool,
    verb_tenses: Vec<String>,
    verb_moods: Vec<String>,
    verb_voices: Vec<String>,
    ptcp_tenses: Vec<String>,
    ptcp_voices: Vec<String>,
    ptcp_genders: Vec<String>,
    cells: HashMap<String, String>,
}

fn default_init() -> InitState {
    InitState {
        greek: String::new(),
        translation: String::new(),
        pos: "noun".into(),
        gender: String::new(),
        dual: false,
        verb_tenses: vec!["pres".into()],
        verb_moods: vec!["ind".into()],
        verb_voices: vec!["act".into()],
        ptcp_tenses: vec!["pres".into()],
        ptcp_voices: vec!["act".into()],
        ptcp_genders: vec!["m".into(), "f".into(), "n".into()],
        cells: HashMap::new(),
    }
}

fn compute_init_state(
    edit_id: Option<i64>,
    custom_lemmas: &[Lemma],
    custom_forms: &[Form],
) -> InitState {
    let Some(lid) = edit_id else {
        return default_init();
    };
    let Some(lemma) = custom_lemmas.iter().find(|l| l.id == lid) else {
        return default_init();
    };
    let forms: Vec<&Form> = custom_forms.iter().filter(|f| f.lemma_id == lid).collect();
    let pos = lemma.part_of_speech.as_deref().unwrap_or("noun").to_string();
    let greek = lemma.greek.clone();
    let translation = lemma.russian.clone().unwrap_or_default();
    let dual = forms.iter().any(|f| f.number_tag.as_deref() == Some("du"));
    let mut cells: HashMap<String, String> = HashMap::new();

    match pos.as_str() {
        "verb" => {
            let mut tenses: Vec<String> = vec![];
            let mut moods: Vec<String> = vec![];
            let mut voices: Vec<String> = vec![];
            for f in &forms {
                let t = f.tense_tag.as_deref().unwrap_or("").to_string();
                let m = f.mood_tag.as_deref().unwrap_or("").to_string();
                let v = f.voice_tag.as_deref().unwrap_or("").to_string();
                let p = f.person_tag.as_deref().unwrap_or("").to_string();
                let n = f.number_tag.as_deref().unwrap_or("").to_string();
                if !t.is_empty() && !tenses.contains(&t) { tenses.push(t.clone()); }
                if !m.is_empty() && !moods.contains(&m) { moods.push(m.clone()); }
                if !v.is_empty() && !voices.contains(&v) { voices.push(v.clone()); }
                cells.insert(format!("verb:{t}:{m}:{v}:{p}:{n}"), f.greek_form.clone());
            }
            if tenses.is_empty() { tenses.push("pres".into()); }
            if moods.is_empty() { moods.push("ind".into()); }
            if voices.is_empty() { voices.push("act".into()); }
            InitState {
                greek, translation, pos, gender: String::new(), dual,
                verb_tenses: tenses, verb_moods: moods, verb_voices: voices,
                ptcp_tenses: vec!["pres".into()], ptcp_voices: vec!["act".into()],
                ptcp_genders: vec!["m".into(), "f".into(), "n".into()],
                cells,
            }
        }
        "participle" => {
            let mut ptenses: Vec<String> = vec![];
            let mut pvoices: Vec<String> = vec![];
            let mut pgenders: Vec<String> = vec![];
            for f in &forms {
                let t = f.tense_tag.as_deref().unwrap_or("").to_string();
                let v = f.voice_tag.as_deref().unwrap_or("").to_string();
                let c = f.case_tag.as_deref().unwrap_or("").to_string();
                let n = f.number_tag.as_deref().unwrap_or("").to_string();
                let g = f.gender_tag.as_deref().unwrap_or("").to_string();
                if !t.is_empty() && !ptenses.contains(&t) { ptenses.push(t.clone()); }
                if !v.is_empty() && !pvoices.contains(&v) { pvoices.push(v.clone()); }
                if !g.is_empty() && !pgenders.contains(&g) { pgenders.push(g.clone()); }
                cells.insert(format!("ptcp:{t}:{v}:{c}:{n}:{g}"), f.greek_form.clone());
            }
            if ptenses.is_empty() { ptenses.push("pres".into()); }
            if pvoices.is_empty() { pvoices.push("act".into()); }
            if pgenders.is_empty() { pgenders = vec!["m".into(), "f".into(), "n".into()]; }
            InitState {
                greek, translation, pos, gender: String::new(), dual,
                verb_tenses: vec!["pres".into()], verb_moods: vec!["ind".into()],
                verb_voices: vec!["act".into()],
                ptcp_tenses: ptenses, ptcp_voices: pvoices, ptcp_genders: pgenders,
                cells,
            }
        }
        _ => {
            let gender = forms.iter()
                .find_map(|f| f.gender_tag.as_deref())
                .unwrap_or("")
                .to_string();
            for f in &forms {
                let c = f.case_tag.as_deref().unwrap_or("");
                let n = f.number_tag.as_deref().unwrap_or("");
                cells.insert(format!("{c}:{n}"), f.greek_form.clone());
            }
            InitState {
                greek, translation, pos, gender, dual,
                verb_tenses: vec!["pres".into()], verb_moods: vec!["ind".into()],
                verb_voices: vec!["act".into()],
                ptcp_tenses: vec!["pres".into()], ptcp_voices: vec!["act".into()],
                ptcp_genders: vec!["m".into(), "f".into(), "n".into()],
                cells,
            }
        }
    }
}

// ── Route wrapper for editing ─────────────────────────────────────────────────

#[component]
pub fn ParadigmBuilderEditPage(lemma_id: i64) -> Element {
    rsx! { ParadigmBuilderPage { edit_lemma_id: Some(lemma_id) } }
}

// ── Main builder page ─────────────────────────────────────────────────────────

#[component]
pub fn ParadigmBuilderPage(edit_lemma_id: Option<i64>) -> Element {
    let mut state = use_context::<AppState>();
    let settings_snap = state.settings.read().clone();
    let lang = settings_snap.language.clone();
    let morph_lang = settings_snap.morph_language.clone();
    let is_edit = edit_lemma_id.is_some();

    let init = {
        let cl = state.custom_lemmas.read();
        let cf = state.custom_forms.read();
        compute_init_state(edit_lemma_id, &cl, &cf)
    };

    let InitState {
        greek: init_greek,
        translation: init_translation,
        pos: init_pos,
        gender: init_gender,
        dual: init_dual,
        verb_tenses: init_verb_tenses,
        verb_moods: init_verb_moods,
        verb_voices: init_verb_voices,
        ptcp_tenses: init_ptcp_tenses,
        ptcp_voices: init_ptcp_voices,
        ptcp_genders: init_ptcp_genders,
        cells: init_cells,
    } = init;

    let mut lemma_greek    = use_signal(move || init_greek);
    let mut translation    = use_signal(move || init_translation);
    let mut paradigm_pos   = use_signal(move || init_pos);
    let mut paradigm_gender= use_signal(move || init_gender);
    let mut paradigm_dual  = use_signal(move || init_dual);
    let mut verb_tenses    = use_signal(move || init_verb_tenses);
    let mut verb_moods     = use_signal(move || init_verb_moods);
    let mut verb_voices    = use_signal(move || init_verb_voices);
    let mut ptcp_tenses    = use_signal(move || init_ptcp_tenses);
    let mut ptcp_voices    = use_signal(move || init_ptcp_voices);
    let mut ptcp_genders   = use_signal(move || init_ptcp_genders);
    let mut cell_values    = use_signal(move || init_cells);
    let mut save_message   = use_signal(|| Option::<String>::None);

    let custom_lemmas  = state.custom_lemmas.read().clone();
    let is_verb        = paradigm_pos.read().as_str() == "verb";
    let is_participle  = paradigm_pos.read().as_str() == "participle";

    let sg_label = num_label("sg", &morph_lang);
    let du_label = num_label("du", &morph_lang);
    let pl_label = num_label("pl", &morph_lang);
    let number_columns: Vec<(&str, &str)> = if *paradigm_dual.read() {
        vec![("sg", sg_label), ("du", du_label), ("pl", pl_label)]
    } else {
        vec![("sg", sg_label), ("pl", pl_label)]
    };

    let cases = [
        ("nom", case_label("nom", &morph_lang)),
        ("gen", case_label("gen", &morph_lang)),
        ("dat", case_label("dat", &morph_lang)),
        ("acc", case_label("acc", &morph_lang)),
        ("voc", case_label("voc", &morph_lang)),
    ];

    rsx! {
        div { class: "paradigm-builder-page",
            div { class: "paradigm-builder-page__header",
                Link { to: Route::Home {}, class: "btn btn--ghost btn--sm",
                    svg { xmlns: "http://www.w3.org/2000/svg", width: "16", height: "16", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                        polyline { points: "15 18 9 12 15 6" }
                    }
                    " {t(UiKey::TopbarHome, lang.clone())}"
                }
                h2 { class: "paradigm-builder-page__title",
                    { if is_edit { t(UiKey::BuilderEdit, lang.clone()) } else { t(UiKey::BuilderTitle, lang.clone()) } }
                }
            }

            p { class: "settings-help", "{t(UiKey::BuilderHint, lang.clone())}" }

            // ── Existing custom paradigms list ───────────────────────────────
            if !custom_lemmas.is_empty() {
                div { class: "custom-paradigm-list",
                    for lemma in &custom_lemmas {
                        {
                            let lid = lemma.id;
                            let is_being_edited = edit_lemma_id == Some(lid);
                            let greek = lemma.greek.clone();
                            let russian = lemma.russian.clone();
                            rsx! {
                                div {
                                    class: if is_being_edited {
                                        "custom-paradigm-item custom-paradigm-item--editing"
                                    } else {
                                        "custom-paradigm-item"
                                    },
                                    div {
                                        strong { class: "greek-text", "{greek}" }
                                        if is_being_edited {
                                            span { class: "custom-paradigm-item__editing-badge",
                                                "{t(UiKey::BuilderEditingBadge, lang.clone())}"
                                            }
                                        }
                                        if let Some(ru) = russian {
                                            span { class: "custom-paradigm-item__translation",
                                                " «{ru}»"
                                            }
                                        }
                                    }
                                    div { class: "custom-paradigm-item__actions",
                                        if !is_being_edited {
                                            Link {
                                                to: Route::ParadigmBuilderEdit { lemma_id: lid },
                                                class: "btn btn--ghost btn--sm",
                                                "{t(UiKey::BuilderEditBtn, lang.clone())}"
                                            }
                                        }
                                        button {
                                            class: "btn btn--ghost btn--sm",
                                            onclick: move |_| state.remove_custom_paradigm(lid),
                                            "{t(UiKey::Delete, lang.clone())}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // ── Form ─────────────────────────────────────────────────────────
            div { class: "custom-paradigm-form",
                input {
                    class: "lemma-search-input greek-text",
                    r#type: "text",
                    placeholder: t(UiKey::BuilderLemmaField, lang.clone()),
                    value: "{lemma_greek.read()}",
                    oninput: move |e| *lemma_greek.write() = e.value(),
                }
                input {
                    class: "lemma-search-input",
                    r#type: "text",
                    placeholder: t(UiKey::BuilderTranslationField, lang.clone()),
                    value: "{translation.read()}",
                    oninput: move |e| *translation.write() = e.value(),
                }

                div { class: "custom-paradigm-meta",
                    label { class: "custom-field",
                        span { "{t(UiKey::BuilderPosLabel, lang.clone())}" }
                        select {
                            class: "custom-select",
                            value: "{paradigm_pos.read()}",
                            onchange: move |e| {
                                let next_pos = e.value();
                                *paradigm_gender.write() = String::new();
                                *paradigm_pos.write() = next_pos;
                                cell_values.write().clear();
                                *save_message.write() = None;
                            },
                            option { value: "noun",      "{t(UiKey::FilterPosNoun,    morph_lang.clone())}" }
                            option { value: "verb",      "{t(UiKey::FilterPosVerb,    morph_lang.clone())}" }
                            option { value: "adj",       "{t(UiKey::FilterPosAdj,     morph_lang.clone())}" }
                            option { value: "pronoun",   "{t(UiKey::FilterPosPronoun, morph_lang.clone())}" }
                            option { value: "participle","{t(UiKey::FilterPosPart,    morph_lang.clone())}" }
                            option { value: "article",   "{t(UiKey::FilterPosArticle, morph_lang.clone())}" }
                        }
                    }
                    if !is_verb && !is_participle {
                        label { class: "custom-field",
                            span { "{t(UiKey::BuilderGenderLabel, lang.clone())}" }
                            select {
                                class: "custom-select",
                                value: "{paradigm_gender.read()}",
                                onchange: move |e| *paradigm_gender.write() = e.value(),
                                option { value: "", "{t(UiKey::BuilderGenderNone, lang.clone())}" }
                                option { value: "m", "{t(UiKey::BuilderGenderM, morph_lang.clone())}" }
                                option { value: "f", "{t(UiKey::BuilderGenderF, morph_lang.clone())}" }
                                option { value: "n", "{t(UiKey::BuilderGenderN, morph_lang.clone())}" }
                            }
                        }
                    }
                }

                label { class: "toggle-row",
                    input {
                        r#type: "checkbox",
                        checked: *paradigm_dual.read(),
                        onchange: move |e| *paradigm_dual.write() = e.checked(),
                    }
                    span { "{t(UiKey::BuilderDualLabel, lang.clone())}" }
                }

                // ── Verb controls ─────────────────────────────────────────────
                if is_verb {
                    div { class: "custom-verb-controls",
                        div { class: "custom-field",
                            span { "{t(UiKey::BuilderTensesLabel, lang.clone())}" }
                            div { class: "custom-toggle-group",
                                for (value, label) in [
                                    ("pres",   tense_label("pres",   &morph_lang)),
                                    ("imperf",  tense_label("imperf", &morph_lang)),
                                    ("fut",     tense_label("fut",    &morph_lang)),
                                    ("aor1",    tense_label("aor1",   &morph_lang)),
                                    ("aor2",    tense_label("aor2",   &morph_lang)),
                                    ("perf",    tense_label("perf",   &morph_lang)),
                                ] {
                                    {
                                        let active = verb_tenses.read().iter().any(|i| i == value);
                                        let value = value.to_string();
                                        rsx! {
                                            button {
                                                class: if active { "chip chip--active chip--sm" } else { "chip chip--sm" },
                                                onclick: move |_| toggle_required_option(&mut verb_tenses, &value),
                                                "{label}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        div { class: "custom-field",
                            span { "{t(UiKey::BuilderMoodsLabel, lang.clone())}" }
                            div { class: "custom-toggle-group",
                                for (value, label) in [
                                    ("ind",  mood_label("ind",  &morph_lang)),
                                    ("subj", mood_label("subj", &morph_lang)),
                                    ("opt",  mood_label("opt",  &morph_lang)),
                                    ("imp",  mood_label("imp",  &morph_lang)),
                                ] {
                                    {
                                        let active = verb_moods.read().iter().any(|i| i == value);
                                        let value = value.to_string();
                                        rsx! {
                                            button {
                                                class: if active { "chip chip--active chip--sm" } else { "chip chip--sm" },
                                                onclick: move |_| toggle_required_option(&mut verb_moods, &value),
                                                "{label}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        div { class: "custom-field",
                            span { "{t(UiKey::BuilderVoicesLabel, lang.clone())}" }
                            div { class: "custom-toggle-group",
                                for (value, label) in [
                                    ("act",      voice_label("act",      &morph_lang)),
                                    ("mid",      voice_label("mid",      &morph_lang)),
                                    ("pass",     voice_label("pass",     &morph_lang)),
                                    ("mid_pass", voice_label("mid_pass", &morph_lang)),
                                ] {
                                    {
                                        let active = verb_voices.read().iter().any(|i| i == value);
                                        let value = value.to_string();
                                        rsx! {
                                            button {
                                                class: if active { "chip chip--active chip--sm" } else { "chip chip--sm" },
                                                onclick: move |_| toggle_required_option(&mut verb_voices, &value),
                                                "{label}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // ── Participle controls ───────────────────────────────────────
                if is_participle {
                    div { class: "custom-verb-controls",
                        div { class: "custom-field",
                            span { "{t(UiKey::BuilderTensesLabel, lang.clone())}" }
                            div { class: "custom-toggle-group",
                                for (value, label) in [
                                    ("pres", tense_label("pres", &morph_lang)),
                                    ("aor1", tense_label("aor1", &morph_lang)),
                                    ("aor2", tense_label("aor2", &morph_lang)),
                                    ("perf", tense_label("perf", &morph_lang)),
                                    ("fut",  tense_label("fut",  &morph_lang)),
                                ] {
                                    {
                                        let active = ptcp_tenses.read().iter().any(|i| i == value);
                                        let value = value.to_string();
                                        rsx! {
                                            button {
                                                class: if active { "chip chip--active chip--sm" } else { "chip chip--sm" },
                                                onclick: move |_| toggle_required_option(&mut ptcp_tenses, &value),
                                                "{label}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        div { class: "custom-field",
                            span { "{t(UiKey::BuilderVoicesLabel, lang.clone())}" }
                            div { class: "custom-toggle-group",
                                for (value, label) in [
                                    ("act",      voice_label("act",      &morph_lang)),
                                    ("mid",      voice_label("mid",      &morph_lang)),
                                    ("pass",     voice_label("pass",     &morph_lang)),
                                    ("mid_pass", voice_label("mid_pass", &morph_lang)),
                                ] {
                                    {
                                        let active = ptcp_voices.read().iter().any(|i| i == value);
                                        let value = value.to_string();
                                        rsx! {
                                            button {
                                                class: if active { "chip chip--active chip--sm" } else { "chip chip--sm" },
                                                onclick: move |_| toggle_required_option(&mut ptcp_voices, &value),
                                                "{label}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        div { class: "custom-field",
                            span { "{t(UiKey::BuilderGendersLabel, lang.clone())}" }
                            div { class: "custom-toggle-group",
                                for (value, label) in [
                                    ("m", gender_label("m", &morph_lang)),
                                    ("f", gender_label("f", &morph_lang)),
                                    ("n", gender_label("n", &morph_lang)),
                                ] {
                                    {
                                        let active = ptcp_genders.read().iter().any(|i| i == value);
                                        let value = value.to_string();
                                        rsx! {
                                            button {
                                                class: if active { "chip chip--active chip--sm" } else { "chip chip--sm" },
                                                onclick: move |_| toggle_required_option(&mut ptcp_genders, &value),
                                                "{label}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // ── Verb grids ────────────────────────────────────────────────
                if is_verb {
                    for tense in verb_tenses.read().clone() {
                        for mood in verb_moods.read().clone() {
                            for voice in verb_voices.read().clone() {
                                div { class: "custom-grid-block",
                                    div { class: "custom-grid-block__title",
                                        "{tense_label(&tense, &morph_lang)} · {mood_label(&mood, &morph_lang)} · {voice_label(&voice, &morph_lang)}"
                                    }
                                    div { class: "custom-grid-wrapper",
                                        table { class: "custom-grid",
                                            thead {
                                                tr {
                                                    th { "" }
                                                    for (_value, label) in &number_columns {
                                                        th { "{label}" }
                                                    }
                                                }
                                            }
                                            tbody {
                                                for (person_tag, person_lbl) in [
                                                    ("1", person_label("1", &morph_lang)),
                                                    ("2", person_label("2", &morph_lang)),
                                                    ("3", person_label("3", &morph_lang)),
                                                ] {
                                                    tr {
                                                        th { "{person_lbl}" }
                                                        for (number_tag, _label) in &number_columns {
                                                            {
                                                                let key = verb_cell_key(&tense, &mood, &voice, person_tag, number_tag);
                                                                let value = cell_values.read().get(&key).cloned().unwrap_or_default();
                                                                rsx! {
                                                                    td {
                                                                        input {
                                                                            class: "custom-cell-input greek-text",
                                                                            r#type: "text",
                                                                            value: "{value}",
                                                                            oninput: move |e| {
                                                                                cell_values.write().insert(key.clone(), e.value());
                                                                            },
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
                    }
                }

                // ── Participle grids (one per tense × voice) ─────────────────
                if is_participle {
                    for tense in ptcp_tenses.read().clone() {
                        for voice in ptcp_voices.read().clone() {
                            div { class: "custom-grid-block",
                                div { class: "custom-grid-block__title",
                                    "{tense_label(&tense, &morph_lang)} · {voice_label(&voice, &morph_lang)}"
                                }
                                div { class: "custom-grid-wrapper",
                                    table { class: "custom-grid",
                                        thead {
                                            tr {
                                                th { "" }
                                                th { "" }
                                                for gender_tag in ptcp_genders.read().clone() {
                                                    th { "{gender_label(&gender_tag, &morph_lang)}" }
                                                }
                                            }
                                        }
                                        tbody {
                                            for (number_tag, number_lbl) in &number_columns {
                                                for (case_tag, case_lbl) in cases {
                                                    tr {
                                                        th { "{case_lbl}" }
                                                        th { "{number_lbl}" }
                                                        for gender_tag in ptcp_genders.read().clone() {
                                                            {
                                                                let key = format!("ptcp:{tense}:{voice}:{case_tag}:{number_tag}:{gender_tag}");
                                                                let value = cell_values.read().get(&key).cloned().unwrap_or_default();
                                                                rsx! {
                                                                    td {
                                                                        input {
                                                                            class: "custom-cell-input greek-text",
                                                                            r#type: "text",
                                                                            value: "{value}",
                                                                            oninput: move |e| {
                                                                                cell_values.write().insert(key.clone(), e.value());
                                                                            },
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
                    }
                }

                // ── Nominal grid ──────────────────────────────────────────────
                if !is_verb && !is_participle {
                    div { class: "custom-grid-wrapper",
                        table { class: "custom-grid",
                            thead {
                                tr {
                                    th { "" }
                                    for (_value, label) in &number_columns {
                                        th { "{label}" }
                                    }
                                }
                            }
                            tbody {
                                for (case_tag, case_lbl) in cases {
                                    tr {
                                        th { "{case_lbl}" }
                                        for (number_tag, _label) in &number_columns {
                                            {
                                                let key = format!("{case_tag}:{number_tag}");
                                                let value = cell_values.read().get(&key).cloned().unwrap_or_default();
                                                rsx! {
                                                    td {
                                                        input {
                                                            class: "custom-cell-input greek-text",
                                                            r#type: "text",
                                                            value: "{value}",
                                                            oninput: move |e| {
                                                                cell_values.write().insert(key.clone(), e.value());
                                                            },
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

                if let Some(message) = &*save_message.read() {
                    p { class: "settings-help settings-help--status", "{message}" }
                }

                button {
                    class: "btn btn--primary",
                    onclick: move |_| {
                        let lemma_value = lemma_greek.read().clone();
                        let translation_value = translation.read().clone();
                        let pos_value = paradigm_pos.read().clone();
                        if let Some(lid) = edit_lemma_id {
                            state.remove_custom_paradigm(lid);
                        }

                        // Collect all non-empty cells from cell_values directly.
                        // This preserves data for tenses/moods/voices/cases that the
                        // user may not have selected in the current editing session
                        // (smart merge: existing cells are loaded at init and kept).
                        let result = if pos_value == "verb" {
                            let mut entries = vec![];
                            for (key, value) in cell_values.read().iter() {
                                if value.trim().is_empty() { continue; }
                                // key format: "verb:{tense}:{mood}:{voice}:{person}:{number}"
                                let parts: Vec<&str> = key.splitn(6, ':').collect();
                                if parts.len() != 6 || parts[0] != "verb" { continue; }
                                entries.push(CustomVerbFormEntry {
                                    tense_tag:  parts[1].to_string(),
                                    mood_tag:   parts[2].to_string(),
                                    voice_tag:  parts[3].to_string(),
                                    person_tag: parts[4].to_string(),
                                    number_tag: parts[5].to_string(),
                                    greek_form: value.clone(),
                                });
                            }
                            state.add_custom_verb_paradigm(CustomVerbParadigmDraft {
                                lemma_greek: lemma_value,
                                translation: Some(translation_value),
                                entries,
                            })
                        } else if pos_value == "participle" {
                            let mut entries = vec![];
                            for (key, value) in cell_values.read().iter() {
                                if value.trim().is_empty() { continue; }
                                // key format: "ptcp:{tense}:{voice}:{case}:{number}:{gender}"
                                let parts: Vec<&str> = key.splitn(6, ':').collect();
                                if parts.len() != 6 || parts[0] != "ptcp" { continue; }
                                entries.push(CustomParticipleFormEntry {
                                    tense_tag:  parts[1].to_string(),
                                    voice_tag:  parts[2].to_string(),
                                    case_tag:   parts[3].to_string(),
                                    number_tag: parts[4].to_string(),
                                    gender_tag: parts[5].to_string(),
                                    greek_form: value.clone(),
                                });
                            }
                            state.add_custom_participle_paradigm(CustomParticipleDraft {
                                lemma_greek: lemma_value,
                                translation: Some(translation_value),
                                entries,
                            })
                        } else {
                            let gender_value = paradigm_gender.read().clone();
                            let mut entries = vec![];
                            for (key, value) in cell_values.read().iter() {
                                if value.trim().is_empty() { continue; }
                                // key format: "{case}:{number}" (no prefix)
                                if key.starts_with("verb:") || key.starts_with("ptcp:") { continue; }
                                let mut parts = key.splitn(2, ':');
                                let case_tag   = parts.next().unwrap_or("").to_string();
                                let number_tag = parts.next().unwrap_or("").to_string();
                                if case_tag.is_empty() || number_tag.is_empty() { continue; }
                                entries.push(CustomFormEntry { case_tag, number_tag, greek_form: value.clone() });
                            }
                            state.add_custom_nominal_paradigm(CustomNominalParadigmDraft {
                                lemma_greek: lemma_value,
                                translation: Some(translation_value),
                                pos: pos_value,
                                gender: if gender_value.is_empty() { None } else { Some(gender_value) },
                                entries,
                            })
                        };

                        match result {
                            Ok(()) => {
                                // In create mode, reset the form for a new entry.
                                // In edit mode, keep the form populated so the user
                                // can see the saved state and continue editing.
                                if !is_edit {
                                    *lemma_greek.write()     = String::new();
                                    *translation.write()     = String::new();
                                    *paradigm_gender.write() = String::new();
                                    *paradigm_dual.write()   = false;
                                    *verb_tenses.write()     = vec!["pres".into()];
                                    *verb_moods.write()      = vec!["ind".into()];
                                    *verb_voices.write()     = vec!["act".into()];
                                    *ptcp_tenses.write()     = vec!["pres".into()];
                                    *ptcp_voices.write()     = vec!["act".into()];
                                    *ptcp_genders.write()    = vec!["m".into(), "f".into(), "n".into()];
                                    cell_values.write().clear();
                                }
                                *save_message.write() = Some(
                                    if is_edit {
                                        t(UiKey::BuilderUpdated, lang.clone()).to_string()
                                    } else {
                                        t(UiKey::BuilderSavedNominal, lang.clone()).to_string()
                                    }
                                );
                            }
                            Err(error) => {
                                *save_message.write() = Some(error);
                            }
                        }
                    },
                    { if is_edit { t(UiKey::BuilderUpdateBtn, lang.clone()) } else { t(UiKey::BuilderSaveBtn, lang.clone()) } }
                }
            }
        }
    }
}

// ── Utilities ─────────────────────────────────────────────────────────────────

fn toggle_required_option(signal: &mut Signal<Vec<String>>, value: &str) {
    let mut selected = signal.write();
    if let Some(index) = selected.iter().position(|item| item == value) {
        if selected.len() > 1 {
            selected.remove(index);
        }
    } else {
        selected.push(value.to_string());
    }
}

fn verb_cell_key(tense: &str, mood: &str, voice: &str, person: &str, number: &str) -> String {
    format!("verb:{tense}:{mood}:{voice}:{person}:{number}")
}

fn tense_label<'a>(tag: &'a str, lang: &UiLanguage) -> &'static str {
    match lang {
        UiLanguage::En => match tag {
            "pres" => "Pres", "imperf" => "Impf", "fut" => "Fut",
            "aor1" => "Aor I", "aor2" => "Aor II", "perf" => "Perf", _ => "?",
        },
        UiLanguage::Ru => match tag {
            "pres" => "Наст.", "imperf" => "Импф.", "fut" => "Буд.",
            "aor1" => "Аор. I", "aor2" => "Аор. II", "perf" => "Перф.", _ => "?",
        },
    }
}

fn mood_label<'a>(tag: &'a str, lang: &UiLanguage) -> &'static str {
    match lang {
        UiLanguage::En => match tag {
            "ind" => "Ind", "subj" => "Subj", "opt" => "Opt", "imp" => "Imp", _ => "?",
        },
        UiLanguage::Ru => match tag {
            "ind" => "Изъяв.", "subj" => "Сослаг.", "opt" => "Желат.", "imp" => "Повел.", _ => "?",
        },
    }
}

fn voice_label<'a>(tag: &'a str, lang: &UiLanguage) -> &'static str {
    match lang {
        UiLanguage::En => match tag {
            "act" => "Act", "mid" => "Mid", "pass" => "Pass", "mid_pass" => "Mid/Pass", _ => "?",
        },
        UiLanguage::Ru => match tag {
            "act" => "Акт.", "mid" => "Мед.", "pass" => "Пас.", "mid_pass" => "Мед./Пас.", _ => "?",
        },
    }
}

fn gender_label<'a>(tag: &'a str, lang: &UiLanguage) -> &'static str {
    match lang {
        UiLanguage::En => match tag { "m" => "M", "f" => "F", "n" => "N", _ => "?" },
        UiLanguage::Ru => match tag { "m" => "М", "f" => "Ж", "n" => "С", _ => "?" },
    }
}

fn case_label<'a>(tag: &'a str, lang: &UiLanguage) -> &'static str {
    match lang {
        UiLanguage::En => match tag {
            "nom" => "Nom", "gen" => "Gen", "dat" => "Dat", "acc" => "Acc", "voc" => "Voc", _ => "?",
        },
        UiLanguage::Ru => match tag {
            "nom" => "Им.", "gen" => "Род.", "dat" => "Дат.", "acc" => "Вин.", "voc" => "Зв.", _ => "?",
        },
    }
}

fn num_label<'a>(tag: &'a str, lang: &UiLanguage) -> &'static str {
    match lang {
        UiLanguage::En => match tag { "sg" => "Sg", "du" => "Du", "pl" => "Pl", _ => "?" },
        UiLanguage::Ru => match tag { "sg" => "Ед.", "du" => "Дв.", "pl" => "Мн.", _ => "?" },
    }
}

fn person_label<'a>(tag: &'a str, lang: &UiLanguage) -> &'static str {
    match lang {
        UiLanguage::En => match tag { "1" => "1st", "2" => "2nd", "3" => "3rd", _ => "?" },
        UiLanguage::Ru => match tag { "1" => "1-е", "2" => "2-е", "3" => "3-е", _ => "?" },
    }
}
