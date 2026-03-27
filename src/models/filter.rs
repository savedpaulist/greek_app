use serde::{Deserialize, Serialize};

/// Filter parameters for selecting forms to study.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FilterParams {
    // ── Part of speech ──────────────────────────────────────────────────────
    pub pos: Vec<String>,

    // ── Category ids ────────────────────────────────────────────────────────
    pub category_ids: Vec<i64>,

    // ── Lemma ids  ──────────────────────────────────────────────────────────
    pub lemma_ids: Vec<i64>,

    // ── Grammatical tags ────────────────────────────────────────────────────
    pub cases: Vec<String>,
    pub numbers: Vec<String>,
    pub genders: Vec<String>,
    pub tenses: Vec<String>,
    pub voices: Vec<String>,
    pub moods: Vec<String>,
    pub persons: Vec<String>,
    pub degrees: Vec<String>,

    // ── Morphology ─────────────────────────────────────────────────────────
    pub decl_types: Vec<String>,
    pub conj_types: Vec<String>,

    // ── Lesson numbers ──────────────────────────────────────────────────────
    pub lesson_kozar_max: Option<i32>,
    pub lesson_mast_max: Option<i32>,

    // ── Progress filters ────────────────────────────────────────────────────
    /// Only forms that are due for review (next_review <= now).
    pub only_due: bool,
    /// Exclude forms the user has already learned (streak >= 5).
    pub exclude_learned: bool,
}

impl FilterParams {
    pub fn with_pos(mut self, pos: impl Into<String>) -> Self {
        self.pos.push(pos.into());
        self
    }

    #[allow(dead_code)]
    pub fn with_case(mut self, c: impl Into<String>) -> Self {
        self.cases.push(c.into());
        self
    }

    pub fn only_due(mut self) -> Self {
        self.only_due = true;
        self
    }

    #[allow(dead_code)]
    pub fn exclude_learned(mut self) -> Self {
        self.exclude_learned = true;
        self
    }

    /// Returns true if the given form matches all set filters (client-side).
    pub fn matches_form(&self, form: &crate::models::form::Form) -> bool {
        if !self.pos.is_empty() {
            let fpos = form.pos.as_deref().unwrap_or("");
            if !self.pos.iter().any(|p| p == fpos) {
                return false;
            }
        }
        if !self.category_ids.is_empty() {
            let fcat = form.category_id.unwrap_or(0);
            if !self.category_ids.contains(&fcat) {
                return false;
            }
        }
        if !self.lemma_ids.is_empty() && !self.lemma_ids.contains(&form.lemma_id) {
            return false;
        }
        if !self.cases.is_empty() {
            match &form.case_tag {
                Some(c) if self.cases.contains(c) => {}
                _ => return false,
            }
        }
        if !self.numbers.is_empty() {
            match &form.number_tag {
                Some(n) if self.numbers.contains(n) => {}
                _ => return false,
            }
        }
        if !self.genders.is_empty() {
            match &form.gender_tag {
                Some(g) if self.genders.contains(g) => {}
                _ => return false,
            }
        }
        if !self.tenses.is_empty() {
            match &form.tense_tag {
                Some(t) if self.tenses.contains(t) => {}
                _ => return false,
            }
        }
        if !self.voices.is_empty() {
            match &form.voice_tag {
                Some(v) if self.voices.contains(v) => {}
                _ => return false,
            }
        }
        if !self.moods.is_empty() {
            match &form.mood_tag {
                Some(m) if self.moods.contains(m) => {}
                _ => return false,
            }
        }
        if !self.persons.is_empty() {
            match &form.person_tag {
                Some(p) if self.persons.contains(p) => {}
                _ => return false,
            }
        }
        if !self.degrees.is_empty() {
            match &form.degree_tag {
                Some(d) if self.degrees.contains(d) => {}
                _ => return false,
            }
        }
        if !self.decl_types.is_empty() {
            match &form.decl_type {
                Some(d) if self.decl_types.contains(d) => {}
                _ => return false,
            }
        }
        if !self.conj_types.is_empty() {
            match &form.conj_type {
                Some(c) if self.conj_types.contains(c) => {}
                _ => return false,
            }
        }
        if let Some(max) = self.lesson_kozar_max {
            match form.lesson_kozar {
                Some(l) if l <= max => {}
                _ => return false,
            }
        }
        if let Some(max) = self.lesson_mast_max {
            match form.lesson_mast {
                Some(l) if l <= max => {}
                _ => return false,
            }
        }
        true
    }
}
