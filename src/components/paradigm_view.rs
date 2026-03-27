use dioxus::prelude::*;

use crate::i18n::{t, UiKey};
use crate::logic::diacritics::normalize;
use crate::logic::paradigm::{build_nominal_paradigm, build_verb_paradigm};
use crate::models::form::Lemma;
use crate::router::Route;
use crate::state::AppState;

/// Paradigm view: display full declension/conjugation table for a lemma.
#[component]
pub fn ParadigmTableView(lemma_id: i64) -> Element {
    let state = use_context::<AppState>();
    let settings = state.settings.read().clone();
    let include_dual = settings.include_dual;
    let lang = settings.language.clone();
    let morph_lang = settings.morph_language.clone();

    let lemma = match state.lemma_by_id(lemma_id) {
        Some(l) => l,
        None => {
            return rsx! {
                div { class: "empty-state", "{t(UiKey::ParadigmNotFound, lang.clone())}" }
            };
        }
    };

    let forms = state.paradigm_forms_for_lemma(lemma_id);
    if forms.is_empty() {
        return rsx! {
            div { class: "empty-state", "{t(UiKey::ParadigmNoTable, lang.clone())}" }
        };
    }
    let progress = state.progress.read();
    let pos = lemma.part_of_speech.as_deref().unwrap_or("");

    let mut subtitle_parts: Vec<String> = vec![];
    if pos == "noun" {
        if let Some(decl) = forms.iter().find_map(|f| f.decl_type.as_deref()) {
            if decl.starts_with('1') {
                subtitle_parts.push(match morph_lang { crate::state::settings::UiLanguage::Ru => "1-е склонение", crate::state::settings::UiLanguage::En => "1st declension" }.to_string());
            } else if decl.starts_with('2') {
                subtitle_parts.push(match morph_lang { crate::state::settings::UiLanguage::Ru => "2-е склонение", crate::state::settings::UiLanguage::En => "2nd declension" }.to_string());
            } else if decl.starts_with('3') {
                subtitle_parts.push(match morph_lang { crate::state::settings::UiLanguage::Ru => "3-е склонение", crate::state::settings::UiLanguage::En => "3rd declension" }.to_string());
            }
        }
    } else if pos == "verb" {
        if let Some(conj) = forms.iter().find_map(|f| f.conj_type.as_deref()) {
            let label = match (conj, &morph_lang) {
                ("thematic" | "thematic_cons", crate::state::settings::UiLanguage::Ru) => "на -ω",
                ("thematic" | "thematic_cons", crate::state::settings::UiLanguage::En) => "-ω type",
                ("contract_eo", crate::state::settings::UiLanguage::Ru) => "на -έω",
                ("contract_eo", crate::state::settings::UiLanguage::En) => "-έω contract",
                ("contract_ao", crate::state::settings::UiLanguage::Ru) => "на -άω",
                ("contract_ao", crate::state::settings::UiLanguage::En) => "-άω contract",
                ("contract_oo", crate::state::settings::UiLanguage::Ru) => "на -όω",
                ("contract_oo", crate::state::settings::UiLanguage::En) => "-όω contract",
                ("mi_verb", crate::state::settings::UiLanguage::Ru) => "на -μι",
                ("mi_verb", crate::state::settings::UiLanguage::En) => "-μι type",
                _ => "",
            };
            if !label.is_empty() {
                subtitle_parts.push(label.to_string());
            }
        }
    } else if pos == "participle" {
        subtitle_parts.push(match morph_lang { crate::state::settings::UiLanguage::Ru => "причастие", crate::state::settings::UiLanguage::En => "participle" }.to_string());
        if let Some(Ok(label)) = forms.iter().find_map(|f| f.part_type.as_deref()).map(|pt| {
            match (&morph_lang, pt) {
                (crate::state::settings::UiLanguage::Ru, "pres_act") => Ok("наст. вр., действ. з."),
                (crate::state::settings::UiLanguage::Ru, "pres_pass") => Ok("наст. вр., страд. з."),
                (crate::state::settings::UiLanguage::Ru, "aor1_act" | "aor2_act") => Ok("аорист, действ. з."),
                (crate::state::settings::UiLanguage::Ru, "aor1_pass" | "aor2_pass") => Ok("аорист, страд. з."),
                (crate::state::settings::UiLanguage::Ru, "perf_act") => Ok("перфект, действ. з."),
                (crate::state::settings::UiLanguage::Ru, "perf_pass") => Ok("перфект, страд. з."),
                (crate::state::settings::UiLanguage::En, "pres_act") => Ok("pres. act."),
                (crate::state::settings::UiLanguage::En, "pres_pass") => Ok("pres. pass."),
                (crate::state::settings::UiLanguage::En, "aor1_act" | "aor2_act") => Ok("aor. act."),
                (crate::state::settings::UiLanguage::En, "aor1_pass" | "aor2_pass") => Ok("aor. pass."),
                (crate::state::settings::UiLanguage::En, "perf_act") => Ok("perf. act."),
                (crate::state::settings::UiLanguage::En, "perf_pass") => Ok("perf. pass."),
                _ => Err(())
            }
        }) {
            subtitle_parts.push(label.to_string());
        }
    }

    let subtitle = subtitle_parts.join(" · ");

    let table = if pos == "verb" {
        build_verb_paradigm(lemma.clone(), &forms, &morph_lang)
    } else {
        let genders: Vec<&str> = {
            let mut g: Vec<&str> = vec![];
            if forms.iter().any(|f| f.gender_tag.as_deref() == Some("m")) { g.push("m"); }
            if forms.iter().any(|f| f.gender_tag.as_deref() == Some("f")) { g.push("f"); }
            if forms.iter().any(|f| f.gender_tag.as_deref() == Some("n")) { g.push("n"); }
            if g.is_empty() { g.push("m"); }
            g
        };
        build_nominal_paradigm(lemma.clone(), &forms, include_dual, &genders, &morph_lang)
    };

    rsx! {
        div { class: "paradigm-view",
            div { class: "paradigm-header",
                span { class: "paradigm-lemma greek-text", "{table.lemma.greek}" }
                if !subtitle.is_empty() {
                    span { class: "paradigm-meta", "{subtitle}" }
                }
                if let Some(translation) = table.lemma.russian.as_deref().or(table.lemma.english.as_deref()) {
                    span { class: "paradigm-translation", "«{translation}»" }
                }
            }

            div { class: "paradigm-table-wrapper",
                table { class: "paradigm-table",
                    thead {
                        tr {
                            th { class: "paradigm-table__corner", "" }
                            for header in &table.col_headers {
                                th { class: "paradigm-table__col-header", "{header}" }
                            }
                        }
                    }
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

            div { class: "paradigm-legend",
                span { class: "legend-dot legend-dot--learned", "" }
                span { "{t(UiKey::ParadigmLegendLearned, lang.clone())}" }
                span { class: "legend-dot legend-dot--seen", "" }
                span { "{t(UiKey::ParadigmLegendSeen, lang.clone())}" }
            }
        }
    }
}

// ── Lemma picker for paradigm view ────────────────────────────────────────────

#[component]
pub fn LemmaPicker() -> Element {
    let state = use_context::<AppState>();
    let settings_snap = state.settings.read().clone();
    let lang = settings_snap.language.clone();
    let morph_lang = settings_snap.morph_language.clone();
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

    // POS filter chips computed from morph_lang
    let pos_filters: Vec<(&str, String)> = vec![
        ("all",        t(UiKey::FilterPosAll,     morph_lang.clone()).to_string()),
        ("noun",       t(UiKey::FilterPosNoun,    morph_lang.clone()).to_string()),
        ("verb",       t(UiKey::FilterPosVerb,    morph_lang.clone()).to_string()),
        ("adj",        t(UiKey::FilterPosAdj,     morph_lang.clone()).to_string()),
        ("pronoun",    t(UiKey::FilterPosPronoun, morph_lang.clone()).to_string()),
        ("participle", t(UiKey::FilterPosPart,    morph_lang.clone()).to_string()),
    ];

    rsx! {
        div { class: "lemma-picker",
            div { class: "paradigm-pos-filter",
                for (value, label) in pos_filters {
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

            input {
                class: "lemma-search",
                r#type: "search",
                placeholder: t(UiKey::ParadigmSearch, lang.clone()),
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
                    "{t(UiKey::ParadigmBack, lang.clone())}"
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
                                                edit_label: t(UiKey::BuilderEditBtn, lang.clone()).to_string(),
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
    edit_label: String,
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
                    "{edit_label}"
                }
            }
        }
    }
}
