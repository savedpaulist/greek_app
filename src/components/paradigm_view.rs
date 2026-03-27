use dioxus::prelude::*;

use crate::logic::diacritics::normalize;
use crate::logic::paradigm::{build_nominal_paradigm, build_verb_paradigm};
use crate::models::form::Lemma;
use crate::router::Route;
use crate::state::AppState;

const PARADIGM_POS_FILTERS: [(&str, &str); 6] = [
    ("all", "Все"),
    ("noun", "Сущ."),
    ("verb", "Глаг."),
    ("adj", "Прил."),
    ("pronoun", "Мест."),
    ("participle", "Прич."),
];

/// Paradigm view: display full declension/conjugation table for a lemma.
#[component]
pub fn ParadigmTableView(lemma_id: i64) -> Element {
    let state = use_context::<AppState>();
    let settings = state.settings.read();
    let include_dual = settings.include_dual;
    drop(settings);

    let lemma = match state.lemma_by_id(lemma_id) {
        Some(l) => l,
        None => {
            return rsx! {
                div { class: "empty-state", "Лемма не найдена." }
            };
        }
    };

    let forms = state.paradigm_forms_for_lemma(lemma_id);
    if forms.is_empty() {
        return rsx! {
            div { class: "empty-state", "Для этой леммы нет полной парадигмы." }
        };
    }
    let progress = state.progress.read();
    let pos = lemma.part_of_speech.as_deref().unwrap_or("");

    let mut subtitle_parts = vec![];
    if pos == "noun" {
        if let Some(decl) = forms.iter().find_map(|f| f.decl_type.as_deref()) {
            if decl.starts_with('1') {
                subtitle_parts.push("1-е склонение".to_string());
            } else if decl.starts_with('2') {
                subtitle_parts.push("2-е склонение".to_string());
            } else if decl.starts_with('3') {
                subtitle_parts.push("3-е склонение".to_string());
            }
        }
    } else if pos == "verb" {
        if let Some(conj) = forms.iter().find_map(|f| f.conj_type.as_deref()) {
            match conj {
                "thematic" | "thematic_cons" => subtitle_parts.push("на -ω".to_string()),
                "contract_eo" => subtitle_parts.push("на -έω".to_string()),
                "contract_ao" => subtitle_parts.push("на -άω".to_string()),
                "contract_oo" => subtitle_parts.push("на -όω".to_string()),
                "mi_verb" => subtitle_parts.push("на -μι".to_string()),
                _ => {}
            }
        }
    } else if pos == "participle" {
        subtitle_parts.push("причастие".to_string());
        if let Some(Ok(label)) = forms.iter().find_map(|f| f.part_type.as_deref()).map(|pt| match pt {
            "pres_act" => Ok("наст. вр., действ. з."),
            "pres_pass" => Ok("наст. вр., страд. з."),
            "aor1_act" | "aor2_act" => Ok("аорист, действ. з."),
            "aor1_pass" | "aor2_pass" => Ok("аорист, страд. з."),
            "perf_act" => Ok("перфект, действ. з."),
            "perf_pass" => Ok("перфект, страд. з."),
            _ => Err(())
        }) {
            subtitle_parts.push(label.to_string());
        }
    }

    let subtitle = subtitle_parts.join(" · ");

    let table = if pos == "verb" {
        build_verb_paradigm(lemma.clone(), &forms)
    } else {
        let genders: Vec<&str> = {
            let mut g: Vec<&str> = vec![];
            if forms.iter().any(|f| f.gender_tag.as_deref() == Some("m")) {
                g.push("m");
            }
            if forms.iter().any(|f| f.gender_tag.as_deref() == Some("f")) {
                g.push("f");
            }
            if forms.iter().any(|f| f.gender_tag.as_deref() == Some("n")) {
                g.push("n");
            }
            if g.is_empty() {
                g.push("m");
            }
            g
        };
        build_nominal_paradigm(lemma.clone(), &forms, include_dual, &genders)
    };

    rsx! {
        div { class: "paradigm-view",
            // Lemma header
            div { class: "paradigm-header",
                span { class: "paradigm-lemma greek-text", "{table.lemma.greek}" }
                if !subtitle.is_empty() {
                    span { class: "paradigm-meta", "{subtitle}" }
                }
                if let Some(translation) = table.lemma.russian.as_deref().or(table.lemma.english.as_deref()) {
                    span { class: "paradigm-translation", "«{translation}»" }
                }
            }

            // Scrollable table wrapper
            div { class: "paradigm-table-wrapper",
                table { class: "paradigm-table",
                    // Header row
                    thead {
                        tr {
                            th { class: "paradigm-table__corner", "" }
                            for header in &table.col_headers {
                                th { class: "paradigm-table__col-header", "{header}" }
                            }
                        }
                    }
                    // Data rows
                    tbody {
                        for (row_idx, row) in table.cells.iter().enumerate() {
                            tr {
                                th { class: "paradigm-table__row-header",
                                    "{table.row_headers[row_idx]}"
                                }
                                for cell in row {
                                    td {
                                        class: {
                                            if let Some(form) = &cell.form {
                                                let streak = progress
                                                    .get(&form.id)
                                                    .map(|p| p.streak)
                                                    .unwrap_or(0);
                                                if streak >= 5 {
                                                    "paradigm-cell paradigm-cell--learned"
                                                } else if streak > 0 {
                                                    "paradigm-cell paradigm-cell--seen"
                                                } else {
                                                    "paradigm-cell"
                                                }
                                            } else {
                                                "paradigm-cell paradigm-cell--empty"
                                            }
                                        },
                                        if let Some(form) = &cell.form {
                                            span { class: "greek-text", "{form.greek_form}" }
                                        } else {
                                            span { "—" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Legend
            div { class: "paradigm-legend",
                span { class: "legend-dot legend-dot--learned", "" }
                span { "Выучено (≥5)" }
                span { class: "legend-dot legend-dot--seen", "" }
                span { "В процессе" }
            }
        }
    }
}

// ── Lemma picker for paradigm view ────────────────────────────────────────────

#[component]
pub fn LemmaPicker() -> Element {
    let state = use_context::<AppState>();
    let mut selected_lemma = use_signal(|| Option::<i64>::None);
    let search = use_signal(|| String::new());
    let mut pos_filter = use_signal(|| "all".to_string());

    let lemmas = state.lemmas.read();
    let search_value = search.read().clone();
    let pos_value = pos_filter.read().clone();
    let normalized_query = if search_value.is_empty() {
        None
    } else {
        Some(normalize(search_value.as_str(), true))
    };

    let filtered_lemmas: Vec<_> = lemmas
        .iter()
        .filter(|l| {
            if !state.lemma_has_paradigm(l.id) {
                return false;
            }
            if pos_value != "all" && l.part_of_speech.as_deref() != Some(pos_value.as_str()) {
                return false;
            }

            if let Some(query) = &normalized_query {
                let lowercase_query = search_value.to_lowercase();
                normalize(&l.greek, true).contains(query)
                    || l.russian
                        .as_deref()
                        .map(|r| r.to_lowercase().contains(lowercase_query.as_str()))
                        .unwrap_or(false)
                    || l.english
                        .as_deref()
                        .map(|e| e.to_lowercase().contains(lowercase_query.as_str()))
                        .unwrap_or(false)
            } else {
                true
            }
        })
        .collect();

    rsx! {
        div { class: "lemma-picker",
            div { class: "paradigm-pos-filter",
                for (value, label) in PARADIGM_POS_FILTERS {
                    {
                        let is_active = pos_value == value;
                        let value = value.to_string();
                        rsx! {
                            button {
                                class: if is_active { "chip chip--active chip--sm" } else { "chip chip--sm" },
                                onclick: move |_| *pos_filter.write() = value.clone(),
                                "{label}"
                            }
                        }
                    }
                }
            }

            // Search box
            input {
                class: "lemma-search",
                r#type: "search",
                placeholder: "Поиск…",
                value: "{search.read()}",
                oninput: {
                    let mut search = search.clone();
                    move |e| *search.write() = e.value()
                },
            }

            if let Some(lid) = *selected_lemma.read() {
                button {
                    class: "btn btn--ghost btn--sm",
                    onclick: move |_| *selected_lemma.write() = None,
                    "← Все леммы"
                }
                ParadigmTableView { lemma_id: lid }
            } else {
                div { class: "lemma-list-container",
                    ul { class: "lemma-list",
                        {
                            let custom_ids: std::collections::HashSet<i64> = state
                                .custom_lemmas
                                .read()
                                .iter()
                                .map(|l| l.id)
                                .collect();
                            rsx! {
                                for lemma in filtered_lemmas {
                                    {
                                        let lid = lemma.id;
                                        let is_custom = custom_ids.contains(&lid);
                                        rsx! {
                                            LemmaRow {
                                                lemma: lemma.clone(),
                                                is_custom,
                                                on_select: {
                                                    let mut selected_lemma = selected_lemma.clone();
                                                    move |_| *selected_lemma.write() = Some(lid)
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

#[component]
fn LemmaRow(
    lemma: Lemma,
    is_custom: bool,
    on_select: EventHandler<MouseEvent>,
) -> Element {
    let translation = lemma.russian.as_deref().or(lemma.english.as_deref()).unwrap_or("");
    let pos = lemma.part_of_speech.as_deref().unwrap_or("?");
    let lemma_id = lemma.id;
    rsx! {
        li { class: "lemma-item",
            button { class: "lemma-item__btn", onclick: move |e| on_select.call(e),
                span { class: "lemma-item__greek greek-text", "{lemma.greek}" }
                span { class: "lemma-item__translation", "{translation}" }
                span { class: "lemma-item__pos", "[{pos}]" }
            }
            if is_custom {
                Link {
                    to: Route::ParadigmBuilderEdit { lemma_id },
                    class: "btn btn--ghost btn--sm lemma-item__edit-btn",
                    "Изменить"
                }
            }
        }
    }
}
