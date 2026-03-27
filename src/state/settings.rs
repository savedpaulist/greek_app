use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Theme {
    GruvboxLight,
    GruvboxDark,
    DraculaLight,
    DraculaDark,
    NordLight,
    NordDark,
    Base16Darkmoss,
    Base16Everforest,
    Base16RosePine,
    Base16TokyoNight,
    Base16Kanagawa,
    PinkPastelLight,
    PinkPastelDark,
    SeriousBlueLight,
    SeriousBlueDark,
}

impl Theme {
    pub fn data_attr(&self) -> &'static str {
        match self {
            Theme::GruvboxLight => "gruvbox-light",
            Theme::GruvboxDark => "gruvbox-dark",
            Theme::DraculaLight => "dracula-light",
            Theme::DraculaDark => "dracula-dark",
            Theme::NordLight => "nord-light",
            Theme::NordDark => "nord-dark",
            Theme::Base16Darkmoss => "base16-darkmoss",
            Theme::Base16Everforest => "base16-everforest",
            Theme::Base16RosePine => "base16-rose-pine",
            Theme::Base16TokyoNight => "base16-tokyo-night",
            Theme::Base16Kanagawa => "base16-kanagawa",
            Theme::PinkPastelLight => "catppuccin-latte",
            Theme::PinkPastelDark => "catppuccin-mocha",
            Theme::SeriousBlueLight => "solarized-light",
            Theme::SeriousBlueDark => "solarized-dark",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Theme::GruvboxLight => "Gruvbox Light",
            Theme::GruvboxDark => "Gruvbox Dark",
            Theme::DraculaLight => "Dracula Light",
            Theme::DraculaDark => "Dracula Dark",
            Theme::NordLight => "Nord Light",
            Theme::NordDark => "Nord Dark",
            Theme::Base16Darkmoss => "Darkmoss",
            Theme::Base16Everforest => "Everforest Dark",
            Theme::Base16RosePine => "Rosé Pine",
            Theme::Base16TokyoNight => "Tokyo Night",
            Theme::Base16Kanagawa => "Kanagawa",
            Theme::PinkPastelLight => "Catppuccin Latte",
            Theme::PinkPastelDark => "Catppuccin Mocha",
            Theme::SeriousBlueLight => "Solarized Light",
            Theme::SeriousBlueDark => "Solarized Dark",
        }
    }

    pub fn all() -> &'static [Theme] {
        &[
            Theme::GruvboxLight,
            Theme::GruvboxDark,
            Theme::DraculaLight,
            Theme::DraculaDark,
            Theme::NordLight,
            Theme::NordDark,
            Theme::Base16Darkmoss,
            Theme::Base16Everforest,
            Theme::Base16RosePine,
            Theme::Base16TokyoNight,
            Theme::Base16Kanagawa,
            Theme::PinkPastelLight,
            Theme::PinkPastelDark,
            Theme::SeriousBlueLight,
            Theme::SeriousBlueDark,
        ]
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme::GruvboxLight
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum GreekFont {
    #[default]
    GfsDidot,
    GfsNeohellenic,
    NotoSerifGreek,
    Cardo,
    GentiumPlus,
}

impl GreekFont {
    pub fn css_family(&self) -> &'static str {
        match self {
            GreekFont::GfsDidot => "'Noto Serif', serif",
            GreekFont::GfsNeohellenic => "'Noto Sans', sans-serif",
            GreekFont::NotoSerifGreek => "'Literata', serif",
            GreekFont::Cardo => "'Lora', serif",
            GreekFont::GentiumPlus => "'Crimson Pro', serif",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            GreekFont::GfsDidot => "Noto Serif",
            GreekFont::GfsNeohellenic => "Noto Sans",
            GreekFont::NotoSerifGreek => "Literata",
            GreekFont::Cardo => "Lora",
            GreekFont::GentiumPlus => "Crimson Pro",
        }
    }

    pub fn all() -> &'static [GreekFont] {
        &[
            GreekFont::GfsDidot,
            GreekFont::GfsNeohellenic,
            GreekFont::NotoSerifGreek,
            GreekFont::Cardo,
            GreekFont::GentiumPlus,
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum UiLanguage {
    #[default]
    Ru,
    En,
}

/// Persistent user settings, saved to localStorage.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub theme: Theme,
    pub greek_font: GreekFont,
    pub language: UiLanguage,
    pub ignore_diacritics: bool,
    pub has_diacritic_keyboard: bool,
    pub show_transliteration: bool,
    pub show_translation: bool,
    pub include_dual: bool,
    /// First launch done (onboarding seen).
    pub onboarding_done: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: Theme::GruvboxLight,
            greek_font: GreekFont::GfsDidot,
            language: UiLanguage::Ru,
            ignore_diacritics: false,
            has_diacritic_keyboard: false,
            show_transliteration: false,
            show_translation: true,
            include_dual: false,
            onboarding_done: false,
        }
    }
}
