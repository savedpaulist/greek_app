use dioxus::prelude::*;

use crate::i18n::{t, UiKey};
use crate::logic::diacritics::{compare_greek, diff_chars, strip_leading_article};
use crate::logic::sm2::quality_from_answer;
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

// ── FillInView ────────────────────────────────────────────────────────────

/// Fill-in mode: show lemma + grammar description, user types the form.
#[component]
pub fn FillInView() -> Element {
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
    let shuffled = use_signal(|| crate::logic::shuffled_indices(total));
    let mut index = use_signal(|| 0usize);
    let mut session_correct: Signal<u32> = use_signal(|| 0);
    let mut input_value = use_signal(|| String::new());
    let mut submitted = use_signal(|| false);
    let mut is_correct = use_signal(|| false);
    // Incremented on every navigation to invalidate pending auto-advance tasks.
    let mut question_gen = use_signal(|| 0u32);

    // ── Session results screen ────────────────────────────────────────────
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
                        *submitted.write() = false;
                        *input_value.write() = String::new();
                        *question_gen.write() += 1;
                    },
                    "{t(UiKey::FlashcardRetry, lang.clone())}"
                }
            }
        };
    }

    let ignore_diacritics = state.settings.read().ignore_diacritics;
    let current_form = forms[shuffled.read()[*index.read()]].clone();
    let lemma = state.lemma_by_id(current_form.lemma_id);
    let expected = current_form.greek_form.clone();
    let expected2 = expected.clone();

    let diff = if *submitted.read() {
        let answer = input_value.read();
        let answer_str: &str = &*answer;
        let user_has_article = strip_leading_article(answer_str.trim()) != answer_str.trim();
        let diff_target = if !user_has_article {
            strip_leading_article(expected.trim()).to_string()
        } else {
            expected.clone()
        };
        diff_chars(answer_str, &diff_target, ignore_diacritics)
    } else {
        vec![]
    };

    let showing_wrong = *submitted.read() && !*is_correct.read();

    rsx! {
        div { class: "fillin-screen",
            // Progress bar
            div { class: "session-progress",
                div {
                    class: "session-progress__bar",
                    style: "width: {(*index.read() * 100 / session_count)}%;",
                }
                span { class: "session-progress__label", "{*index.read() + 1}/{session_count}" }
            }

            div {
                class: if *submitted.read() {
                    if *is_correct.read() { "fillin-card fillin-card--correct" }
                    else { "fillin-card fillin-card--wrong" }
                } else { "fillin-card" },

                // Prompt
                div { class: "fillin-card__prompt",
                    if let Some(lemma) = &lemma {
                        p { class: "fillin-card__lemma greek-text", "{lemma.test_prompt_greek()}" }
                        {
                            let tr = lemma.translation(&lang);
                            if !tr.is_empty() {
                                rsx! { p { class: "fillin-card__translation", "«{tr}»" } }
                            } else { rsx! {} }
                        }
                    }
                    p { class: "fillin-card__grammar", "{current_form.grammar_label(&morph_lang)}" }
                }

                // Input / result area
                div { class: "fillin-card__input-area",
                    if *submitted.read() {
                        // Diff display
                        div { class: "fillin-diff",
                            for (ch, correct) in &diff {
                                span {
                                    class: if *correct { "diff-char diff-char--ok" } else { "diff-char diff-char--err" },
                                    "{ch}"
                                }
                            }
                        }
                        p { class: "fillin-answer greek-text",
                            "{t(UiKey::FillInAnswer, lang.clone())}: {current_form.greek_form}"
                        }

                        // Both correct and wrong show a timer circle + skip button.
                        // Correct: green, 800 ms. Wrong: red, 4 000 ms.
                        div { class: "fillin-auto-advance",
                            svg {
                                class: "timer-circle",
                                view_box: "0 0 36 36",
                                xmlns: "http://www.w3.org/2000/svg",
                                circle {
                                    class: "timer-circle__track",
                                    cx: "18", cy: "18", r: "16",
                                    stroke_width: "3",
                                }
                                circle {
                                    class: if showing_wrong {
                                        "timer-circle__fill"
                                    } else {
                                        "timer-circle__fill timer-circle__fill--correct"
                                    },
                                    cx: "18", cy: "18", r: "16",
                                    stroke_width: "3",
                                }
                            }
                            button {
                                class: "btn btn--ghost btn--sm",
                                onclick: move |_| {
                                    *question_gen.write() += 1;
                                    *index.write() += 1;
                                    *submitted.write() = false;
                                    *input_value.write() = String::new();
                                },
                                "{t(UiKey::FillInSkip, lang.clone())}"
                            }
                        }
                    } else {
                        // Input phase
                        input {
                            class: "fillin-input greek-text",
                            r#type: "text",
                            value: "{input_value.read()}",
                            placeholder: t(UiKey::FillInPlaceholder, lang.clone()),
                            oninput: move |e| *input_value.write() = e.value(),
                            onkeydown: move |e: KeyboardEvent| {
                                if e.key() == Key::Enter && !input_value.read().is_empty() {
                                    let ok = compare_greek(&input_value.read(), &expected, ignore_diacritics);
                                    let q = quality_from_answer(ok, false);
                                    state.record_answer(current_form.id, q);
                                    if ok { *session_correct.write() += 1; }
                                    *is_correct.write() = ok;
                                    *submitted.write() = true;
                                    let delay = if ok { 800_i32 } else { 4000_i32 };
                                    let gen = *question_gen.read();
                                    let mut idx = index;
                                    let mut sub = submitted;
                                    let mut inp = input_value;
                                    let mut qgen = question_gen;
                                    spawn(async move {
                                        sleep_ms(delay).await;
                                        if *qgen.read() == gen {
                                            *qgen.write() += 1;
                                            *idx.write() += 1;
                                            *sub.write() = false;
                                            *inp.write() = String::new();
                                        }
                                    });
                                }
                            },
                        }
                        div { class: "fillin-actions",
                            button {
                                class: "btn btn--primary",
                                disabled: input_value.read().is_empty(),
                                onclick: move |_| {
                                    let ok = compare_greek(&input_value.read(), &expected2, ignore_diacritics);
                                    let q = quality_from_answer(ok, false);
                                    state.record_answer(current_form.id, q);
                                    if ok { *session_correct.write() += 1; }
                                    *is_correct.write() = ok;
                                    *submitted.write() = true;
                                    let delay = if ok { 800_i32 } else { 4000_i32 };
                                    let gen = *question_gen.read();
                                    let mut idx = index;
                                    let mut sub = submitted;
                                    let mut inp = input_value;
                                    let mut qgen = question_gen;
                                    spawn(async move {
                                        sleep_ms(delay).await;
                                        if *qgen.read() == gen {
                                            *qgen.write() += 1;
                                            *idx.write() += 1;
                                            *sub.write() = false;
                                            *inp.write() = String::new();
                                        }
                                    });
                                },
                                "{t(UiKey::FillInSubmit, lang.clone())}"
                            }
                        }
                        p { class: "fillin-hint",
                            if ignore_diacritics {
                                "{t(UiKey::FillInHintDiacriticsOff, lang.clone())}"
                            } else {
                                "{t(UiKey::FillInHintDiacriticsOn, lang.clone())}"
                            }
                        }
                    }
                }
            }
        }
    }
}
