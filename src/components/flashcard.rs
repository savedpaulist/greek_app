use std::collections::HashSet;

use dioxus::prelude::*;

use crate::i18n::{t, UiKey};
use crate::logic::diacritics::similarity_score;
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

// ── Phase state machine ───────────────────────────────────────────────────
//
// Key invariant: choice buttons exist in the DOM ONLY during `Choosing`.
// During `Feedback`, they are fully absent — so the mobile browser has no
// element at position #N to re-focus after the DOM update.

#[derive(Clone, PartialEq)]
enum CardPhase {
    /// Showing 6 multiple-choice buttons — interactive.
    Choosing,
    /// Brief feedback panel (NO choice buttons in DOM) — auto-advances.
    Feedback { is_correct: bool },
}

// ── FlashcardView ─────────────────────────────────────────────────────────

/// Flashcard: show lemma + grammar prompt, user picks the correct form.
#[component]
pub fn FlashcardView(reverse: bool) -> Element {
    let mut state = use_context::<AppState>();
    let settings_snap = state.settings.read().clone();
    let lang = settings_snap.language.clone();
    let morph_lang = settings_snap.morph_language.clone();
    let forms = state.filtered_forms();

    if forms.is_empty() {
        return rsx! {
            div { class: "empty-state",
                p { "{t(UiKey::EmptyNoForms, lang.clone())}" }
            }
        };
    }

    let total = forms.len();
    let session_count = total.min(SESSION_SIZE);
    let mut phase = use_signal(|| CardPhase::Choosing);
    let mut index = use_signal(|| 0usize);
    let mut session_correct: Signal<u32> = use_signal(|| 0);
    let mut shuffle_seed = use_signal(fresh_shuffle_seed);
    // Incremented each time a wrong-answer feedback starts or is skipped.
    // Spawned tasks capture this value and no-op if it has changed.
    let mut feedback_gen: Signal<u32> = use_signal(|| 0);

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
                        *phase.write() = CardPhase::Choosing;
                        *shuffle_seed.write() = fresh_shuffle_seed();
                    },
                    "{t(UiKey::FlashcardRetry, lang.clone())}"
                }
            }
        };
    }

    let real_idx = order[*index.read() % total];
    let current_form = forms[real_idx].clone();
    let current_lemma = state.lemma_by_id(current_form.lemma_id);

    // The text that represents "the correct answer" shown in feedback / reveal.
    let answer_text = if reverse {
        current_form.test_prompt_greek()
    } else {
        current_form.grammar_label(&morph_lang)
    };

    let form_choices = if reverse {
        build_same_lemma_choices(&forms, &current_form, 6)
    } else {
        build_same_lemma_grammar_choices(&forms, &current_form, 6)
    };

    // Pre-read phase so we can branch without holding the signal guard.
    let phase_val = phase.read().clone();
    let feedback_correct: Option<bool> = match &phase_val {
        CardPhase::Feedback { is_correct } => Some(*is_correct),
        _ => None,
    };
    let is_choosing = matches!(phase_val, CardPhase::Choosing);

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

                // Prompt — always visible, independent of phase.
                div { class: "flashcard__prompt",
                    if reverse {
                        if let Some(lemma) = &current_lemma {
                            {
                                let tr = lemma.translation(&lang);
                                if !tr.is_empty() {
                                    rsx! { p { class: "flashcard__translation", "«{tr}»" } }
                                } else { rsx! {} }
                            }
                        }
                        p { class: "flashcard__grammar", "{current_form.grammar_label(&morph_lang)}" }
                    } else {
                        p { class: "flashcard__lemma greek-text", "{current_form.test_prompt_greek()}" }
                        if let Some(lemma) = &current_lemma {
                            {
                                let tr = lemma.translation(&lang);
                                if !tr.is_empty() {
                                    rsx! { p { class: "flashcard__translation", "«{tr}»" } }
                                } else { rsx! {} }
                            }
                        }
                    }
                }

                // Answer area — switches between three mutually-exclusive phases.
                div { class: "flashcard__answers",

                    // ── CHOOSING: 6 mc-choice buttons ─────────────────────────
                    if is_choosing {
                        div { class: "mc-choices",
                            for choice in form_choices {
                                {
                                    let is_correct_choice = choice.id == current_form.id;
                                    let choice_text = if reverse {
                                        choice.test_prompt_greek()
                                    } else {
                                        choice.grammar_label(&morph_lang)
                                    };
                                    rsx! {
                                        button {
                                            class: "mc-choice",
                                            onclick: move |_| {
                                                // Guard: only fire once per question.
                                                if !matches!(*phase.read(), CardPhase::Choosing) {
                                                    return;
                                                }
                                                let q = quality_from_answer(is_correct_choice, false);
                                                state.record_answer(current_form.id, q);
                                                if is_correct_choice {
                                                    *session_correct.write() += 1;
                                                }
                                                // Transition to Feedback — choice buttons leave DOM.
                                                *phase.write() = CardPhase::Feedback {
                                                    is_correct: is_correct_choice,
                                                };
                                                let delay = if is_correct_choice { 800_i32 } else { 4000_i32 };
                                                let mut ph = phase;
                                                let mut idx = index;
                                                if !is_correct_choice {
                                                    *feedback_gen.write() += 1;
                                                }
                                                let gen = *feedback_gen.read();
                                                let mut fgen = feedback_gen;
                                                spawn(async move {
                                                    sleep_ms(delay).await;
                                                    // Only advance if this feedback hasn't been skipped.
                                                    if *fgen.read() == gen {
                                                        let next = *idx.read() + 1;
                                                        *idx.write() = next;
                                                        *ph.write() = CardPhase::Choosing;
                                                    }
                                                });
                                            },
                                            if reverse {
                                                span { class: "greek-text", "{choice_text}" }
                                            } else {
                                                span { class: "mc-choice__grammar", "{choice_text}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // ── FEEDBACK: no choice buttons in DOM ────────────────────
                    if let Some(is_correct) = feedback_correct {
                        div {
                            class: if is_correct {
                                "feedback-panel feedback-panel--correct"
                            } else {
                                "feedback-panel feedback-panel--wrong"
                            },
                            span { class: "feedback-panel__icon",
                                if is_correct { "✓" } else { "✗" }
                            }
                            p { class: "feedback-panel__answer greek-text", "{answer_text}" }
                            if !is_correct {
                                div { class: "fillin-auto-advance",
                                    svg {
                                        class: "timer-circle",
                                        view_box: "0 0 36 36",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        circle { class: "timer-circle__track", cx: "18", cy: "18", r: "16", stroke_width: "3" }
                                        circle { class: "timer-circle__fill",  cx: "18", cy: "18", r: "16", stroke_width: "3" }
                                    }
                                    button {
                                        class: "btn btn--ghost btn--sm",
                                        onclick: move |_| {
                                            // Invalidate pending spawn before advancing.
                                            *feedback_gen.write() += 1;
                                            let next = *index.read() + 1;
                                            *index.write() = next;
                                            *phase.write() = CardPhase::Choosing;
                                        },
                                        "{t(UiKey::FillInSkip, lang.clone())}"
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
/// Distractors are ranked by *spelling similarity* to `correct` — first come
/// forms that differ only by accents, then by other diacritics, then by 1
/// letter, 2 letters, and so on. Same-lemma forms are always preferred; if
/// the same-lemma pool is exhausted we fall back to other forms across the
/// whole `forms` slice so the learner still sees `n` options.
fn build_same_lemma_choices(forms: &[Form], correct: &Form, n: usize) -> Vec<Form> {
    let mut seen: HashSet<String> = HashSet::new();
    seen.insert(correct.greek_form.clone());

    let rank = |a: &Form, b: &Form| {
        let sa = similarity_score(&a.greek_form, &correct.greek_form);
        let sb = similarity_score(&b.greek_form, &correct.greek_form);
        sa.cmp(&sb).then_with(|| a.id.cmp(&b.id))
    };

    let mut same_lemma: Vec<Form> = forms
        .iter()
        .filter(|f| {
            f.id != correct.id
                && f.lemma_id == correct.lemma_id
                && f.greek_form != correct.greek_form
        })
        .cloned()
        .collect();
    same_lemma.sort_by(rank);

    let mut choices = vec![correct.clone()];
    for d in &same_lemma {
        if choices.len() >= n {
            break;
        }
        if seen.insert(d.greek_form.clone()) {
            choices.push(d.clone());
        }
    }

    if choices.len() < n {
        let mut fallback: Vec<Form> = forms
            .iter()
            .filter(|f| {
                f.id != correct.id
                    && f.lemma_id != correct.lemma_id
                    && f.greek_form != correct.greek_form
            })
            .cloned()
            .collect();
        fallback.sort_by(rank);
        for d in fallback {
            if choices.len() >= n {
                break;
            }
            if seen.insert(d.greek_form.clone()) {
                choices.push(d);
            }
        }
    }

    choices.sort_by_key(|f| (f.id as u64).wrapping_mul(6364136223846793005));
    choices
}

/// Count how many grammatical-tag attributes differ between two forms.
/// Lower = more similar grammar (useful for ranking distractors in the
/// forward-mode "pick the right grammar label" quiz).
fn grammar_tag_distance(a: &Form, b: &Form) -> usize {
    let pairs: [(&Option<String>, &Option<String>); 8] = [
        (&a.case_tag, &b.case_tag),
        (&a.number_tag, &b.number_tag),
        (&a.gender_tag, &b.gender_tag),
        (&a.tense_tag, &b.tense_tag),
        (&a.voice_tag, &b.voice_tag),
        (&a.mood_tag, &b.mood_tag),
        (&a.person_tag, &b.person_tag),
        (&a.degree_tag, &b.degree_tag),
    ];
    pairs.iter().filter(|(x, y)| x != y).count()
}

/// Pick `n` forms whose *grammar labels* are distinct from `correct`, ranked
/// by grammar-tag similarity (fewer differing tags first). Falls back to the
/// whole pool if the same-lemma pool runs dry.
fn build_same_lemma_grammar_choices(forms: &[Form], correct: &Form, n: usize) -> Vec<Form> {
    let mut seen: HashSet<String> = HashSet::new();
    seen.insert(correct.grammar_label_ru());

    let rank = |a: &Form, b: &Form| {
        let da = grammar_tag_distance(a, correct);
        let db = grammar_tag_distance(b, correct);
        da.cmp(&db).then_with(|| a.id.cmp(&b.id))
    };

    let mut same_lemma: Vec<Form> = forms
        .iter()
        .filter(|f| {
            f.id != correct.id
                && f.lemma_id == correct.lemma_id
                && f.grammar_label_ru() != correct.grammar_label_ru()
        })
        .cloned()
        .collect();
    same_lemma.sort_by(rank);

    let mut choices = vec![correct.clone()];
    for d in &same_lemma {
        if choices.len() >= n {
            break;
        }
        let grammar = d.grammar_label_ru();
        if seen.insert(grammar) {
            choices.push(d.clone());
        }
    }

    if choices.len() < n {
        let mut fallback: Vec<Form> = forms
            .iter()
            .filter(|f| {
                f.id != correct.id
                    && f.lemma_id != correct.lemma_id
                    && f.pos == correct.pos
                    && f.grammar_label_ru() != correct.grammar_label_ru()
            })
            .cloned()
            .collect();
        fallback.sort_by(rank);
        for d in fallback {
            if choices.len() >= n {
                break;
            }
            let grammar = d.grammar_label_ru();
            if seen.insert(grammar) {
                choices.push(d);
            }
        }
    }

    choices.sort_by_key(|f| (f.id as u64).wrapping_mul(6364136223846793005));
    choices
}
