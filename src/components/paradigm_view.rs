use dioxus::prelude::*;

use crate::i18n::{t, UiKey};
use crate::logic::diacritics::normalize;
use crate::logic::paradigm::{
    build_nominal_paradigm, build_verb_paradigms,
    mood_label, mood_order, tense_label, tense_order, voice_label, voice_order,
};
use crate::models::form::Lemma;
use crate::router::Route;
use crate::state::AppState;
use crate::state::settings::UiLanguage;

/// Paradigm view: display full declension/conjugation table for a lemma.
#[component]
pub fn ParadigmTableView(lemma_id: i64) -> Element {
    let state = use_context::<AppState>();
    let settings = state.settings.read().clone();
    let include_dual = settings.include_dual;
    let lang = settings.language.clone();
    let morph_lang = settings.morph_language.clone();

    // ── Verb paradigm filters (local state, only used for verbs) ──────────
    let mut tense_filter = use_signal(|| Vec::<String>::new());
    let mut voice_filter = use_signal(|| Vec::<String>::new());
    let mut mood_filter  = use_signal(|| Vec::<String>::new());

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
                subtitle_parts.push(match morph_lang { UiLanguage::Ru => "1-е склонение", UiLanguage::En => "1st declension" }.to_string());
            } else if decl.starts_with('2') {
                subtitle_parts.push(match morph_lang { UiLanguage::Ru => "2-е склонение", UiLanguage::En => "2nd declension" }.to_string());
            } else if decl.starts_with('3') {
                subtitle_parts.push(match morph_lang { UiLanguage::Ru => "3-е склонение", UiLanguage::En => "3rd declension" }.to_string());
            }
        }
    } else if pos == "verb" {
        if let Some(conj) = forms.iter().find_map(|f| f.conj_type.as_deref()) {
            let label = match (conj, &morph_lang) {
                ("thematic" | "thematic_cons", UiLanguage::Ru) => "на -ω",
                ("thematic" | "thematic_cons", UiLanguage::En) => "-ω type",
                ("contract_eo", UiLanguage::Ru) => "на -έω",
                ("contract_eo", UiLanguage::En) => "-έω contract",
                ("contract_ao", UiLanguage::Ru) => "на -άω",
                ("contract_ao", UiLanguage::En) => "-άω contract",
                ("contract_oo", UiLanguage::Ru) => "на -όω",
                ("contract_oo", UiLanguage::En) => "-όω contract",
                ("mi_verb", UiLanguage::Ru) => "на -μι",
                ("mi_verb", UiLanguage::En) => "-μι type",
                _ => "",
            };
            if !label.is_empty() {
                subtitle_parts.push(label.to_string());
            }
        }
    } else if pos == "participle" {
        subtitle_parts.push(match morph_lang { UiLanguage::Ru => "причастие", UiLanguage::En => "participle" }.to_string());
        if let Some(Ok(label)) = forms.iter().find_map(|f| f.part_type.as_deref()).map(|pt| {
            match (&morph_lang, pt) {
                (UiLanguage::Ru, "pres_act") => Ok("наст. вр., действ. з."),
                (UiLanguage::Ru, "pres_pass") => Ok("наст. вр., страд. з."),
                (UiLanguage::Ru, "aor1_act" | "aor2_act") => Ok("аорист, действ. з."),
                (UiLanguage::Ru, "aor1_pass" | "aor2_pass") => Ok("аорист, страд. з."),
                (UiLanguage::Ru, "perf_act") => Ok("перфект, действ. з."),
                (UiLanguage::Ru, "perf_pass") => Ok("перфект, страд. з."),
                (UiLanguage::En, "pres_act") => Ok("pres. act."),
                (UiLanguage::En, "pres_pass") => Ok("pres. pass."),
                (UiLanguage::En, "aor1_act" | "aor2_act") => Ok("aor. act."),
                (UiLanguage::En, "aor1_pass" | "aor2_pass") => Ok("aor. pass."),
                (UiLanguage::En, "perf_act") => Ok("perf. act."),
                (UiLanguage::En, "perf_pass") => Ok("perf. pass."),
                _ => Err(())
            }
        }) {
            subtitle_parts.push(label.to_string());
        }
    }

    let subtitle = subtitle_parts.join(" · ");

    if pos == "verb" {
        let verb_tables = build_verb_paradigms(lemma.clone(), &forms, include_dual, &morph_lang);

        // Collect unique tense/voice/mood keys present in the tables (ordered).
        let mut unique_tenses: Vec<String> = vec![];
        let mut unique_voices: Vec<String> = vec![];
        let mut unique_moods: Vec<String> = vec![];
        for t in &verb_tables {
            if let Some(k) = &t.tense_key {
                if !unique_tenses.contains(k) { unique_tenses.push(k.clone()); }
            }
            if let Some(k) = &t.voice_key {
                if !unique_voices.contains(k) { unique_voices.push(k.clone()); }
            }
            if let Some(k) = &t.mood_key {
                if !unique_moods.contains(k) { unique_moods.push(k.clone()); }
            }
        }
        unique_tenses.sort_by_key(|k| tense_order(k.as_str()));
        unique_voices.sort_by_key(|k| voice_order(k.as_str()));
        unique_moods.sort_by_key(|k| mood_order(k.as_str()));

        // Apply filters.
        let tf = tense_filter.read().clone();
        let vf = voice_filter.read().clone();
        let mf = mood_filter.read().clone();
        let filtered_tables: Vec<_> = verb_tables.iter()
            .filter(|t| {
                let tense_ok = tf.is_empty() || t.tense_key.as_ref().map(|k| tf.contains(k)).unwrap_or(true);
                let voice_ok = vf.is_empty() || t.voice_key.as_ref().map(|k| vf.contains(k)).unwrap_or(true);
                let mood_ok  = mf.is_empty() || t.mood_key.as_ref().map(|k| mf.contains(k)).unwrap_or(true);
                tense_ok && voice_ok && mood_ok
            })
            .collect();

        // Tense filter label
        let tense_section_label = match morph_lang {
            UiLanguage::Ru => t(UiKey::FiltersTense, lang.clone()),
            UiLanguage::En => t(UiKey::FiltersTense, lang.clone()),
        };
        let voice_section_label = t(UiKey::FiltersVoice, lang.clone());
        let mood_section_label  = t(UiKey::FiltersMood,  lang.clone());

        rsx! {
            div { class: "paradigm-view",
                div { class: "paradigm-header",
                    span { class: "paradigm-lemma greek-text", "{lemma.greek}" }
                    if !subtitle.is_empty() {
                        span { class: "paradigm-meta", "{subtitle}" }
                    }
                    {
                        let tr = lemma.translation(&lang);
                        if !tr.is_empty() {
                            rsx! { span { class: "paradigm-translation", "«{tr}»" } }
                        } else { rsx! {} }
                    }
                }

                // ── Verb filter chips ─────────────────────────────────────
                if unique_tenses.len() > 1 || unique_voices.len() > 1 || unique_moods.len() > 1 {
                    div { class: "paradigm-verb-filter",
                        if unique_tenses.len() > 1 {
                            span { class: "paradigm-verb-filter__label", "{tense_section_label}" }
                            div { class: "filter-chips filter-chips--sm",
                                for key in unique_tenses {
                                    {
                                        let key2 = key.clone();
                                        let label = tense_label(&key, &morph_lang).to_string();
                                        let active = tf.contains(&key);
                                        rsx! {
                                            button {
                                                class: if active { "chip chip--active chip--sm" } else { "chip chip--sm" },
                                                onclick: move |_| {
                                                    let mut f = tense_filter.write();
                                                    if f.contains(&key2) { f.retain(|x| x != &key2); } else { f.push(key2.clone()); }
                                                },
                                                "{label}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if unique_voices.len() > 1 {
                            span { class: "paradigm-verb-filter__label", "{voice_section_label}" }
                            div { class: "filter-chips filter-chips--sm",
                                for key in unique_voices {
                                    {
                                        let key2 = key.clone();
                                        let label = voice_label(&key, &morph_lang).to_string();
                                        let active = vf.contains(&key);
                                        rsx! {
                                            button {
                                                class: if active { "chip chip--active chip--sm" } else { "chip chip--sm" },
                                                onclick: move |_| {
                                                    let mut f = voice_filter.write();
                                                    if f.contains(&key2) { f.retain(|x| x != &key2); } else { f.push(key2.clone()); }
                                                },
                                                "{label}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if unique_moods.len() > 1 {
                            span { class: "paradigm-verb-filter__label", "{mood_section_label}" }
                            div { class: "filter-chips filter-chips--sm",
                                for key in unique_moods {
                                    {
                                        let key2 = key.clone();
                                        let label = mood_label(&key, &morph_lang).to_string();
                                        let active = mf.contains(&key);
                                        rsx! {
                                            button {
                                                class: if active { "chip chip--active chip--sm" } else { "chip chip--sm" },
                                                onclick: move |_| {
                                                    let mut f = mood_filter.write();
                                                    if f.contains(&key2) { f.retain(|x| x != &key2); } else { f.push(key2.clone()); }
                                                },
                                                "{label}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                div { class: "verb-paradigm-blocks",
                    for table in &filtered_tables {
                        div { class: "verb-paradigm-block",
                            if let Some(title) = &table.title {
                                h4 { class: "verb-paradigm-title", "{title}" }
                            }
                            div { class: "paradigm-table-wrapper",
                                table { class: "paradigm-table paradigm-table--verb",
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
    } else {
        let genders: Vec<&str> = {
            let mut g: Vec<&str> = vec![];
            if forms.iter().any(|f| f.gender_tag.as_deref() == Some("m")) { g.push("m"); }
            if forms.iter().any(|f| f.gender_tag.as_deref() == Some("f")) { g.push("f"); }
            if forms.iter().any(|f| f.gender_tag.as_deref() == Some("n")) { g.push("n"); }
            if g.is_empty() { g.push("m"); }
            g
        };
        let table = build_nominal_paradigm(lemma.clone(), &forms, include_dual, &genders, &morph_lang);
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
    // Normalize query for Greek (strip diacritics, lowercase)
    let query_norm = normalize(search_value.as_str(), true).to_lowercase();

    let mut filtered_lemmas: Vec<_> = lemmas
        .iter()
        .filter(|l| {
            if !state.lemma_has_paradigm(l.id) {
                return false;
            }
            if pos_value != "all" && l.part_of_speech.as_deref() != Some(pos_value.as_str()) {
                return false;
            }
            if search_value.is_empty() {
                true
            } else {
                let lowercase_query = search_value.to_lowercase();
                normalize(&l.greek, true).to_lowercase().contains(&query_norm)
                    || l.russian
                        .as_deref()
                        .map(|r| r.to_lowercase().contains(lowercase_query.as_str()))
                        .unwrap_or(false)
                    || l.english
                        .as_deref()
                        .map(|e| e.to_lowercase().contains(lowercase_query.as_str()))
                        .unwrap_or(false)
            }
        })
        .collect();

    // Sort alphabetically by normalized Greek
    filtered_lemmas.sort_by(|a, b| {
        normalize(&a.greek, true).cmp(&normalize(&b.greek, true))
    });

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
                div { class: "paradigm-detail-container",
                    ParadigmTableView { lemma_id: lid }
                }
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
    let state = use_context::<AppState>();
    let lang = state.settings.read().language.clone();
    let translation = lemma.translation(&lang);
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
