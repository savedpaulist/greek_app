use std::collections::HashMap;

use dioxus::prelude::*;

use crate::state::{
    app_state::{
        CustomFormEntry, CustomNominalParadigmDraft, CustomVerbFormEntry,
        CustomVerbParadigmDraft,
    },
    settings::{GreekFont, Theme, UiLanguage},
    AppState,
};

#[component]
pub fn SettingsPage() -> Element {
    rsx! {
        div { class: "settings-page",
            h2 { class: "settings-page__title", "Настройки" }
            SettingsPanel {}
        }
    }
}

#[component]
pub fn SettingsPanel() -> Element {
    let mut state = use_context::<AppState>();
    let mut lemma_greek = use_signal(String::new);
    let mut translation = use_signal(String::new);
    let mut paradigm_pos = use_signal(|| "noun".to_string());
    let mut paradigm_gender = use_signal(String::new);
    let mut paradigm_dual = use_signal(|| false);
    let mut verb_tenses = use_signal(|| vec!["pres".to_string()]);
    let mut verb_moods = use_signal(|| vec!["ind".to_string()]);
    let mut verb_voices = use_signal(|| vec!["act".to_string()]);
    let mut cell_values = use_signal(HashMap::<String, String>::new);
    let mut save_message = use_signal(|| None::<String>);

    let settings_snapshot = state.settings.read().clone();
    let custom_lemmas = state.custom_lemmas.read().clone();
    let is_verb = paradigm_pos.read().as_str() == "verb";
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
        div { class: "settings-panel",
            section { class: "settings-section",
                h3 { class: "settings-section__title", "Тема" }
                div { class: "theme-nav-row",
                    button {
                        class: "btn btn--ghost btn--sm",
                        title: "Предыдущая тема",
                        onclick: move |_| {
                            let mut settings = state.settings.write();
                            let all = Theme::all();
                            let idx = all.iter().position(|t| t == &settings.theme).unwrap_or(0);
                            let prev_idx = if idx == 0 { all.len() - 1 } else { idx - 1 };
                            settings.theme = all[prev_idx].clone();
                            drop(settings);
                            state.save_settings();
                        },
                        svg { xmlns: "http://www.w3.org/2000/svg", width: "16", height: "16", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                            polyline { points: "15 18 9 12 15 6" }
                        }
                    }
                    span { class: "theme-nav-row__label", "{settings_snapshot.theme.label()}" }
                    button {
                        class: "btn btn--ghost btn--sm",
                        title: "Следующая тема",
                        onclick: move |_| {
                            let mut settings = state.settings.write();
                            let all = Theme::all();
                            let idx = all.iter().position(|t| t == &settings.theme).unwrap_or(0);
                            let next_idx = (idx + 1) % all.len();
                            settings.theme = all[next_idx].clone();
                            drop(settings);
                            state.save_settings();
                        },
                        svg { xmlns: "http://www.w3.org/2000/svg", width: "16", height: "16", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                            polyline { points: "9 18 15 12 9 6" }
                        }
                    }
                }
                div { class: "theme-grid",
                    for theme in Theme::all() {
                        button {
                            class: if settings_snapshot.theme == *theme {
                                "theme-chip theme-chip--active"
                            } else {
                                "theme-chip"
                            },
                            "data-theme": theme.data_attr(),
                            onclick: {
                                let theme = theme.clone();
                                move |_| {
                                    state.settings.write().theme = theme.clone();
                                    state.save_settings();
                                }
                            },
                            "{theme.label()}"
                        }
                    }
                }
            }

            section { class: "settings-section",
                h3 { class: "settings-section__title", "Шрифт для греческого" }
                div { class: "font-list",
                    for font in GreekFont::all() {
                        button {
                            class: if settings_snapshot.greek_font == *font {
                                "font-chip font-chip--active"
                            } else {
                                "font-chip"
                            },
                            style: "font-family: {font.css_family()};",
                            onclick: {
                                let font = font.clone();
                                move |_| {
                                    state.settings.write().greek_font = font.clone();
                                    state.save_settings();
                                }
                            },
                            "αβγδεζ — {font.label()}"
                        }
                    }
                }
            }

            section { class: "settings-section",
                h3 { class: "settings-section__title", "Язык интерфейса" }
                div { class: "lang-row",
                    button {
                        class: if matches!(settings_snapshot.language, UiLanguage::Ru) { "btn btn--primary" } else { "btn btn--ghost" },
                        onclick: move |_| {
                            state.settings.write().language = UiLanguage::Ru;
                            state.save_settings();
                        },
                        "Русский"
                    }
                    button {
                        class: if matches!(settings_snapshot.language, UiLanguage::En) { "btn btn--primary" } else { "btn btn--ghost" },
                        onclick: move |_| {
                            state.settings.write().language = UiLanguage::En;
                            state.save_settings();
                        },
                        "English"
                    }
                }
            }

            section { class: "settings-section",
                h3 { class: "settings-section__title", "Опции" }
                label { class: "toggle-row",
                    input {
                        r#type: "checkbox",
                        checked: settings_snapshot.ignore_diacritics,
                        onchange: move |e| {
                            state.settings.write().ignore_diacritics = e.checked();
                            state.save_settings();
                        },
                    }
                    span { "Игнорировать диакритику при проверке" }
                }
                label { class: "toggle-row",
                    input {
                        r#type: "checkbox",
                        checked: settings_snapshot.show_transliteration,
                        onchange: move |e| {
                            state.settings.write().show_transliteration = e.checked();
                            state.save_settings();
                        },
                    }
                    span { "Показывать транслитерацию" }
                }
                label { class: "toggle-row",
                    input {
                        r#type: "checkbox",
                        checked: settings_snapshot.include_dual,
                        onchange: move |e| {
                            state.settings.write().include_dual = e.checked();
                            state.save_settings();
                        },
                    }
                    span { "Показывать двойственное число в таблицах" }
                }
            }

            section { class: "settings-section",
                h3 { class: "settings-section__title", "Своя парадигма" }
                p { class: "settings-help", "Созданные формы сохраняются в JSON и сразу подмешиваются в общие фильтры и проверки." }

                if !custom_lemmas.is_empty() {
                    div { class: "custom-paradigm-list",
                        for lemma in custom_lemmas {
                            div { class: "custom-paradigm-item",
                                div {
                                    strong { class: "greek-text", "{lemma.greek}" }
                                    if let Some(ru) = &lemma.russian {
                                        span { class: "custom-paradigm-item__translation", "«{ru}»" }
                                    }
                                }
                                button {
                                    class: "btn btn--ghost btn--sm",
                                    onclick: move |_| state.remove_custom_paradigm(lemma.id),
                                    "Удалить"
                                }
                            }
                        }
                    }
                }

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
                                    if next_pos == "verb" {
                                        *paradigm_gender.write() = String::new();
                                    }
                                    *paradigm_pos.write() = next_pos;
                                    cell_values.write().clear();
                                    *save_message.write() = None;
                                },
                                option { value: "noun", "Существительное" }
                                option { value: "verb", "Глагол" }
                                option { value: "adj", "Прилагательное" }
                                option { value: "pronoun", "Местоимение" }
                                option { value: "article", "Артикль" }
                            }
                        }
                        if !is_verb {
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
                                            let active = verb_tenses.read().iter().any(|item| item == value);
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
                                            let active = verb_moods.read().iter().any(|item| item == value);
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
                                            let active = verb_voices.read().iter().any(|item| item == value);
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
                    } else {
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
                            let active_numbers = if *paradigm_dual.read() {
                                vec![("sg", "Ед."), ("du", "Дв."), ("pl", "Мн.")]
                            } else {
                                vec![("sg", "Ед."), ("pl", "Мн.")]
                            };

                            if pos_value == "verb" {
                                let selected_tenses = verb_tenses.read().clone();
                                let selected_moods = verb_moods.read().clone();
                                let selected_voices = verb_voices.read().clone();
                                let mut entries = vec![];

                                for tense_tag in selected_tenses {
                                    for mood_tag in &selected_moods {
                                        for voice_tag in &selected_voices {
                                            for person_tag in ["1", "2", "3"] {
                                                for (number_tag, _number_label) in &active_numbers {
                                                    let key = verb_cell_key(&tense_tag, mood_tag, voice_tag, person_tag, number_tag);
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

                                let draft = CustomVerbParadigmDraft {
                                    lemma_greek: lemma_value,
                                    translation: Some(translation_value),
                                    entries,
                                };

                                match state.add_custom_verb_paradigm(draft) {
                                    Ok(()) => {
                                        *lemma_greek.write() = String::new();
                                        *translation.write() = String::new();
                                        *paradigm_dual.write() = false;
                                        *verb_tenses.write() = vec!["pres".into()];
                                        *verb_moods.write() = vec!["ind".into()];
                                        *verb_voices.write() = vec!["act".into()];
                                        cell_values.write().clear();
                                        *save_message.write() = Some("Глагольная парадигма сохранена и уже участвует в фильтрах и тренировках.".into());
                                    }
                                    Err(error) => {
                                        *save_message.write() = Some(error);
                                    }
                                }
                            } else {
                                let gender_value = paradigm_gender.read().clone();
                                let mut entries = vec![];

                                for (case_tag, _case_label) in [
                                    ("nom", "Им."),
                                    ("gen", "Род."),
                                    ("dat", "Дат."),
                                    ("acc", "Вин."),
                                    ("voc", "Зв."),
                                ] {
                                    for (number_tag, _number_label) in &active_numbers {
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

                                let draft = CustomNominalParadigmDraft {
                                    lemma_greek: lemma_value,
                                    translation: Some(translation_value),
                                    pos: pos_value,
                                    gender: if gender_value.is_empty() { None } else { Some(gender_value) },
                                    entries,
                                };

                                match state.add_custom_nominal_paradigm(draft) {
                                    Ok(()) => {
                                        *lemma_greek.write() = String::new();
                                        *translation.write() = String::new();
                                        *paradigm_gender.write() = String::new();
                                        *paradigm_dual.write() = false;
                                        cell_values.write().clear();
                                        *save_message.write() = Some("Парадигма сохранена и уже участвует в фильтрах и тренировках.".into());
                                    }
                                    Err(error) => {
                                        *save_message.write() = Some(error);
                                    }
                                }
                            }
                        },
                        "Сохранить парадигму"
                    }
                }
            }
        }
    }
}

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

fn tense_label_ru(tag: &str) -> &str {
    match tag {
        "pres" => "Наст.",
        "imperf" => "Импф.",
        "fut" => "Буд.",
        "aor1" => "Аор. I",
        "aor2" => "Аор. II",
        "perf" => "Перф.",
        _ => tag,
    }
}

fn mood_label_ru(tag: &str) -> &str {
    match tag {
        "ind" => "Изъяв.",
        "subj" => "Сослаг.",
        "opt" => "Желат.",
        "imp" => "Повел.",
        _ => tag,
    }
}

fn voice_label_ru(tag: &str) -> &str {
    match tag {
        "act" => "Акт.",
        "mid" => "Мед.",
        "pass" => "Пас.",
        "mid_pass" => "Мед./Пас.",
        _ => tag,
    }
}
