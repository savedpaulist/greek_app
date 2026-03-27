use std::collections::HashSet;

use dioxus::prelude::*;

use crate::logic::sm2::quality_from_answer;
use crate::models::Form;
use crate::state::AppState;

const SESSION_SIZE: usize = 10;

const PEPE_IMGS: [Asset; 11] = [
    asset!("/assets/pepe/pepe_0.png"),
    asset!("/assets/pepe/pepe_1.png"),
    asset!("/assets/pepe/pepe_2.png"),
    asset!("/assets/pepe/pepe_3.png"),
    asset!("/assets/pepe/pepe_4.png"),
    asset!("/assets/pepe/pepe_5.png"),
    asset!("/assets/pepe/pepe_6.png"),
    asset!("/assets/pepe/pepe_7.png"),
    asset!("/assets/pepe/pepe_8.png"),
    asset!("/assets/pepe/pepe_9.png"),
    asset!("/assets/pepe/pepe_10.png"),
];

// ── WASM async sleep helper ───────────────────────────────────────────────

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

// ── Feedback state ────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq)]
enum Feedback {
    Correct,
    Wrong(i64), // form_id of the wrong choice the user picked
}

// ── FlashcardView ─────────────────────────────────────────────────────────

/// Flashcard: show lemma + grammar prompt, user picks the correct form.
#[component]
pub fn FlashcardView(reverse: bool) -> Element {
    let mut state = use_context::<AppState>();
    let forms = state.filtered_forms();

    if forms.is_empty() {
        return rsx! {
            div { class: "empty-state",
                p { "Нет форм по текущему фильтру." }
            }
        };
    }

    let total = forms.len();
    let session_count = total.min(SESSION_SIZE);
    let mut index = use_signal(|| 0usize);
    let mut revealed = use_signal(|| false);
    let mut feedback: Signal<Option<Feedback>> = use_signal(|| None);
    let mut session_correct: Signal<u32> = use_signal(|| 0);
    let mut shuffle_seed = use_signal(fresh_shuffle_seed);

    let order = build_shuffled_order(&forms, *shuffle_seed.read());

    // ── Session results screen ──────────────────────────────────────────────
    if *index.read() >= session_count {
        let correct = (*session_correct.read() as usize).min(10);
        let pepe_src = PEPE_IMGS[correct];
        return rsx! {
            div { class: "session-results",
                h2 { class: "session-results__score",
                    "{*session_correct.read()}/{session_count}"
                }
                img {
                    class: "session-results__pepe",
                    src: pepe_src,
                    alt: "результат",
                }
                button {
                    class: "btn btn--primary",
                    onclick: move |_| {
                        *index.write() = 0;
                        *session_correct.write() = 0;
                        *revealed.write() = false;
                        *feedback.write() = None;
                        *shuffle_seed.write() = fresh_shuffle_seed();
                    },
                    "Ещё раз"
                }
            }
        };
    }

    let real_idx = order[*index.read() % total];
    let current_form = forms[real_idx].clone();
    let current_lemma = state.lemma_by_id(current_form.lemma_id);
    let revealed_answer = if reverse {
        current_form.test_prompt_greek()
    } else {
        current_form.grammar_label_ru()
    };
    let form_choices = if reverse {
        build_same_lemma_choices(&forms, &current_form, 6)
    } else {
        build_same_lemma_grammar_choices(&forms, &current_form, 6)
    };

    rsx! {
        div { class: "flashcard-screen",
            // Progress bar
            div { class: "session-progress",
                div {
                    class: "session-progress__bar",
                    style: "width: {(*index.read() * 100 / session_count)}%;",
                }
                span { class: "session-progress__label",
                    "{*index.read() + 1}/{session_count}"
                }
            }

            div { class: if reverse { "flashcard flashcard--reverse" } else { "flashcard" },
                // Prompt
                div { class: "flashcard__prompt",
                    if reverse {
                        if let Some(lemma) = &current_lemma {
                            if let Some(ru) = &lemma.russian {
                                p { class: "flashcard__translation", "«{ru}»" }
                            }
                        }
                        p { class: "flashcard__grammar", "{current_form.grammar_label_ru()}" }
                    } else {
                        p { class: "flashcard__lemma greek-text", "{current_form.test_prompt_greek()}" }
                        if let Some(lemma) = &current_lemma {
                            if let Some(ru) = &lemma.russian {
                                p { class: "flashcard__translation", "«{ru}»" }
                            }
                        }
                    }
                }

                // Answer area
                div { class: "flashcard__answers",
                    if *revealed.read() {
                        div { class: "flashcard__reveal",
                            p { class: "flashcard__answer greek-text",
                                "{revealed_answer}"
                            }
                            div { class: "flashcard__buttons",
                                button {
                                    class: "btn btn--danger",
                                    onclick: move |_| {
                                        state.record_answer(current_form.id, 1);
                                        let next = *index.read() + 1;
                                        *index.write() = next;
                                        *revealed.write() = false;
                                        *feedback.write() = None;
                                    },
                                    "✗ Не знал"
                                }
                                button {
                                    class: "btn btn--warning",
                                    onclick: move |_| {
                                        state.record_answer(current_form.id, 3);
                                        *session_correct.write() += 1;
                                        let next = *index.read() + 1;
                                        *index.write() = next;
                                        *revealed.write() = false;
                                        *feedback.write() = None;
                                    },
                                    "~ С трудом"
                                }
                                button {
                                    class: "btn btn--success",
                                    onclick: move |_| {
                                        state.record_answer(current_form.id, 5);
                                        *session_correct.write() += 1;
                                        let next = *index.read() + 1;
                                        *index.write() = next;
                                        *revealed.write() = false;
                                        *feedback.write() = None;
                                    },
                                    "✓ Знал"
                                }
                            }
                        }
                    } else {
                        div { class: "mc-choices",
                            for choice in form_choices {
                                {
                                    let is_correct_choice = choice.id == current_form.id;
                                    let chosen_id = choice.id;
                                    let fb = *feedback.read();
                                    let btn_class = match fb {
                                        Some(Feedback::Correct) if is_correct_choice =>
                                            "mc-choice mc-choice--correct",
                                        Some(Feedback::Wrong(id)) if id == chosen_id =>
                                            "mc-choice mc-choice--wrong",
                                        Some(Feedback::Wrong(_)) if is_correct_choice =>
                                            "mc-choice mc-choice--correct",
                                        _ => "mc-choice",
                                    };
                                    let locked = fb.is_some();
                                    rsx! {
                                        button {
                                            class: "{btn_class}",
                                            disabled: locked,
                                            onclick: move |_| {
                                                if feedback.read().is_some() { return; }
                                                let q = quality_from_answer(is_correct_choice, false);
                                                state.record_answer(current_form.id, q);
                                                if is_correct_choice {
                                                    *session_correct.write() += 1;
                                                    *feedback.write() = Some(Feedback::Correct);
                                                    let mut fb = feedback;
                                                    let mut idx = index;
                                                    spawn(async move {
                                                        sleep_ms(800).await;
                                                        *fb.write() = None;
                                                        *idx.write() += 1;
                                                    });
                                                } else {
                                                    *feedback.write() = Some(Feedback::Wrong(chosen_id));
                                                    let mut fb = feedback;
                                                    let mut idx = index;
                                                    let mut rev = revealed;
                                                    spawn(async move {
                                                        sleep_ms(1500).await;
                                                        *fb.write() = None;
                                                        *rev.write() = false;
                                                        *idx.write() += 1;
                                                    });
                                                }
                                            },
                                            if reverse {
                                                span { class: "greek-text", "{choice.test_prompt_greek()}" }
                                            } else {
                                                span { class: "mc-choice__grammar", "{choice.grammar_label_ru()}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        button {
                            class: "btn btn--ghost btn--sm",
                            disabled: feedback.read().is_some(),
                            onclick: move |_| *revealed.write() = true,
                            "Показать ответ"
                        }
                    }
                }
            }
        }
    }
}

// ── Choice builder ────────────────────────────────────────────────────────

fn build_shuffled_order(forms: &[Form], seed: u64) -> Vec<usize> {
    let n = forms.len();
    let mut order: Vec<usize> = (0..n).collect();
    let shuffle_seed = forms.iter().enumerate().fold(seed, |acc, (i, form)| {
        acc.wrapping_add(
            (form.id as u64).wrapping_mul((i as u64 + 1).wrapping_mul(6364136223846793005)),
        )
    });
    let mut rng = shuffle_seed;
    for i in (1..n).rev() {
        rng = rng
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let j = (rng >> 33) as usize % (i + 1);
        order.swap(i, j);
    }
    separate_adjacent_lemmas(&mut order, forms, shuffle_seed);
    order
}

fn separate_adjacent_lemmas(order: &mut [usize], forms: &[Form], seed: u64) {
    if order.len() < 2 {
        return;
    }

    let mut cursor = seed as usize;
    for idx in 1..order.len() {
        let prev_lemma = forms[order[idx - 1]].lemma_id;
        let current_lemma = forms[order[idx]].lemma_id;
        if prev_lemma != current_lemma {
            continue;
        }

        let mut swap_with = None;
        for offset in idx + 1..order.len() {
            let candidate_lemma = forms[order[offset]].lemma_id;
            if candidate_lemma != prev_lemma {
                swap_with = Some(offset);
                break;
            }
        }

        if swap_with.is_none() {
            for _ in 0..idx {
                cursor = (cursor + 1) % idx;
                let candidate_lemma = forms[order[cursor]].lemma_id;
                let before_ok = cursor == 0 || forms[order[cursor - 1]].lemma_id != current_lemma;
                let after_ok = cursor + 1 >= order.len() || forms[order[cursor + 1]].lemma_id != current_lemma;
                if candidate_lemma != current_lemma && before_ok && after_ok {
                    swap_with = Some(cursor);
                    break;
                }
            }
        }

        if let Some(target) = swap_with {
            order.swap(idx, target);
        }
    }
}

fn fresh_shuffle_seed() -> u64 {
    #[cfg(target_arch = "wasm32")]
    {
        ((js_sys::Date::now() as u64) << 16) ^ ((js_sys::Math::random() * 1_000_000.0) as u64)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::time::{SystemTime, UNIX_EPOCH};

        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos() as u64)
            .unwrap_or(0x9e3779b97f4a7c15)
    }
}

/// Pick `n` forms (1 correct + n-1 distractors) for multiple choice.
/// Uses only forms from the same lemma/paradigm as the asked form.
fn build_same_lemma_choices(forms: &[Form], correct: &Form, n: usize) -> Vec<Form> {
    let mut seen: HashSet<String> = HashSet::new();
    seen.insert(correct.greek_form.clone());

    let mut distractors: Vec<Form> = forms
        .iter()
        .filter(|f| {
            if f.id == correct.id { return false; }
            if f.lemma_id != correct.lemma_id { return false; }
            if f.greek_form == correct.greek_form { return false; }
            true
        })
        .cloned()
        .collect();

    distractors.sort_by_key(|f| {
        (f.id as u64).wrapping_mul(2654435761)
            ^ (correct.id as u64).wrapping_mul(1000003)
    });

    let mut choices = vec![correct.clone()];
    for d in distractors {
        if choices.len() >= n { break; }
        if seen.insert(d.greek_form.clone()) {
            choices.push(d);
        }
    }

    choices.sort_by_key(|f| (f.id as u64).wrapping_mul(6364136223846793005));
    choices
}

fn build_same_lemma_grammar_choices(forms: &[Form], correct: &Form, n: usize) -> Vec<Form> {
    let mut seen: HashSet<String> = HashSet::new();
    seen.insert(correct.grammar_label_ru());

    let mut distractors: Vec<Form> = forms
        .iter()
        .filter(|f| {
            if f.id == correct.id {
                return false;
            }
            if f.lemma_id != correct.lemma_id {
                return false;
            }

            let grammar = f.grammar_label_ru();
            grammar != correct.grammar_label_ru()
        })
        .cloned()
        .collect();

    distractors.sort_by_key(|f| {
        (f.id as u64).wrapping_mul(2654435761)
            ^ (correct.id as u64).wrapping_mul(1000003)
    });

    let mut choices = vec![correct.clone()];
    for distractor in distractors {
        if choices.len() >= n {
            break;
        }

        let grammar = distractor.grammar_label_ru();
        if seen.insert(grammar) {
            choices.push(distractor);
        }
    }

    choices.sort_by_key(|f| (f.id as u64).wrapping_mul(6364136223846793005));
    choices
}
