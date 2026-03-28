use std::collections::HashMap;

use dioxus::prelude::*;

use crate::i18n::{t, UiKey};
use crate::logic::paradigm::{build_nominal_paradigm, build_verb_paradigm};
use crate::models::form::Lemma;
use crate::state::AppState;

// ── WASM sleep helper (reused from flashcard) ────────────────────────────────
#[cfg(target_arch = "wasm32")]
async fn sleep_ms(ms: i32) {
    use wasm_bindgen::prelude::*;
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms)
            .unwrap();
    });
    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
}
#[cfg(not(target_arch = "wasm32"))]
async fn sleep_ms(_ms: i32) {}

// ── Feedback state ────────────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq)]
enum CellFeedback {
    Correct,
    Wrong,
}

// ── BuildParadigmPage ─────────────────────────────────────────────────────────

#[component]
pub fn BuildParadigmPage() -> Element {
    let state = use_context::<AppState>();
    let mut selected_lemma: Signal<Option<i64>> = use_signal(|| None);

    if let Some(lid) = *selected_lemma.read() {
        return rsx! { BuildGame { lemma_id: lid, on_back: move |_| *selected_lemma.write() = None } };
    }

    let lang = state.settings.read().language.clone();
    let title = t(UiKey::ModeBuildTitle, lang.clone());
    let hint = t(UiKey::BuildSelectWord, lang.clone());
    let placeholder = t(UiKey::ParadigmSearch, lang.clone());

    let lemmas = state.lemmas.read();
    let mut search = use_signal(|| String::new());
    let q = search.read().to_lowercase();
    let shown: Vec<&Lemma> = lemmas.iter()
        .filter(|l| q.is_empty()
            || l.greek.contains(q.as_str())
            || l.russian.as_deref().unwrap_or("").to_lowercase().contains(q.as_str())
            || l.english.as_deref().unwrap_or("").to_lowercase().contains(q.as_str()))
        .take(30)
        .collect();

    rsx! {
        div { class: "study-page",
            h2 { class: "study-page__title", "{title}" }
            p { class: "study-page__hint", "{hint}" }
            input {
                class: "lemma-search",
                r#type: "search",
                placeholder: "{placeholder}",
                value: "{search.read()}",
                oninput: move |e| *search.write() = e.value(),
            }
            ul { class: "lemma-list",
                for lemma in shown {
                    {
                        let lid = lemma.id;
                        let greek = lemma.greek.clone();
                        let trans = lemma.translation(&lang).to_string();
                        let pos = lemma.part_of_speech.clone().unwrap_or_default();
                        rsx! {
                            li { class: "lemma-item",
                                button {
                                    class: "lemma-item__btn",
                                    onclick: move |_| *selected_lemma.write() = Some(lid),
                                    span { class: "lemma-item__greek greek-text", "{greek}" }
                                    span { class: "lemma-item__translation", "{trans}" }
                                    span { class: "lemma-item__pos", "[{pos}]" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ── BuildGame ─────────────────────────────────────────────────────────────────

#[component]
fn BuildGame(lemma_id: i64, on_back: EventHandler<()>) -> Element {
    let state = use_context::<AppState>();
    let lang = state.settings.read().language.clone();
    let lemma = match state.lemma_by_id(lemma_id) {
        Some(l) => l,
        None => {
            let msg = t(UiKey::ParadigmNotFound, lang);
            return rsx! { div { "{msg}" } };
        }
    };
    let forms = state.forms_for_lemma(lemma_id);
    let include_dual = state.settings.read().include_dual;
    let pos = lemma.part_of_speech.as_deref().unwrap_or("");

    let table = if pos == "verb" {
        build_verb_paradigm(lemma.clone(), &forms)
    } else {
        let genders: Vec<&'static str> = {
            let mut g = vec![];
            if forms.iter().any(|f| f.gender_tag.as_deref() == Some("m")) { g.push("m"); }
            if forms.iter().any(|f| f.gender_tag.as_deref() == Some("f")) { g.push("f"); }
            if forms.iter().any(|f| f.gender_tag.as_deref() == Some("n")) { g.push("n"); }
            if g.is_empty() { g.push("m"); }
            g
        };
        build_nominal_paradigm(lemma.clone(), &forms, include_dual, &genders)
    };

    // Collect all non-empty form IDs in table order
    let cell_form_ids: Vec<i64> = table.cells.iter()
        .flat_map(|row| row.iter())
        .filter_map(|c| c.form.as_ref().map(|f| f.id))
        .collect();
    let total_cells = cell_form_ids.len();

    // Game state
    let mut filled: Signal<HashMap<i64, bool>> = use_signal(HashMap::new);
    let mut current_idx: Signal<usize> = use_signal(|| 0);
    let mut cell_feedback: Signal<Option<CellFeedback>> = use_signal(|| None);

    let filled_count = filled.read().values().filter(|&&v| v).count();
    let game_done = filled_count >= total_cells;

    // Current target cell
    let cur_idx = *current_idx.read() % total_cells.max(1);
    let maybe_form_id = cell_form_ids.get(cur_idx).copied();

    // Build 4 choices for the current cell
    let choices: Vec<(i64, String)> = if let Some(target_id) = maybe_form_id {
        let target = forms.iter().find(|f| f.id == target_id).cloned();
        if let Some(target_form) = target {
            // Get distractors: same pos, different text
            let mut seen = std::collections::HashSet::new();
            seen.insert(target_form.greek_form.clone());
            let mut distractors: Vec<_> = state.forms.read().iter()
                .filter(|f| {
                    f.id != target_form.id
                        && f.pos == target_form.pos
                        && f.greek_form != target_form.greek_form
                        && seen.insert(f.greek_form.clone())
                })
                .take(3)
                .map(|f| (f.id, f.greek_form.clone()))
                .collect();
            let mut choices = vec![(target_form.id, target_form.greek_form.clone())];
            choices.append(&mut distractors);
            // Shuffle deterministically
            choices.sort_by_key(|(id, _)| (*id as u64).wrapping_mul(6364136223846793005));
            choices
        } else { vec![] }
    } else { vec![] };

    let target_id = maybe_form_id.unwrap_or(0);

    rsx! {
        div { class: "study-page build-game",
            // Header
            div { class: "build-game__header",
                button {
                    class: "btn btn--ghost btn--sm",
                    onclick: move |_| on_back.call(()),
                    "{t(UiKey::Back, lang.clone())}"
                }
                span { class: "build-game__lemma greek-text", "{lemma.greek}" }
                span { class: "build-game__progress", "{filled_count}/{total_cells}" }
            }

            if game_done {
                div { class: "build-game__done",
                    p { class: "build-game__done-text", "{t(UiKey::BuildDone, lang.clone())}" }
                    button {
                        class: "btn btn--primary",
                        onclick: move |_| {
                            *filled.write() = HashMap::new();
                            *current_idx.write() = 0;
                            *cell_feedback.write() = None;
                        },
                        "{t(UiKey::BuildRepeat, lang.clone())}"
                    }
                }
            } else {
                // Paradigm table
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
                                        {
                                            let is_active = cell.form.as_ref().map(|f| f.id) == maybe_form_id;
                                            let is_filled = cell.form.as_ref().map(|f| filled.read().contains_key(&f.id)).unwrap_or(false);
                                            let cell_text = if is_filled {
                                                cell.form.as_ref().map(|f| f.greek_form.clone()).unwrap_or_default()
                                            } else if is_active {
                                                "?".to_string()
                                            } else if cell.form.is_some() {
                                                "·".to_string()
                                            } else {
                                                "—".to_string()
                                            };
                                            let td_class = if cell.form.is_none() {
                                                "paradigm-cell paradigm-cell--empty"
                                            } else if is_active {
                                                match *cell_feedback.read() {
                                                    Some(CellFeedback::Correct) => "paradigm-cell paradigm-cell--active paradigm-cell--correct",
                                                    Some(CellFeedback::Wrong) => "paradigm-cell paradigm-cell--active paradigm-cell--wrong",
                                                    None => "paradigm-cell paradigm-cell--active",
                                                }
                                            } else if is_filled {
                                                "paradigm-cell paradigm-cell--filled"
                                            } else {
                                                "paradigm-cell paradigm-cell--pending"
                                            };
                                            rsx! {
                                                td { class: "{td_class}",
                                                    span { class: "greek-text", "{cell_text}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Choice buttons for current cell
                if !choices.is_empty() {
                    div { class: "build-game__choices",
                        for (choice_id, choice_text) in choices {
                            {
                                let is_correct = choice_id == target_id;
                                let locked = cell_feedback.read().is_some();
                                rsx! {
                                    button {
                                        class: "mc-choice",
                                        disabled: locked,
                                        onclick: move |_| {
                                            if cell_feedback.read().is_some() { return; }
                                            if is_correct {
                                                filled.write().insert(target_id, true);
                                                *cell_feedback.write() = Some(CellFeedback::Correct);
                                                let mut fb = cell_feedback;
                                                let mut ci = current_idx;
                                                let n = total_cells;
                                                spawn(async move {
                                                    sleep_ms(700).await;
                                                    let next = (*ci.read() + 1) % n.max(1);
                                                    *ci.write() = next;
                                                    *fb.write() = None;
                                                });
                                            } else {
                                                *cell_feedback.write() = Some(CellFeedback::Wrong);
                                                let mut fb = cell_feedback;
                                                spawn(async move {
                                                    sleep_ms(1200).await;
                                                    *fb.write() = None;
                                                });
                                            }
                                        },
                                        span { class: "greek-text", "{choice_text}" }
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

