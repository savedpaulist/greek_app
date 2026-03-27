use std::collections::HashMap;

use dioxus::prelude::*;

use crate::models::{Form, Lemma};
use crate::router::Route;
use crate::state::{
    app_state::{
        CustomFormEntry, CustomNominalParadigmDraft, CustomParticipleFormEntry,
        CustomParticipleDraft, CustomVerbFormEntry, CustomVerbParadigmDraft,
    },
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

/// Rendered at `/paradigm-builder/:lemma_id` — pre-fills the form from an
/// existing custom paradigm.
#[component]
pub fn ParadigmBuilderEditPage(lemma_id: i64) -> Element {
    rsx! { ParadigmBuilderPage { edit_lemma_id: Some(lemma_id) } }
}

// ── Main builder page ─────────────────────────────────────────────────────────

#[component]
pub fn ParadigmBuilderPage(edit_lemma_id: Option<i64>) -> Element {
    let mut state = use_context::<AppState>();
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

    let number_columns: Vec<(&str, &str)> = if *paradigm_dual.read() {
        vec![("sg", "Ед."), ("du", "Дв."), ("pl", "Мн.")]
    } else {
        vec![("sg", "Ед."), ("pl", "Мн.")]
    };
    let cases = [
        ("nom", "Им."),
        ("gen", "Род."),
        ("dat", "Дат."),
        ("acc", "Вин."),
        ("voc", "Зв."),
    ];

    rsx! {
        div { class: "paradigm-builder-page",
            div { class: "paradigm-builder-page__header",
                Link { to: Route::Home {}, class: "btn btn--ghost btn--sm",
                    svg { xmlns: "http://www.w3.org/2000/svg", width: "16", height: "16", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                        polyline { points: "15 18 9 12 15 6" }
                    }
                    " Главная"
                }
                h2 { class: "paradigm-builder-page__title",
                    { if is_edit { "Редактировать парадигму" } else { "Создать парадигму" } }
                }
            }

            p { class: "settings-help",
                "Созданные формы сохраняются в JSON и сразу подмешиваются в общие фильтры и проверки."
            }

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
                                                " ✏ редактируется"
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
                                                "Изменить"
                                            }
                                        }
                                        button {
                                            class: "btn btn--ghost btn--sm",
                                            onclick: move |_| state.remove_custom_paradigm(lid),
                                            "Удалить"
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
                    placeholder: "Лемма",
                    value: "{lemma_greek.read()}",
                    oninput: move |e| *lemma_greek.write() = e.value(),
                }
                input {
                    class: "lemma-search-input",
                    r#type: "text",
                    placeholder: "Перевод / помета",
                    value: "{translation.read()}",
                    oninput: move |e| *translation.write() = e.value(),
                }

                div { class: "custom-paradigm-meta",
                    label { class: "custom-field",
                        span { "Часть речи" }
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
                            option { value: "noun", "Существительное" }
                            option { value: "verb", "Глагол" }
                            option { value: "adj", "Прилагательное" }
                            option { value: "pronoun", "Местоимение" }
                            option { value: "participle", "Причастие" }
                            option { value: "article", "Артикль" }
                        }
                    }
                    if !is_verb && !is_participle {
                        label { class: "custom-field",
                            span { "Род" }
                            select {
                                class: "custom-select",
                                value: "{paradigm_gender.read()}",
                                onchange: move |e| *paradigm_gender.write() = e.value(),
                                option { value: "", "Не указывать" }
                                option { value: "m", "Мужской" }
                                option { value: "f", "Женский" }
                                option { value: "n", "Средний" }
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
                    span { "Добавить двойственное число" }
                }

                // ── Verb controls ─────────────────────────────────────────────
                if is_verb {
                    div { class: "custom-verb-controls",
                        div { class: "custom-field",
                            span { "Времена" }
                            div { class: "custom-toggle-group",
                                for (value, label) in [
                                    ("pres", "Наст."),
                                    ("imperf", "Импф."),
                                    ("fut", "Буд."),
                                    ("aor1", "Аор. I"),
                                    ("aor2", "Аор. II"),
                                    ("perf", "Перф."),
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
                            span { "Наклонения" }
                            div { class: "custom-toggle-group",
                                for (value, label) in [
                                    ("ind", "Изъяв."),
                                    ("subj", "Сослаг."),
                                    ("opt", "Желат."),
                                    ("imp", "Повел."),
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
                            span { "Залоги" }
                            div { class: "custom-toggle-group",
                                for (value, label) in [
                                    ("act", "Акт."),
                                    ("mid", "Мед."),
                                    ("pass", "Пас."),
                                    ("mid_pass", "Мед./Пас."),
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
                            span { "Времена" }
                            div { class: "custom-toggle-group",
                                for (value, label) in [
                                    ("pres", "Наст."),
                                    ("aor1", "Аор. I"),
                                    ("aor2", "Аор. II"),
                                    ("perf", "Перф."),
                                    ("fut", "Буд."),
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
                            span { "Залоги" }
                            div { class: "custom-toggle-group",
                                for (value, label) in [
                                    ("act", "Акт."),
                                    ("mid", "Мед."),
                                    ("pass", "Пас."),
                                    ("mid_pass", "Мед./Пас."),
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
                            span { "Роды" }
                            div { class: "custom-toggle-group",
                                for (value, label) in [("m", "М"), ("f", "Ж"), ("n", "С")] {
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
                                        "{tense_label_ru(&tense)} · {mood_label_ru(&mood)} · {voice_label_ru(&voice)}"
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
                                                for (person_tag, person_label) in [("1", "1-е"), ("2", "2-е"), ("3", "3-е")] {
                                                    tr {
                                                        th { "{person_label}" }
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
                                    "{tense_label_ru(&tense)} · {voice_label_ru(&voice)}"
                                }
                                div { class: "custom-grid-wrapper",
                                    table { class: "custom-grid",
                                        thead {
                                            tr {
                                                th { "" }
                                                th { "" }
                                                for gender_tag in ptcp_genders.read().clone() {
                                                    th { "{gender_label_ru(&gender_tag)}" }
                                                }
                                            }
                                        }
                                        tbody {
                                            for (number_tag, number_label) in &number_columns {
                                                for (case_tag, case_label) in cases {
                                                    tr {
                                                        th { "{case_label}" }
                                                        th { "{number_label}" }
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
                                for (case_tag, case_label) in cases {
                                    tr {
                                        th { "{case_label}" }
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
                        let active_numbers: Vec<(&str, &str)> = if *paradigm_dual.read() {
                            vec![("sg", "Ед."), ("du", "Дв."), ("pl", "Мн.")]
                        } else {
                            vec![("sg", "Ед."), ("pl", "Мн.")]
                        };

                        // Remove the old version when editing
                        if let Some(lid) = edit_lemma_id {
                            state.remove_custom_paradigm(lid);
                        }

                        let result = if pos_value == "verb" {
                            let sel_tenses = verb_tenses.read().clone();
                            let sel_moods  = verb_moods.read().clone();
                            let sel_voices = verb_voices.read().clone();
                            let mut entries = vec![];
                            for tense_tag in &sel_tenses {
                                for mood_tag in &sel_moods {
                                    for voice_tag in &sel_voices {
                                        for person_tag in ["1", "2", "3"] {
                                            for (number_tag, _) in &active_numbers {
                                                let key = verb_cell_key(tense_tag, mood_tag, voice_tag, person_tag, number_tag);
                                                let value = cell_values.read().get(&key).cloned().unwrap_or_default();
                                                if !value.trim().is_empty() {
                                                    entries.push(CustomVerbFormEntry {
                                                        tense_tag: tense_tag.clone(),
                                                        mood_tag: mood_tag.clone(),
                                                        voice_tag: voice_tag.clone(),
                                                        person_tag: person_tag.to_string(),
                                                        number_tag: (*number_tag).to_string(),
                                                        greek_form: value,
                                                    });
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            state.add_custom_verb_paradigm(CustomVerbParadigmDraft {
                                lemma_greek: lemma_value,
                                translation: Some(translation_value),
                                entries,
                            })
                        } else if pos_value == "participle" {
                            let sel_tenses  = ptcp_tenses.read().clone();
                            let sel_voices  = ptcp_voices.read().clone();
                            let sel_genders = ptcp_genders.read().clone();
                            let mut entries = vec![];
                            for tense_tag in &sel_tenses {
                                for voice_tag in &sel_voices {
                                    for (case_tag, _) in [("nom",""),("gen",""),("dat",""),("acc",""),("voc","")] {
                                        for (number_tag, _) in &active_numbers {
                                            for gender_tag in &sel_genders {
                                                let key = format!("ptcp:{tense_tag}:{voice_tag}:{case_tag}:{number_tag}:{gender_tag}");
                                                let value = cell_values.read().get(&key).cloned().unwrap_or_default();
                                                if !value.trim().is_empty() {
                                                    entries.push(CustomParticipleFormEntry {
                                                        tense_tag: tense_tag.clone(),
                                                        voice_tag: voice_tag.clone(),
                                                        case_tag: case_tag.to_string(),
                                                        number_tag: (*number_tag).to_string(),
                                                        gender_tag: gender_tag.clone(),
                                                        greek_form: value,
                                                    });
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            state.add_custom_participle_paradigm(CustomParticipleDraft {
                                lemma_greek: lemma_value,
                                translation: Some(translation_value),
                                entries,
                            })
                        } else {
                            let gender_value = paradigm_gender.read().clone();
                            let mut entries = vec![];
                            for (case_tag, _) in [("nom",""),("gen",""),("dat",""),("acc",""),("voc","")] {
                                for (number_tag, _) in &active_numbers {
                                    let key = format!("{case_tag}:{number_tag}");
                                    let value = cell_values.read().get(&key).cloned().unwrap_or_default();
                                    if !value.trim().is_empty() {
                                        entries.push(CustomFormEntry {
                                            case_tag: case_tag.to_string(),
                                            number_tag: (*number_tag).to_string(),
                                            greek_form: value,
                                        });
                                    }
                                }
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
                                *save_message.write() = Some(
                                    if is_edit {
                                        "Парадигма обновлена.".into()
                                    } else {
                                        "Парадигма сохранена и уже участвует в фильтрах и тренировках.".into()
                                    }
                                );
                            }
                            Err(error) => {
                                *save_message.write() = Some(error);
                            }
                        }
                    },
                    { if is_edit { "Обновить парадигму" } else { "Сохранить парадигму" } }
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

fn gender_label_ru(tag: &str) -> &str {
    match tag {
        "m" => "М",
        "f" => "Ж",
        "n" => "С",
        _ => tag,
    }
}

fn tense_label_ru(tag: &str) -> &str {
    match tag {
        "pres"   => "Наст.",
        "imperf" => "Импф.",
        "fut"    => "Буд.",
        "aor1"   => "Аор. I",
        "aor2"   => "Аор. II",
        "perf"   => "Перф.",
        _        => tag,
    }
}

fn mood_label_ru(tag: &str) -> &str {
    match tag {
        "ind"  => "Изъяв.",
        "subj" => "Сослаг.",
        "opt"  => "Желат.",
        "imp"  => "Повел.",
        _      => tag,
    }
}

fn voice_label_ru(tag: &str) -> &str {
    match tag {
        "act"      => "Акт.",
        "mid"      => "Мед.",
        "pass"     => "Пас.",
        "mid_pass" => "Мед./Пас.",
        _          => tag,
    }
}
