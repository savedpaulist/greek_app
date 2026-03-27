use dioxus::prelude::*;

use crate::logic::diacritics::normalize;
use crate::models::FilterParams;
use crate::pages::settings::SettingsPanel;
use crate::state::AppState;

fn is_overlay_layout() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window()
            .and_then(|w| w.inner_width().ok())
            .and_then(|v| v.as_f64())
            .map(|w| w < 1400.0)
            .unwrap_or(true)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        false
    }
}

/// Slide-in sidebars for filters and settings.
#[component]
pub fn Sidebar() -> Element {
    let mut state = use_context::<AppState>();
    let filters_open = *state.filters_open.read();
    let settings_open = *state.settings_open.read();
    let filters_class = if filters_open { "sidebar sidebar--filters sidebar--open" } else { "sidebar sidebar--filters" };
    let settings_class = if settings_open { "sidebar sidebar--right sidebar--settings sidebar--open" } else { "sidebar sidebar--right sidebar--settings" };
    let overlay_layout = is_overlay_layout();

    rsx! {
        // Transparent shield on desktop, dark backdrop on mobile — both intercept clicks
        // so tapping outside the sidebar closes it without triggering page content.
        if filters_open || settings_open {
            div {
                class: if overlay_layout { "sidebar-backdrop" } else { "sidebar-backdrop sidebar-backdrop--ghost" },
                onclick: move |_| {
                    *state.filters_open.write() = false;
                    *state.settings_open.write() = false;
                },
            }
        }
        aside {
            class: "{filters_class}",
            div { class: "sidebar__header",
                span { class: "sidebar__title",
                    svg { xmlns: "http://www.w3.org/2000/svg", width: "18", height: "18", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                        circle { cx: "11", cy: "11", r: "8" }
                        line { x1: "21", y1: "21", x2: "16.65", y2: "16.65" }
                    }
                    " Фильтры"
                }
            }
            div { class: "sidebar__body",
                FilterPanel {}
                LemmaFilterPanel {}
            }
        }
        aside {
            class: "{settings_class}",
            div { class: "sidebar__header",
                span { class: "sidebar__title",
                    svg { xmlns: "http://www.w3.org/2000/svg", width: "18", height: "18", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                        circle { cx: "12", cy: "12", r: "3" }
                        path { d: "M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z" }
                    }
                    " Настройки"
                }
                button {
                    class: "sidebar__close",
                    onclick: move |_| *state.settings_open.write() = false,
                    svg { xmlns: "http://www.w3.org/2000/svg", width: "18", height: "18", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                        line { x1: "18", y1: "6", x2: "6", y2: "18" }
                        line { x1: "6", y1: "6", x2: "18", y2: "18" }
                    }
                }
            }
            div { class: "sidebar__body",
                SettingsPanel {}
            }
        }
    }
}

// ── Filter panel ────────────────────────────────────────────────────────────

