use dioxus::prelude::*;

use crate::logic::diacritics::{compare_greek, diff_chars, strip_leading_article};
use crate::logic::sm2::quality_from_answer;
use crate::state::AppState;

/// Fill-in mode: show lemma + grammar description, user types the form.
#[component]
pub fn FillInView() -> Element {
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
    let mut index = use_signal(|| 0usize);
    let mut input_value = use_signal(|| String::new());
    let mut submitted = use_signal(|| false);
    let mut is_correct = use_signal(|| false);

    let ignore_diacritics = state.settings.read().ignore_diacritics;
    let current_form = forms[*index.read() % total].clone();
    let lemma = state.lemma_by_id(current_form.lemma_id);
    let expected = current_form.greek_form.clone();
    let expected2 = expected.clone();
    let diff = if *submitted.read() {
        // If the user answered without an article but expected has one,
        // diff against the stripped form so char highlighting is meaningful.
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

    rsx! {
        div { class: "fillin-screen",
            // Progress
            div { class: "session-progress",
                div {
                    class: "session-progress__bar",
                    style: "width: {(*index.read() * 100 / total)}%;",
                }
                span { class: "session-progress__label", "{*index.read() + 1}/{total}" }
            }

            div {
                class: {
                    let base = "fillin-card";
                    if *submitted.read() {
                        if *is_correct.read() { "fillin-card fillin-card--correct" }
                        else { "fillin-card fillin-card--wrong" }
                    } else { base }
                },
                // Prompt
                div { class: "fillin-card__prompt",
                    if let Some(lemma) = &lemma {
                        p { class: "fillin-card__lemma greek-text", "{lemma.test_prompt_greek()}" }
                        if let Some(ru) = &lemma.russian {
                            p { class: "fillin-card__translation", "«{ru}»" }
                        }
                    }
                    p { class: "fillin-card__grammar", "{current_form.grammar_label_ru()}" }
                }

                // Input area
                div { class: "fillin-card__input-area",
                    if *submitted.read() {
                        div { class: "fillin-diff",
                            for (ch, correct) in &diff {
                                span {
                                    class: if *correct { "diff-char diff-char--ok" } else { "diff-char diff-char--err" },
                                    "{ch}"
                                }
                            }
                        }
                        p { class: "fillin-answer greek-text",
                            "Правильно: {current_form.greek_form}"
                        }
                        button {
                            class: "btn btn--primary",
                            onclick: move |_| {
                                    let q = quality_from_answer(*is_correct.read(), false);
                                    state.record_answer(current_form.id, q);
                                    let next = (*index.read() + 1) % total;
                                    *index.write() = next;
                                    *submitted.write() = false;
                                    *input_value.write() = String::new();
                            },
                            "Далее →"
                        }
                    } else {
                        input {
                            class: "fillin-input greek-text",
                            r#type: "text",
                            value: "{input_value.read()}",
                            placeholder: "Введите форму…",
                            oninput: move |e| *input_value.write() = e.value(),
                            onkeydown: move |e: KeyboardEvent| {
                                    if e.key() == Key::Enter && !input_value.read().is_empty() {
                                        let ok = compare_greek(
                                            &input_value.read(),
                                            &expected,
                                            ignore_diacritics,
                                        );
                                        *is_correct.write() = ok;
                                        *submitted.write() = true;
                                    }
                                },
                        }
                        div { class: "fillin-actions",
                            button {
                                class: "btn btn--primary",
                                disabled: input_value.read().is_empty(),
                                onclick: move |_| {
                                        let ok = compare_greek(
                                            &input_value.read(),
                                            &expected2,
                                            ignore_diacritics,
                                        );
                                        *is_correct.write() = ok;
                                        *submitted.write() = true;
                                    },
                                "Проверить →"
                            }
                        }
                        // Diacritics toggle note
                        p { class: "fillin-hint",
                            if ignore_diacritics {
                                "⌨️ Диакритика игнорируется"
                            } else {
                                "⌨️ Диакритика учитывается"
                            }
                        }
                    }
                }
            }
        }
    }
}
