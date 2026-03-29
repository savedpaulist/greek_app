use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use js_sys;

use crate::{
    i18n::{t, UiKey},
    state::{
        settings::{GreekFont, Theme, UiLanguage, UiSize},
        AppState,
    },
};

#[component]
pub fn SettingsPage() -> Element {
    let state = use_context::<AppState>();
    let lang = state.settings.read().language.clone();
    let page_title = t(UiKey::SettingsTitle, lang);
    rsx! {
        div { class: "settings-page",
            h2 { class: "settings-page__title", "{page_title}" }
            SettingsPanel {}
        }
    }
}

#[component]
pub fn SettingsPanel() -> Element {
    let mut state = use_context::<AppState>();

    let settings_snapshot = state.settings.read().clone();
    let lang = settings_snapshot.language.clone();

    rsx! {
        div { class: "settings-panel",

            // ── Language ──────────────────────────────────────────────────
            section { class: "settings-section",
                h3 { class: "settings-section__title", "{t(UiKey::SettingsLang, lang.clone())}" }
                div { class: "lang-row",
                    span { class: "lang-row__label", "{t(UiKey::SettingsLangUi, lang.clone())}" }
                    div { class: "lang-chips",
                        for variant in [UiLanguage::Ru, UiLanguage::En] {
                            button {
                                class: if settings_snapshot.language == variant {
                                    "lang-chip lang-chip--active"
                                } else {
                                    "lang-chip"
                                },
                                onclick: {
                                    let variant = variant.clone();
                                    move |_| {
                                        state.settings.write().language = variant.clone();
                                        state.save_settings();
                                    }
                                },
                                { match variant { UiLanguage::Ru => "RU", UiLanguage::En => "EN" } }
                            }
                        }
                    }
                }
                div { class: "lang-row",
                    span { class: "lang-row__label", "{t(UiKey::SettingsLangMorph, lang.clone())}" }
                    div { class: "lang-chips",
                        for variant in [UiLanguage::Ru, UiLanguage::En] {
                            button {
                                class: if settings_snapshot.morph_language == variant {
                                    "lang-chip lang-chip--active"
                                } else {
                                    "lang-chip"
                                },
                                onclick: {
                                    let variant = variant.clone();
                                    move |_| {
                                        state.settings.write().morph_language = variant.clone();
                                        state.save_settings();
                                    }
                                },
                                { match variant { UiLanguage::Ru => "RU", UiLanguage::En => "EN" } }
                            }
                        }
                    }
                }
            }

            // ── Theme ─────────────────────────────────────────────────────
            section { class: "settings-section",
                h3 { class: "settings-section__title", "{t(UiKey::SettingsTheme, lang.clone())}" }
                div { class: "theme-nav-row",
                    button {
                        class: "btn btn--ghost btn--sm",
                        title: t(UiKey::SettingsThemePrev, lang.clone()),
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
                        title: t(UiKey::SettingsThemeNext, lang.clone()),
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
                    // Render all preset themes (everything except Custom).
                    for theme in Theme::all().iter().filter(|t| !matches!(t, Theme::Custom)) {
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

                // Custom theme button — always visible below the preset grid.
                button {
                    class: if settings_snapshot.theme == Theme::Custom {
                        "theme-chip-custom theme-chip-custom--active"
                    } else {
                        "theme-chip-custom"
                    },
                    onclick: move |_| {
                        state.settings.write().theme = Theme::Custom;
                        state.save_settings();
                    },
                    "✎ Custom"
                }

                // Colour editor — only visible when Custom is active.
                if settings_snapshot.theme == Theme::Custom {
                    CustomThemeEditor {}
                }
            }

            // ── Font ──────────────────────────────────────────────────────
            section { class: "settings-section",
                h3 { class: "settings-section__title", "{t(UiKey::SettingsFont, lang.clone())}" }
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

            // ── UI Size ───────────────────────────────────────────────────
            section { class: "settings-section",
                h3 { class: "settings-section__title", "{t(UiKey::SettingsUiSize, lang.clone())}" }
                div { class: "lang-chips",
                    for size in [UiSize::Small, UiSize::Medium, UiSize::Large] {
                        {
                            let label = match size {
                                UiSize::Small => t(UiKey::SettingsUiSizeSmall, lang.clone()),
                                UiSize::Medium => t(UiKey::SettingsUiSizeMedium, lang.clone()),
                                UiSize::Large => t(UiKey::SettingsUiSizeLarge, lang.clone()),
                            };
                            rsx! {
                                button {
                                    class: if settings_snapshot.ui_size == size {
                                        "lang-chip lang-chip--active"
                                    } else {
                                        "lang-chip"
                                    },
                                    onclick: {
                                        let size = size.clone();
                                        move |_| {
                                            state.settings.write().ui_size = size.clone();
                                            state.save_settings();
                                        }
                                    },
                                    "{label}"
                                }
                            }
                        }
                    }
                }
            }

            // ── Install app ───────────────────────────────────────────────
            InstallSection { lang: lang.clone() }

            // ── Options ───────────────────────────────────────────────────
            section { class: "settings-section",
                h3 { class: "settings-section__title", "{t(UiKey::SettingsOptions, lang.clone())}" }
                label { class: "toggle-row",
                    input {
                        r#type: "checkbox",
                        checked: settings_snapshot.ignore_diacritics,
                        onchange: move |e| {
                            state.settings.write().ignore_diacritics = e.checked();
                            state.save_settings();
                        },
                    }
                    span { "{t(UiKey::SettingsIgnoreDiacritics, lang.clone())}" }
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
                    span { "{t(UiKey::SettingsShowTransliteration, lang.clone())}" }
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
                    span { "{t(UiKey::SettingsIncludeDual, lang.clone())}" }
                }
            }
        }
    }
}

// ── Install-app section ──────────────────────────────────────────────────────

#[component]
fn InstallSection(lang: crate::state::settings::UiLanguage) -> Element {
    // Detect on mount whether the PWA install prompt is available.
    // pwaCanInstall() is exposed by pwa-install.js and returns true when:
    //   - not already running as a standalone PWA, AND
    //   - either a deferred Chrome/Android prompt is captured, or it's iOS Safari.
    let mut can_install = use_signal(|| false);

    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            let result = js_sys::eval("!!(window.pwaCanInstall && window.pwaCanInstall())")
                .ok()
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            can_install.set(result);
        }
    });

    if !can_install() {
        return rsx! {};
    }

    rsx! {
        section { class: "settings-section",
            h3 { class: "settings-section__title", "{t(UiKey::SettingsInstall, lang.clone())}" }
            p { class: "settings-help", "{t(UiKey::SettingsInstallDesc, lang.clone())}" }
            button {
                class: "btn btn--primary btn--sm",
                onclick: move |_| {
                    #[cfg(target_arch = "wasm32")]
                    {
                        let _ = js_sys::eval("window.pwaInstall && window.pwaInstall()");
                    }
                },
                "{t(UiKey::SettingsInstallBtn, lang.clone())}"
            }
        }
    }
}

// ── Custom theme colour editor ───────────────────────────────────────────────

#[component]
fn CustomThemeEditor() -> Element {
    let mut state = use_context::<AppState>();
    let snap = state.settings.read().clone();
    let lang = snap.language.clone();
    let c = snap.custom_theme.clone();

    rsx! {
        div { class: "custom-theme-editor",
            p { class: "custom-theme-editor__title", "{t(UiKey::ThemeCustomEdit, lang.clone())}" }

            ColorRow {
                label: t(UiKey::ThemeColorBg, lang.clone()).to_string(),
                value: c.bg.clone(),
                onchange: move |v| { state.settings.write().custom_theme.bg = v; state.save_settings(); },
            }
            ColorRow {
                label: t(UiKey::ThemeColorBg2, lang.clone()).to_string(),
                value: c.bg2.clone(),
                onchange: move |v| { state.settings.write().custom_theme.bg2 = v; state.save_settings(); },
            }
            ColorRow {
                label: t(UiKey::ThemeColorBg3, lang.clone()).to_string(),
                value: c.bg3.clone(),
                onchange: move |v| { state.settings.write().custom_theme.bg3 = v; state.save_settings(); },
            }
            ColorRow {
                label: t(UiKey::ThemeColorFg, lang.clone()).to_string(),
                value: c.fg.clone(),
                onchange: move |v| { state.settings.write().custom_theme.fg = v; state.save_settings(); },
            }
            ColorRow {
                label: t(UiKey::ThemeColorFg2, lang.clone()).to_string(),
                value: c.fg2.clone(),
                onchange: move |v| { state.settings.write().custom_theme.fg2 = v; state.save_settings(); },
            }
            ColorRow {
                label: t(UiKey::ThemeColorAccent, lang.clone()).to_string(),
                value: c.accent.clone(),
                onchange: move |v| { state.settings.write().custom_theme.accent = v; state.save_settings(); },
            }
            ColorRow {
                label: t(UiKey::ThemeColorAccent2, lang.clone()).to_string(),
                value: c.accent2.clone(),
                onchange: move |v| { state.settings.write().custom_theme.accent2 = v; state.save_settings(); },
            }
            ColorRow {
                label: t(UiKey::ThemeColorRed, lang.clone()).to_string(),
                value: c.red.clone(),
                onchange: move |v| { state.settings.write().custom_theme.red = v; state.save_settings(); },
            }
            ColorRow {
                label: t(UiKey::ThemeColorGreen, lang.clone()).to_string(),
                value: c.green.clone(),
                onchange: move |v| { state.settings.write().custom_theme.green = v; state.save_settings(); },
            }
            ColorRow {
                label: t(UiKey::ThemeColorBorder, lang.clone()).to_string(),
                value: c.border.clone(),
                onchange: move |v| { state.settings.write().custom_theme.border = v; state.save_settings(); },
            }
        }
    }
}

#[component]
fn ColorRow(label: String, value: String, onchange: EventHandler<String>) -> Element {
    rsx! {
        div { class: "color-row",
            label { class: "color-row__label", "{label}" }
            input {
                class: "color-row__swatch",
                r#type: "color",
                value: "{value}",
                oninput: move |e| onchange.call(e.value()),
            }
            span { class: "color-row__hex", "{value}" }
        }
    }
}
