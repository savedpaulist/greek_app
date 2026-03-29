use crate::{i18n::{t, UiKey}, router::Route, state::{AppState, settings::Theme}};
use dioxus::prelude::*;

fn is_overlay_layout() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window()
            .and_then(|w| w.inner_width().ok())
            .and_then(|v| v.as_f64())
            .map(|w| w < 1400.0) // If less than 1400, space is tight, act as overlay
            .unwrap_or(true)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        false
    }
}

/// Top-level app shell: TopBar + Sidebar overlay + page content via Outlet.
#[component]
pub fn Shell() -> Element {
    let mut state = use_context::<AppState>();
    let settings = state.settings.read();
    let theme = settings.theme.data_attr();
    let greek_font = settings.greek_font.css_family();
    let size_css = format!("html{{font-size:{}px;}}", settings.ui_size.font_size_px());
    // Build a <style> block containing the custom palette when Custom is active.
    // Using a child <style> element (instead of inline vars on the root div) ensures
    // that Dioxus removes it from the DOM cleanly when switching to a preset theme,
    // so no stale CSS variables linger.
    let custom_css = if settings.theme == Theme::Custom {
        let c = &settings.custom_theme;
        format!(
            "[data-theme='custom']{{--bg:{bg};--bg2:{bg2};--bg3:{bg3};\
             --fg:{fg};--fg2:{fg2};\
             --accent:{ac};--accent2:{ac2};\
             --red:{red};--green:{green};--border:{border};\
             --blue:#458588;--orange:{ac};--purple:{ac2};\
             --shadow:rgba(0,0,0,0.15);\
             --correct:{green};--wrong:{red};\
             --radius:12px;--radius-sm:8px;}}",
            bg = c.bg, bg2 = c.bg2, bg3 = c.bg3,
            fg = c.fg, fg2 = c.fg2,
            ac = c.accent, ac2 = c.accent2,
            red = c.red, green = c.green, border = c.border,
        )
    } else {
        String::new()
    };
    drop(settings);

    let filters_open = *state.filters_open.read();
    let settings_open = *state.settings_open.read();

    rsx! {
        // Inject custom theme CSS into the document via a <style> element.
        // When the user switches back to a preset theme, Dioxus removes this
        // element and the [data-theme="custom"] rules vanish immediately.
        style { "{size_css}" }
        if !custom_css.is_empty() {
            style { "{custom_css}" }
        }
        div {
            "data-theme": theme,
            "data-filters-open": "{filters_open}",
            "data-settings-open": "{settings_open}",
            style: "--greek-font: {greek_font};",
            class: "app-shell",
            // Hidden swipe control buttons — clicked programmatically by swipe.js
            button {
                id: "swipe-open-filters",
                style: "display:none;",
                onclick: move |_| {
                    *state.filters_open.write() = true;
                    if is_overlay_layout() { *state.settings_open.write() = false; }
                }
            }
            button {
                id: "swipe-close-filters",
                style: "display:none;",
                onclick: move |_| { *state.filters_open.write() = false; }
            }
            button {
                id: "swipe-open-settings",
                style: "display:none;",
                onclick: move |_| {
                    *state.settings_open.write() = true;
                    if is_overlay_layout() { *state.filters_open.write() = false; }
                }
            }
            button {
                id: "swipe-close-settings",
                style: "display:none;",
                onclick: move |_| { *state.settings_open.write() = false; }
            }
            TopBar {}
            crate::components::sidebar::Sidebar {}
            main {
                class: "page-content",
                Outlet::<Route> {}
            }
        }
    }
}

// ── TopBar ──────────────────────────────────────────────────────────────────

#[component]
pub fn TopBar() -> Element {
    let mut state = use_context::<AppState>();
    let lang = state.settings.read().language.clone();

    rsx! {
        header { class: "topbar",
            div { class: "topbar__inner",
                // Hamburger / filter toggle
                button {
                    class: "topbar__menu-btn",
                    onclick: move |_| {
                        let mut filters_open = state.filters_open.write();
                        *filters_open = !*filters_open;
                        if *filters_open && is_overlay_layout() {
                            *state.settings_open.write() = false;
                        }
                    },
                    // Hamburger icon
                    svg { xmlns: "http://www.w3.org/2000/svg", width: "20", height: "20", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                        line { x1: "3", y1: "6", x2: "21", y2: "6" }
                        line { x1: "3", y1: "12", x2: "21", y2: "12" }
                        line { x1: "3", y1: "18", x2: "21", y2: "18" }
                    }
                }
                // App name
                Link { to: Route::Home {}, class: "topbar__title", "{t(UiKey::TopbarHome, lang.clone())}" }
                // Right actions
                nav { class: "topbar__actions",
                    Link { to: Route::Progress {}, class: "topbar__icon-btn", title: t(UiKey::TopbarProgress, lang.clone()),
                        svg { xmlns: "http://www.w3.org/2000/svg", width: "20", height: "20", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                            line { x1: "18", y1: "20", x2: "18", y2: "10" }
                            line { x1: "12", y1: "20", x2: "12", y2: "4" }
                            line { x1: "6", y1: "20", x2: "6", y2: "14" }
                        }
                    }
                    button {
                        class: "topbar__icon-btn",
                        title: t(UiKey::TopbarSettings, lang.clone()),
                        onclick: move |_| {
                            let mut settings_open = state.settings_open.write();
                            *settings_open = !*settings_open;
                            if *settings_open && is_overlay_layout() {
                                *state.filters_open.write() = false;
                            }
                        },
                        svg { xmlns: "http://www.w3.org/2000/svg", width: "20", height: "20", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                            circle { cx: "12", cy: "12", r: "3" }
                            path { d: "M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z" }
                        }
                    }
                }
            }
        }
    }
}

// ── Mode picker chips ───────────────────────────────────────────────────────

#[component]
pub fn ModeChip(label: String, icon: String, to: Route) -> Element {
    rsx! {
        Link { to, class: "mode-chip",
            span { class: "mode-chip__icon", "{icon}" }
            span { class: "mode-chip__label", "{label}" }
        }
    }
}
