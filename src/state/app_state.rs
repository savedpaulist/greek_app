use std::collections::HashMap;
#[cfg(not(target_arch = "wasm32"))]
use std::fs;
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::models::{Category, FilterParams, Form, Lemma, ProgressRecord, Session, Tag};
use crate::state::settings::Settings;

const PROGRESS_KEY: &str = "sfodra_progress_v1";
const SETTINGS_KEY: &str = "sfodra_settings_v1";
const SESSIONS_KEY: &str = "sfodra_sessions_v1";
const CUSTOM_DATA_KEY: &str = "sfodra_custom_data_v1";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomData {
    pub lemmas: Vec<Lemma>,
    pub forms: Vec<Form>,
}

#[derive(Debug, Clone)]
pub struct CustomFormEntry {
    pub case_tag: String,
    pub number_tag: String,
    pub greek_form: String,
}

#[derive(Debug, Clone)]
pub struct CustomVerbFormEntry {
    pub tense_tag: String,
    pub mood_tag: String,
    pub voice_tag: String,
    pub person_tag: String,
    pub number_tag: String,
    pub greek_form: String,
}

#[derive(Debug, Clone)]
pub struct CustomNominalParadigmDraft {
    pub lemma_greek: String,
    pub translation: Option<String>,
    pub pos: String,
    pub gender: Option<String>,
    pub entries: Vec<CustomFormEntry>,
}

#[derive(Debug, Clone)]
pub struct CustomVerbParadigmDraft {
    pub lemma_greek: String,
    pub translation: Option<String>,
    pub entries: Vec<CustomVerbFormEntry>,
}

// ── Persistent storage helpers ────────────────────────────────────────────

#[cfg(not(target_arch = "wasm32"))]
fn storage_file_path(key: &str) -> Option<PathBuf> {
    let base_dir = dirs::data_local_dir()
        .or_else(dirs::data_dir)
        .or_else(|| std::env::current_dir().ok())?;
    Some(base_dir.join("morph-app").join(format!("{key}.json")))
}

fn ls_get(key: &str) -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window()?
            .local_storage()
            .ok()??
            .get_item(key)
            .ok()?
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let path = storage_file_path(key)?;
        fs::read_to_string(path).ok()
    }
}

fn ls_set(key: &str, value: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(storage) =
            web_sys::window().and_then(|w| w.local_storage().ok()).flatten()
        {
            let _ = storage.set_item(key, value);
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Some(path) = storage_file_path(key) {
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }

            let temp_path = path.with_extension("json.tmp");
            if fs::write(&temp_path, value).is_ok() {
                let _ = fs::rename(&temp_path, &path);
            } else {
                let _ = fs::write(&path, value);
            }
        }
    }
}

// ── AppState ───────────────────────────────────────────────────────────────

/// Complete application state, shared via Dioxus context.
#[derive(Clone, Copy)]
pub struct AppState {
    /// All forms from the embedded database.
    pub forms: Signal<Vec<Form>>,
    /// All lemmas.
    pub lemmas: Signal<Vec<Lemma>>,
    /// User-authored lemmas persisted as JSON.
    pub custom_lemmas: Signal<Vec<Lemma>>,
    /// User-authored forms persisted as JSON.
    pub custom_forms: Signal<Vec<Form>>,
    /// Category tree (flat, with children populated).
    #[allow(dead_code)]
    pub categories: Signal<Vec<Category>>,
    /// Tag lookup table.
    #[allow(dead_code)]
    pub tags: Signal<Vec<Tag>>,
    /// Per-form progress records, keyed by form_id.
    pub progress: Signal<HashMap<i64, ProgressRecord>>,
    /// User settings.
    pub settings: Signal<Settings>,
    /// Session history.
    pub sessions: Signal<Vec<Session>>,
    /// Active filter for the current study session.
    pub filter: Signal<FilterParams>,
    /// Active filters drawer state
    pub filters_open: Signal<bool>,
    /// Active settings drawer state
    pub settings_open: Signal<bool>,
}

impl AppState {
    /// Load or initialise state from persistent storage + embedded DB data.
    pub fn init(
        mut forms: Vec<Form>,
        mut lemmas: Vec<Lemma>,
        categories: Vec<Category>,
        tags: Vec<Tag>,
    ) -> Self {
        let progress: HashMap<i64, ProgressRecord> = ls_get(PROGRESS_KEY)
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default();

        let settings: Settings = ls_get(SETTINGS_KEY)
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default();

        let sessions: Vec<Session> = ls_get(SESSIONS_KEY)
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default();

        let custom_data: CustomData = ls_get(CUSTOM_DATA_KEY)
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default();

        forms.extend(custom_data.forms.clone());
        lemmas.extend(custom_data.lemmas.clone());

        Self {
            forms: Signal::new(forms),
            lemmas: Signal::new(lemmas),
            custom_lemmas: Signal::new(custom_data.lemmas),
            custom_forms: Signal::new(custom_data.forms),
            categories: Signal::new(categories),
            tags: Signal::new(tags),
            progress: Signal::new(progress),
            settings: Signal::new(settings),
            sessions: Signal::new(sessions),
            filter: Signal::new(FilterParams::default()),
            filters_open: Signal::new(false),
            settings_open: Signal::new(false),
        }
    }

