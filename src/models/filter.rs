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

/// Study preset filter definitions.
impl FilterParams {
    pub fn preset(id: &str) -> Option<FilterParams> {
        match id {
            "decl1_nouns" => {
                let mut f = FilterParams::default().with_pos("noun");
                f.decl_types.extend(["1a_a_long".into(), "1b_a_short".into(), "1c_e_short".into(), "1d_as_es".into()]);
                Some(f)
            }
            "decl2_nouns" => {
                let mut f = FilterParams::default().with_pos("noun");
                f.decl_types.extend(["2a_os".into(), "2b_on".into()]);
                Some(f)
            }
            "decl3_nouns" => {
                let mut f = FilterParams::default().with_pos("noun");
                f.decl_types.extend([
                    "3a_sonant".into(),
                    "3b_labial".into(),
                    "3c_velar".into(),
                    "3d_dental".into(),
                    "3f_sigma".into(),
                ]);
                Some(f)
            }
            "adj_2ending" => {
                let mut f = FilterParams::default().with_pos("adj");
                f.decl_types.push("2ending".into());
                Some(f)
            }
            "adj_3ending" => {
                let mut f = FilterParams::default().with_pos("adj");
                f.decl_types.extend(["3ending_asc".into(), "3ending_eia".into()]);
                Some(f)
            }
            "verbs_thematic" => {
                let mut f = FilterParams::default().with_pos("verb");
                f.conj_types.push("thematic".into());
                Some(f)
            }
            "verbs_contract_eo" => {
                let mut f = FilterParams::default().with_pos("verb");
                f.conj_types.push("contract_eo".into());
                Some(f)
            }
            "verbs_mi" => {
                let mut f = FilterParams::default().with_pos("verb");
                f.conj_types.push("mi_verb".into());
                Some(f)
            }
            "pronouns_personal" => {
                let mut f = FilterParams::default().with_pos("pronoun");
                f.decl_types.push("personal".into());
                Some(f)
            }
            "pronouns_demonstrative" => {
                let mut f = FilterParams::default().with_pos("pronoun");
                f.decl_types.push("demonstrative".into());
                Some(f)
            }
            "pronouns_relative" => {
                let mut f = FilterParams::default().with_pos("pronoun");
                f.decl_types.push("relative".into());
                Some(f)
            }
            "due_only" => Some(FilterParams::default().only_due()),
            // Legacy aliases kept for backward-compat
            "nouns_all" => Some(FilterParams::default().with_pos("noun")),
            "adj_all" => Some(FilterParams::default().with_pos("adj")),
            "pronouns_all" => Some(FilterParams::default().with_pos("pronoun")),
            _ => None,
        }
    }

    pub fn presets() -> &'static [(&'static str, &'static str, &'static str)] {
        &[
            ("decl1_nouns",          "1-е склонение",          "1st declension nouns"),
            ("decl2_nouns",          "2-е склонение",          "2nd declension nouns"),
            ("decl3_nouns",          "3-е склонение",          "3rd declension nouns"),
            ("adj_2ending",          "Прил. 2-оконч.",         "2-ending adjectives"),
            ("adj_3ending",          "Прил. 3-оконч.",         "3-ending adjectives"),
            ("verbs_thematic",       "Глаголы на -ω",          "Thematic verbs (-ω)"),
            ("verbs_contract_eo",    "Глаголы на -έω",         "Contract verbs (-έω)"),
            ("verbs_mi",             "Глаголы на -μι",         "Mi-verbs (-μι)"),
            ("pronouns_personal",    "Мест. личные",           "Personal pronouns"),
            ("pronouns_demonstrative","Мест. указательные",    "Demonstrative pronouns"),
            ("pronouns_relative",    "Мест. относительные",    "Relative pronouns"),
            ("due_only",             "Только к повторению",    "Due for review only"),
        ]
    }
}
