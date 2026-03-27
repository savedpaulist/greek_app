//! Internationalisation support.
//!
//! All UI strings are keyed here. To add a new language:
//!   1. Add a variant to `UiLanguage` in `state/settings.rs`
//!   2. Add a matching arm to `UiKey::en()` / create a new `fn XX()` method below
//!   3. Add the variant to the `t()` match
//!
//! Grammar-form labels (tense, case, person…) live in `models/form.rs` and are
//! intentionally kept separate because they are data-model labels, not UI strings.

use crate::state::settings::UiLanguage;

/// Get a translated UI string for the given key and language.
#[allow(dead_code)]
pub fn t(key: UiKey, lang: UiLanguage) -> &'static str {
    match lang {
        UiLanguage::Ru => key.ru(),
        UiLanguage::En => key.en(),
    }
}

/// Every translatable string in the app UI.
#[allow(dead_code)]
pub enum UiKey {
    // ── App shell ────────────────────────────────────────────────────────────
    AppTitle,
    // ── Settings ─────────────────────────────────────────────────────────────
    SettingsTitle,
    SettingsTheme,
    SettingsFont,
    SettingsOptions,
    SettingsIgnoreDiacritics,
    SettingsShowTransliteration,
    SettingsIncludeDual,
    // ── Filters ──────────────────────────────────────────────────────────────
    FiltersTitle,
    FiltersPos,
    FiltersTense,
    FiltersPerson,
    FiltersVoice,
    FiltersMood,
    FiltersCase,
    FiltersNumber,
    FiltersWords,
    FiltersReset,
    FiltersSearch,
    // ── Paradigm builder ─────────────────────────────────────────────────────
    BuilderTitle,
    BuilderSavedNominal,
    BuilderSavedVerb,
    BuilderLemmaField,
    BuilderTranslationField,
    BuilderSaveBtn,
    // ── Study modes ───────────────────────────────────────────────────────────
    ModeParadigm,
    ModeFlashcard,
    ModeFlashcardRev,
    ModeFillIn,
    // ── Progress ─────────────────────────────────────────────────────────────
    ProgressTitle,
    // ── Generic ──────────────────────────────────────────────────────────────
    Back,
    Reset,
    Delete,
}

impl UiKey {
    fn ru(&self) -> &'static str {
        match self {
            UiKey::AppTitle => "σφόδρα",
            UiKey::SettingsTitle => "Настройки",
            UiKey::SettingsTheme => "Тема",
            UiKey::SettingsFont => "Шрифт для греческого",
            UiKey::SettingsOptions => "Опции",
            UiKey::SettingsIgnoreDiacritics => "Игнорировать диакритику при проверке",
            UiKey::SettingsShowTransliteration => "Показывать транслитерацию",
            UiKey::SettingsIncludeDual => "Показывать двойственное число в таблицах",
            UiKey::FiltersTitle => "Фильтры",
            UiKey::FiltersPos => "Часть речи",
            UiKey::FiltersTense => "Время",
            UiKey::FiltersPerson => "Лицо",
            UiKey::FiltersVoice => "Залог",
            UiKey::FiltersMood => "Наклонение",
            UiKey::FiltersCase => "Падеж",
            UiKey::FiltersNumber => "Число",
            UiKey::FiltersWords => "Конкретные слова",
            UiKey::FiltersReset => "Сбросить фильтры",
            UiKey::FiltersSearch => "Поиск по слову или переводу…",
            UiKey::BuilderTitle => "Создать парадигму",
            UiKey::BuilderSavedNominal => "Парадигма сохранена и уже участвует в фильтрах и тренировках.",
            UiKey::BuilderSavedVerb => "Глагольная парадигма сохранена и уже участвует в фильтрах и тренировках.",
            UiKey::BuilderLemmaField => "Лемма",
            UiKey::BuilderTranslationField => "Перевод / помета",
            UiKey::BuilderSaveBtn => "Сохранить парадигму",
            UiKey::ModeParadigm => "Просмотр парадигм",
            UiKey::ModeFlashcard => "Карточки",
            UiKey::ModeFlashcardRev => "Обратные карточки",
            UiKey::ModeFillIn => "Вписать форму",
            UiKey::ProgressTitle => "Прогресс",
            UiKey::Back => "← Назад",
            UiKey::Reset => "Сбросить",
            UiKey::Delete => "Удалить",
        }
    }

    fn en(&self) -> &'static str {
        match self {
            UiKey::AppTitle => "σφόδρα",
            UiKey::SettingsTitle => "Settings",
            UiKey::SettingsTheme => "Theme",
            UiKey::SettingsFont => "Greek font",
            UiKey::SettingsOptions => "Options",
            UiKey::SettingsIgnoreDiacritics => "Ignore diacritics when checking",
            UiKey::SettingsShowTransliteration => "Show transliteration",
            UiKey::SettingsIncludeDual => "Show dual in paradigm tables",
            UiKey::FiltersTitle => "Filters",
            UiKey::FiltersPos => "Part of speech",
            UiKey::FiltersTense => "Tense",
            UiKey::FiltersPerson => "Person",
            UiKey::FiltersVoice => "Voice",
            UiKey::FiltersMood => "Mood",
            UiKey::FiltersCase => "Case",
            UiKey::FiltersNumber => "Number",
            UiKey::FiltersWords => "Specific words",
            UiKey::FiltersReset => "Reset filters",
            UiKey::FiltersSearch => "Search by word or translation…",
            UiKey::BuilderTitle => "Create paradigm",
            UiKey::BuilderSavedNominal => "Paradigm saved and included in filters and drills.",
            UiKey::BuilderSavedVerb => "Verb paradigm saved and included in filters and drills.",
            UiKey::BuilderLemmaField => "Lemma",
            UiKey::BuilderTranslationField => "Translation / note",
            UiKey::BuilderSaveBtn => "Save paradigm",
            UiKey::ModeParadigm => "Paradigm view",
            UiKey::ModeFlashcard => "Flashcards",
            UiKey::ModeFlashcardRev => "Reverse flashcards",
            UiKey::ModeFillIn => "Fill in form",
            UiKey::ProgressTitle => "Progress",
            UiKey::Back => "← Back",
            UiKey::Reset => "Reset",
            UiKey::Delete => "Delete",
        }
    }
}