#[component]
fn FilterPanel() -> Element {
    let mut state = use_context::<AppState>();
    let filter_snapshot = state.filter.read().clone();
    let selected_pos = filter_snapshot.pos.clone();
    let show_verbal = selected_pos.iter().any(|p| p == "verb");
    let show_nominal = selected_pos.iter().any(|p| matches!(p.as_str(), "noun" | "adj" | "pronoun" | "article"));

    rsx! {
        section { class: "sidebar__section",
            h3 { class: "sidebar__section-title", "Часть речи" }
            div { class: "filter-chips",
                for (value , label) in [
                    ("noun", "Сущ."),
                    ("verb", "Глагол"),
                    ("participle", "Прич."),
                    ("adj", "Прил."),
                    ("pronoun", "Мест."),
                    ("article", "Артикль"),
                    ("num", "Числ."),
                ]
                {
                    PosChip { value: value.to_string(), label: label.to_string() }
                }
            }

            // Tense chips (shown when verb is selected)
            if show_verbal {
                h4 { class: "sidebar__section-subtitle", "Время" }
                div { class: "filter-chips filter-chips--sm",
                    for (value, label) in [
                        ("pres", "Наст."), ("imperf", "Импф."), ("fut", "Буд."),
                        ("aor1", "Аор. I"), ("aor2", "Аор. II"), ("perf", "Перф."),
                    ] {
                        TagChip { field: "tense".to_string(), value: value.to_string(), label: label.to_string() }
                    }
                }
                h4 { class: "sidebar__section-subtitle", "Лицо" }
                div { class: "filter-chips filter-chips--sm",
                    for (value, label) in [("1", "1л."), ("2", "2л."), ("3", "3л.")] {
                        TagChip { field: "person".to_string(), value: value.to_string(), label: label.to_string() }
                    }
                }
                h4 { class: "sidebar__section-subtitle", "Залог" }
                div { class: "filter-chips filter-chips--sm",
                    for (value, label) in [("act", "Акт."), ("mid", "Мед."), ("pass", "Пас."), ("mid_pass", "Мед./Пас.")] {
                        TagChip { field: "voice".to_string(), value: value.to_string(), label: label.to_string() }
                    }
                }
                h4 { class: "sidebar__section-subtitle", "Наклонение" }
                div { class: "filter-chips filter-chips--sm",
                    for (value, label) in [
                        ("ind", "Изъяв."), ("subj", "Сослаг."), ("opt", "Жел."),
                        ("imp", "Повел."), ("inf", "Инфин."), ("part", "Прич."),
                    ] {
                        TagChip { field: "mood".to_string(), value: value.to_string(), label: label.to_string() }
                    }
                }
            }

            // Case chips (shown when nominal POS selected)
            if show_nominal {
                h4 { class: "sidebar__section-subtitle", "Падеж" }
                div { class: "filter-chips filter-chips--sm",
                    for (value, label) in [
                        ("nom", "Им."), ("gen", "Род."), ("dat", "Дат."),
                        ("acc", "Вин."), ("voc", "Зват."),
                    ] {
                        TagChip { field: "case".to_string(), value: value.to_string(), label: label.to_string() }
                    }
                }
                h4 { class: "sidebar__section-subtitle", "Число" }
                div { class: "filter-chips filter-chips--sm",
                    for (value, label) in [("sg", "Ед."), ("pl", "Мн."), ("du", "Дв.")] {
                        TagChip { field: "number".to_string(), value: value.to_string(), label: label.to_string() }
                    }
                }
            }

            button {
                class: "btn btn--ghost btn--sm",
                onclick: move |_| *state.filter.write() = FilterParams::default(),
                "Сбросить фильтры"
            }
        }
    }
}

#[component]
fn PosChip(value: String, label: String) -> Element {
    let mut state = use_context::<AppState>();
    let active = state.filter.read().pos.contains(&value);
    let value_clone = value.clone();

    rsx! {
        button {
            class: if active { "chip chip--active" } else { "chip" },
            onclick: move |_| {
                let mut filter = state.filter.write();
                if filter.pos.contains(&value_clone) {
                    filter.pos.retain(|p| p != &value_clone);
                } else {
                    filter.pos.push(value_clone.clone());
                }
            },
            "{label}"
        }
    }
}

/// Generic grammatical tag chip: field = "tense" | "case" | "number" | "person" | "voice" | "mood"
#[component]
fn TagChip(field: String, value: String, label: String) -> Element {
    let mut state = use_context::<AppState>();
    let field2 = field.clone();
    let value2 = value.clone();
    let active = {
        let f = state.filter.read();
        let list = tag_field(&f, &field);
        list.contains(&value)
    };

    rsx! {
        button {
            class: if active { "chip chip--active chip--sm" } else { "chip chip--sm" },
            onclick: move |_| {
                let mut filter = state.filter.write();
                let list = tag_field_mut(&mut filter, &field2);
                if list.contains(&value2) {
                    list.retain(|v| v != &value2);
                } else {
                    list.push(value2.clone());
                }
            },
            "{label}"
        }
    }
}