    /// Persist progress to browser storage or app files.
    pub fn save_progress(&self) {
        if let Ok(json) = serde_json::to_string(&*self.progress.read()) {
            ls_set(PROGRESS_KEY, &json);
        }
    }

    /// Persist settings to browser storage or app files.
    pub fn save_settings(&self) {
        if let Ok(json) = serde_json::to_string(&*self.settings.read()) {
            ls_set(SETTINGS_KEY, &json);
        }
    }

    /// Persist sessions to browser storage or app files.
    #[allow(dead_code)]
    pub fn save_sessions(&self) {
        if let Ok(json) = serde_json::to_string(&*self.sessions.read()) {
            ls_set(SESSIONS_KEY, &json);
        }
    }

    pub fn save_custom_data(&self) {
        let payload = CustomData {
            lemmas: self.custom_lemmas.read().clone(),
            forms: self.custom_forms.read().clone(),
        };
        if let Ok(json) = serde_json::to_string(&payload) {
            ls_set(CUSTOM_DATA_KEY, &json);
        }
    }

    /// Record a correct/incorrect answer for. Updates SM-2, persists.
    pub fn record_answer(&mut self, form_id: i64, quality: crate::logic::sm2::Quality) {
        let mut progress = self.progress.write();
        let rec = progress.entry(form_id).or_insert_with(|| ProgressRecord::new(form_id));
        crate::logic::sm2::sm2_update(rec, quality);
        drop(progress);
        self.save_progress();
    }

    /// Reset all progress.
    pub fn reset_progress(&mut self) {
        *self.progress.write() = HashMap::new();
        self.save_progress();
    }

