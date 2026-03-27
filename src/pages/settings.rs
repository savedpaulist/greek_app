use dioxus::prelude::*;

use crate::state::{
    settings::{GreekFont, Theme},
    AppState,
};

#[component]
pub fn SettingsPage() -> Element {
    rsx! {
        div { class: "settings-page",
            h2 { class: "settings-page__title", "Настройки" }
            SettingsPanel {}
        }
    }
}

#[component]
pub fn SettingsPanel() -> Element {
    let mut state = use_context::<AppState>();

    let settings_snapshot = state.settings.read().clone();

    rsx! {
        div { class: "settings-panel",
            section { class: "settings-section",
                h3 { class: "settings-section__title", "Тема" }
                div { class: "theme-nav-row",
                    button {
                        class: "btn btn--ghost btn--sm",
                        title: "Предыдущая тема",
                        onclick: move |_| {
                            let mut settings = state.settings.write();
                            let all = Theme::all();
                            let idx = all.iter().position(|t| t == &settings.theme).unwrap_or(0);
                            let prev_idx = if idx == 0 { all.len() - 1 } else { idx - 1 };
                            settings.theme = all[prev_idx].clone();
                            drop(settings);
                            state.save_settings();
                        },
                        svg { xmlns: "http://www.w3.org/2000/svg", width: "16", height: "16", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                            polyline { points: "15 18 9 12 15 6" }
                        }
                    }
                    span { class: "theme-nav-row__label", "{settings_snapshot.theme.label()}" }
                    button {
                        class: "btn btn--ghost btn--sm",
                        title: "Следующая тема",
                        onclick: move |_| {
                            let mut settings = state.settings.write();
                            let all = Theme::all();
                            let idx = all.iter().position(|t| t == &settings.theme).unwrap_or(0);
                            let next_idx = (idx + 1) % all.len();
                            settings.theme = all[next_idx].clone();
                            drop(settings);
                            state.save_settings();
                        },
                        svg { xmlns: "http://www.w3.org/2000/svg", width: "16", height: "16", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                            polyline { points: "9 18 15 12 9 6" }
                        }
                    }
                }
                div { class: "theme-grid",
                    for theme in Theme::all() {
                        button {
                            class: if settings_snapshot.theme == *theme {
                                "theme-chip theme-chip--active"
                            } else {
                                "theme-chip"
                            },
                            "data-theme": theme.data_attr(),
                            onclick: {
                                let theme = theme.clone();
                                move |_| {
                                    state.settings.write().theme = theme.clone();
                                    state.save_settings();
                                }
                            },
                            "{theme.label()}"
                        }
                    }
                }
            }

            section { class: "settings-section",
                h3 { class: "settings-section__title", "Шрифт для греческого" }
                div { class: "font-list",
                    for font in GreekFont::all() {
                        button {
                            class: if settings_snapshot.greek_font == *font {
                                "font-chip font-chip--active"
                            } else {
                                "font-chip"
                            },
                            style: "font-family: {font.css_family()};",
                            onclick: {
                                let font = font.clone();
                                move |_| {
                                    state.settings.write().greek_font = font.clone();
                                    state.save_settings();
                                }
                            },
                            "αβγδεζ — {font.label()}"
                        }
                    }
                }
            }

            section { class: "settings-section",
                h3 { class: "settings-section__title", "Опции" }
                label { class: "toggle-row",
                    input {
                        r#type: "checkbox",
                        checked: settings_snapshot.ignore_diacritics,
                        onchange: move |e| {
                            state.settings.write().ignore_diacritics = e.checked();
                            state.save_settings();
                        },
                    }
                    span { "Игнорировать диакритику при проверке" }
                }
                label { class: "toggle-row",
                    input {
                        r#type: "checkbox",
                        checked: settings_snapshot.show_transliteration,
                        onchange: move |e| {
                            state.settings.write().show_transliteration = e.checked();
                            state.save_settings();
                        },
                    }
                    span { "Показывать транслитерацию" }
                }
                label { class: "toggle-row",
                    input {
                        r#type: "checkbox",
                        checked: settings_snapshot.include_dual,
                        onchange: move |e| {
                            state.settings.write().include_dual = e.checked();
                            state.save_settings();
                        },
                    }
                    span { "Показывать двойственное число в таблицах" }
                }
            }
        }
    }
}

