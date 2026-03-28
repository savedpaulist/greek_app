use dioxus::prelude::*;

use crate::i18n::{t, UiKey};
use crate::logic::diacritics::{compare_greek, diff_chars, strip_leading_article};
use crate::logic::sm2::quality_from_answer;
use crate::state::AppState;

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
    let mut index = use_signal(|| 0usize);
    let mut input_value = use_signal(|| String::new());
    let mut submitted = use_signal(|| false);
    let mut is_correct = use_signal(|| false);
    // Incremented on every navigation to invalidate pending auto-advance tasks.
    let mut question_gen = use_signal(|| 0u32);

    let ignore_diacritics = state.settings.read().ignore_diacritics;
    let current_form = forms[*index.read() % total].clone();
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
                    style: "width: {(*index.read() * 100 / total)}%;",
                }
                span { class: "session-progress__label", "{*index.read() + 1}/{total}" }
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
                        if let Some(ru) = &lemma.russian {
                            p { class: "fillin-card__translation", "«{ru}»" }
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

                        if showing_wrong {
                            // Wrong: circle timer + skip button
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
                                        class: "timer-circle__fill",
                                        cx: "18", cy: "18", r: "16",
                                        stroke_width: "3",
                                    }
                                }
                                button {
                                    class: "btn btn--ghost btn--sm",
                                    onclick: move |_| {
                                        // Invalidate the pending auto-advance task.
                                        *question_gen.write() += 1;
                                        let next = (*index.read() + 1) % total;
                                        *index.write() = next;
                                        *submitted.write() = false;
                                        *input_value.write() = String::new();
                                    },
                                    "{t(UiKey::FillInSkip, lang.clone())}"
                                }
                            }
                        } else {
                            // Correct: manual Next button
                            button {
                                class: "btn btn--primary",
                                onclick: move |_| {
                                    *question_gen.write() += 1;
                                    let next = (*index.read() + 1) % total;
                                    *index.write() = next;
                                    *submitted.write() = false;
                                    *input_value.write() = String::new();
                                },
                                "{t(UiKey::FillInNext, lang.clone())}"
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
                                    *is_correct.write() = ok;
                                    *submitted.write() = true;
                                    if !ok {
                                        let gen = *question_gen.read();
                                        let mut idx = index;
                                        let mut sub = submitted;
                                        let mut inp = input_value;
                                        let mut qgen = question_gen;
                                        spawn(async move {
                                            sleep_ms(4000).await;
                                            if *qgen.read() == gen {
                                                *qgen.write() += 1;
                                                let next = (*idx.read() + 1) % total;
                                                *idx.write() = next;
                                                *sub.write() = false;
                                                *inp.write() = String::new();
                                            }
                                        });
                                    }
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
                                    *is_correct.write() = ok;
                                    *submitted.write() = true;
                                    if !ok {
                                        let gen = *question_gen.read();
                                        let mut idx = index;
                                        let mut sub = submitted;
                                        let mut inp = input_value;
                                        let mut qgen = question_gen;
                                        spawn(async move {
                                            sleep_ms(4000).await;
                                            if *qgen.read() == gen {
                                                *qgen.write() += 1;
                                                let next = (*idx.read() + 1) % total;
                                                *idx.write() = next;
                                                *sub.write() = false;
                                                *inp.write() = String::new();
                                            }
                                        });
                                    }
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