    /// Returns forms matching the current filter.
    pub fn filtered_forms(&self) -> Vec<Form> {
        let filter = self.filter.read();
        let progress = self.progress.read();
        let include_dual = self.settings.read().include_dual;
        self.forms
            .read()
            .iter()
            .filter(|f| {
                if !f.is_paradigm_form() {
                    return false;
                }
                if !include_dual && f.number_tag.as_deref() == Some("du") {
                    return false;
                }
                if !filter.matches_form(f) {
                    return false;
                }
                if filter.only_due {
                    let due = progress
                        .get(&f.id)
                        .map(|p| p.is_due())
                        .unwrap_or(true);
                    if !due {
                        return false;
                    }
                }
                if filter.exclude_learned {
                    let learned = progress.get(&f.id).map(|p| p.is_learned()).unwrap_or(false);
                    if learned {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect()
    }

    pub fn lemma_has_paradigm(&self, lemma_id: i64) -> bool {
        self.forms
            .read()
            .iter()
            .any(|form| form.lemma_id == lemma_id && form.is_paradigm_form())
    }

    /// Lookup lemma by id.
    pub fn lemma_by_id(&self, id: i64) -> Option<Lemma> {
        self.lemmas.read().iter().find(|l| l.id == id).cloned()
    }

    pub fn paradigm_forms_for_lemma(&self, lemma_id: i64) -> Vec<Form> {
        self.forms
            .read()
            .iter()
            .filter(|form| form.lemma_id == lemma_id && form.is_paradigm_form())
            .cloned()
            .collect()
    }

    pub fn add_custom_nominal_paradigm(
        &mut self,
        draft: CustomNominalParadigmDraft,
    ) -> Result<(), String> {
        let lemma_greek = draft.lemma_greek.trim().to_string();
        if lemma_greek.is_empty() {
            return Err("Укажите заголовочную форму леммы.".into());
        }

        let entries: Vec<CustomFormEntry> = draft
            .entries
            .into_iter()
            .filter(|entry| !entry.greek_form.trim().is_empty())
            .collect();
        if entries.is_empty() {
            return Err("Заполните хотя бы одну форму в парадигме.".into());
        }

        let next_lemma_id = self.lemmas.read().iter().map(|lemma| lemma.id).max().unwrap_or(0) + 1;
        let mut next_form_id = self.forms.read().iter().map(|form| form.id).max().unwrap_or(0) + 1;
        let gender = draft.gender.and_then(|value| {
            let trimmed = value.trim().to_string();
            if trimmed.is_empty() { None } else { Some(trimmed) }
        });

        let custom_lemma = Lemma {
            id: next_lemma_id,
            greek: lemma_greek,
            latin: None,
            english: None,
            russian: draft.translation.and_then(|value| {
                let trimmed = value.trim().to_string();
                if trimmed.is_empty() { None } else { Some(trimmed) }
            }),
            part_of_speech: Some(draft.pos.clone()),
            category_id: None,
            lesson_kozar: None,
            lesson_mast: None,
            sort_order: 10_000 + next_lemma_id as i32,
        };

        let custom_forms: Vec<Form> = entries
            .into_iter()
            .enumerate()
            .map(|(index, entry)| {
                let form = Form {
                    id: next_form_id + index as i64,
                    lemma_id: next_lemma_id,
                    greek_form: entry.greek_form.trim().to_string(),
                    transliteration: None,
                    category_id: None,
                    pos: Some(draft.pos.clone()),
                    case_tag: Some(entry.case_tag),
                    number_tag: Some(entry.number_tag),
                    gender_tag: gender.clone(),
                    tense_tag: None,
                    voice_tag: None,
                    mood_tag: None,
                    person_tag: None,
                    degree_tag: None,
                    decl_type: Some("custom".into()),
                    conj_type: None,
                    adj_class: None,
                    part_type: None,
                    num_type: None,
                    pron_type: None,
                    lesson_kozar: None,
                    lesson_mast: None,
                    paradigm_row: Some(index as i32),
                    sort_order: index as i32,
                    notes: Some("custom".into()),
                };
                form
            })
            .collect();
        next_form_id += custom_forms.len() as i64;
        let _ = next_form_id;

        self.custom_lemmas.write().push(custom_lemma.clone());
        self.custom_forms.write().extend(custom_forms.clone());
        self.lemmas.write().push(custom_lemma);
        self.forms.write().extend(custom_forms);
        self.save_custom_data();
        Ok(())
    }

    pub fn add_custom_verb_paradigm(
        &mut self,
        draft: CustomVerbParadigmDraft,
    ) -> Result<(), String> {
        let lemma_greek = draft.lemma_greek.trim().to_string();
        if lemma_greek.is_empty() {
            return Err("Укажите заголовочную форму леммы.".into());
        }

        let entries: Vec<CustomVerbFormEntry> = draft
            .entries
            .into_iter()
            .filter(|entry| !entry.greek_form.trim().is_empty())
            .collect();
        if entries.is_empty() {
            return Err("Заполните хотя бы одну глагольную форму в парадигме.".into());
        }

        let next_lemma_id = self
            .lemmas
            .read()
            .iter()
            .map(|lemma| lemma.id)
            .max()
            .unwrap_or(0)
            + 1;
        let next_form_id = self
            .forms
            .read()
            .iter()
            .map(|form| form.id)
            .max()
            .unwrap_or(0)
            + 1;

        let custom_lemma = Lemma {
            id: next_lemma_id,
            greek: lemma_greek,
            latin: None,
            english: None,
            russian: draft.translation.and_then(|value| {
                let trimmed = value.trim().to_string();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed)
                }
            }),
            part_of_speech: Some("verb".into()),
            category_id: None,
            lesson_kozar: None,
            lesson_mast: None,
            sort_order: 10_000 + next_lemma_id as i32,
        };

        let custom_forms: Vec<Form> = entries
            .into_iter()
            .enumerate()
            .map(|(index, entry)| Form {
                id: next_form_id + index as i64,
                lemma_id: next_lemma_id,
                greek_form: entry.greek_form.trim().to_string(),
                transliteration: None,
                category_id: None,
                pos: Some("verb".into()),
                case_tag: None,
                number_tag: Some(entry.number_tag),
                gender_tag: None,
                tense_tag: Some(entry.tense_tag),
                voice_tag: Some(entry.voice_tag),
                mood_tag: Some(entry.mood_tag),
                person_tag: Some(entry.person_tag),
                degree_tag: None,
                decl_type: None,
                conj_type: Some("custom".into()),
                adj_class: None,
                part_type: None,
                num_type: None,
                pron_type: None,
                lesson_kozar: None,
                lesson_mast: None,
                paradigm_row: Some(index as i32),
                sort_order: index as i32,
                notes: Some("custom".into()),
            })
            .collect();

        self.custom_lemmas.write().push(custom_lemma.clone());
        self.custom_forms.write().extend(custom_forms.clone());
        self.lemmas.write().push(custom_lemma);
        self.forms.write().extend(custom_forms);
        self.save_custom_data();
        Ok(())
    }

    pub fn remove_custom_paradigm(&mut self, lemma_id: i64) {
        self.custom_lemmas.write().retain(|lemma| lemma.id != lemma_id);
        self.custom_forms.write().retain(|form| form.lemma_id != lemma_id);
        self.lemmas.write().retain(|lemma| lemma.id != lemma_id);
        self.forms.write().retain(|form| form.lemma_id != lemma_id);
        self.filter.write().lemma_ids.retain(|id| *id != lemma_id);
        self.save_custom_data();
    }

    /// Stats snapshot.
    pub fn stats(&self) -> ProgressStats {
        let total = self.forms.read().len();
        let progress = self.progress.read();
        let seen = progress.len();
        let learned = progress.values().filter(|p| p.is_learned()).count();
        let total_correct: u32 = progress.values().map(|p| p.correct).sum();
        let total_incorrect: u32 = progress.values().map(|p| p.incorrect).sum();
        let accuracy = if total_correct + total_incorrect > 0 {
            total_correct as f32 / (total_correct + total_incorrect) as f32
        } else {
            0.0
        };
        ProgressStats { total, seen, learned, accuracy, sessions: self.sessions.read().len() }
    }
}

#[derive(Debug, Clone)]
pub struct ProgressStats {
    pub total: usize,
    pub seen: usize,
    pub learned: usize,
    pub accuracy: f32,
    pub sessions: usize,
}