fn tag_field<'a>(f: &'a FilterParams, field: &str) -> &'a Vec<String> {
    match field {
        "tense" => &f.tenses,
        "case" => &f.cases,
        "number" => &f.numbers,
        "person" => &f.persons,
        "voice" => &f.voices,
        "mood" => &f.moods,
        _ => &f.tenses,
    }
}

fn tag_field_mut<'a>(f: &'a mut FilterParams, field: &str) -> &'a mut Vec<String> {
    match field {
        "tense" => &mut f.tenses,
        "case" => &mut f.cases,
        "number" => &mut f.numbers,
        "person" => &mut f.persons,
        "voice" => &mut f.voices,
        "mood" => &mut f.moods,
        _ => &mut f.tenses,
    }
}

// ── Lemma filter panel ────────────────────────────────────────────────────────

#[component]
fn LemmaFilterPanel() -> Element {
    let mut state = use_context::<AppState>();
    let mut lemma_search = use_signal(|| String::new());

    let lemmas = state.lemmas.read().clone();
    let query = lemma_search.read().trim().to_string();
    let query_norm = normalize(&query, true).to_lowercase();
    let query_lower = query.to_lowercase();
    let visible_lemmas: Vec<_> = lemmas.iter()
        .filter(|lemma| {
            if !state.lemma_has_paradigm(lemma.id) {
                return false;
            }
            if query.is_empty() {
                true
            } else {
                normalize(&lemma.greek, true).to_lowercase().contains(&query_norm)
                    || lemma.russian
                        .as_deref()
                        .map(|russian| russian.to_lowercase().contains(&query_lower))
                        .unwrap_or(false)
            }
        })
        .cloned()
        .collect();
    let total_matches = visible_lemmas.len();

    let selected_ids = state.filter.read().lemma_ids.clone();
    let selected_lemmas: Vec<(i64, String)> = selected_ids
        .iter()
        .map(|lid| {
            let name = lemmas
                .iter()
                .find(|lemma| lemma.id == *lid)
                .map(|lemma| lemma.greek.clone())
                .unwrap_or_default();
            (*lid, name)
        })
        .collect();

    rsx! {
        section { class: "sidebar__section",
            h3 { class: "sidebar__section-title", "Конкретные слова" }
            input {
                class: "lemma-search-input",
                r#type: "search",
                placeholder: "Поиск по слову или переводу…",
                value: "{lemma_search.read()}",
                oninput: move |e| *lemma_search.write() = e.value(),
            }
            p { class: "lemma-filter-status",
                if query.is_empty() {
                    "Список слов"
                } else {
                    "Найдено: {total_matches}"
                }
            }
            div { class: "lemma-filter-list",
                if visible_lemmas.is_empty() {
                    p { class: "lemma-filter-empty", "Ничего не найдено" }
                } else {
                    for lemma in visible_lemmas {
                        {
                            let id = lemma.id;
                            let already = selected_ids.contains(&id);
                            let greek = lemma.greek.clone();
                            let russian = lemma.russian.clone().unwrap_or_default();
                            rsx! {
                                button {
                                    class: if already {
                                        "lemma-filter-item__btn lemma-filter-item__btn--active"
                                    } else {
                                        "lemma-filter-item__btn"
                                    },
                                    onclick: move |_| {
                                        let mut filter = state.filter.write();
                                        if filter.lemma_ids.contains(&id) {
                                            filter.lemma_ids.retain(|x| *x != id);
                                        } else {
                                            filter.lemma_ids.push(id);
                                        }
                                    },
                                    span { class: "lemma-filter-item__meta",
                                        span { class: "greek-text", "{greek}" }
                                        if !russian.is_empty() {
                                            span { class: "lemma-filter-item__translation", "{russian}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // Show selected lemmas as removable chips
            if !selected_ids.is_empty() {
                div { class: "filter-chips filter-chips--tags",
                    for (lid, lname) in selected_lemmas {
                        {
                            rsx! {
                                span { class: "chip chip--active chip--removable",
                                    span { class: "greek-text", "{lname}" }
                                    button {
                                        class: "chip__remove",
                                        onclick: move |_| {
                                            state.filter.write().lemma_ids.retain(|x| *x != lid);
                                        },
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
