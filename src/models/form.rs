use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lemma {
    pub id: i64,
    pub greek: String,
    pub latin: Option<String>,
    pub english: Option<String>,
    pub russian: Option<String>,
    pub part_of_speech: Option<String>,
    pub category_id: Option<i64>,
    pub lesson_kozar: Option<i32>,
    pub lesson_mast: Option<i32>,
    pub sort_order: i32,
}

#[allow(dead_code)]
impl Lemma {
    pub fn display_translation(&self) -> &str {
        self.russian.as_deref().or(self.english.as_deref()).unwrap_or("")
    }

    pub fn test_prompt_greek(&self) -> String {
        strip_leading_article(&self.greek, self.part_of_speech.as_deref())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Form {
    pub id: i64,
    pub lemma_id: i64,
    pub greek_form: String,
    pub transliteration: Option<String>,
    pub category_id: Option<i64>,
    pub pos: Option<String>,
    pub case_tag: Option<String>,
    pub number_tag: Option<String>,
    pub gender_tag: Option<String>,
    pub tense_tag: Option<String>,
    pub voice_tag: Option<String>,
    pub mood_tag: Option<String>,
    pub person_tag: Option<String>,
    pub degree_tag: Option<String>,
    pub decl_type: Option<String>,
    pub conj_type: Option<String>,
    pub adj_class: Option<String>,
    pub part_type: Option<String>,
    pub num_type: Option<String>,
    pub pron_type: Option<String>,
    pub lesson_kozar: Option<i32>,
    pub lesson_mast: Option<i32>,
    pub paradigm_row: Option<i32>,
    pub sort_order: i32,
    pub notes: Option<String>,
}

#[allow(dead_code)]
impl Form {
    pub fn test_prompt_greek(&self) -> String {
        strip_leading_article(&self.greek_form, self.pos.as_deref())
    }

    pub fn is_nominal_paradigm_form(&self) -> bool {
        matches!(self.pos.as_deref(), Some("noun" | "adj" | "pronoun" | "article" | "participle"))
            && self.case_tag.is_some()
            && self.number_tag.is_some()
    }

    pub fn is_verb_paradigm_form(&self) -> bool {
        matches!(self.pos.as_deref(), Some("verb"))
            && self.person_tag.is_some()
            && self.number_tag.is_some()
            && self.tense_tag.is_some()
            && self.voice_tag.is_some()
    }

    pub fn is_paradigm_form(&self) -> bool {
        self.is_nominal_paradigm_form() || self.is_verb_paradigm_form()
    }

    /// Human-readable grammatical description.
    pub fn grammar_label_ru(&self) -> String {
        let mut parts = vec![];
        // For participles from the main DB: tense+voice are encoded in part_type
        // (tense_tag and voice_tag are NULL); for custom participles both are set.
        let is_ptcp = self.mood_tag.as_deref() == Some("part")
            || self.pos.as_deref() == Some("participle");
        if is_ptcp && self.tense_tag.is_none() && self.voice_tag.is_none() {
            if let Some(pt) = &self.part_type {
                let (tl, vl) = part_type_label_ru(pt);
                parts.push(tl);
                if !vl.is_empty() { parts.push(vl); }
            }
        } else {
            if let Some(t) = &self.tense_tag { parts.push(tense_ru(t)); }
            if let Some(v) = &self.voice_tag { parts.push(voice_ru(v)); }
        }
        if let Some(m) = &self.mood_tag {
            parts.push(mood_ru(m));
        }
        if let Some(p) = &self.person_tag {
            parts.push(person_ru(p));
        }
        if let Some(n) = &self.number_tag {
            parts.push(number_ru(n));
        }
        if let Some(c) = &self.case_tag {
            parts.push(case_ru(c));
        }
        if let Some(g) = &self.gender_tag {
            parts.push(gender_ru(g));
        }
        if let Some(d) = &self.degree_tag {
            parts.push(degree_ru(d));
        }
        parts.join(" · ")
    }

    pub fn grammar_label_en(&self) -> String {
        let mut parts = vec![];
        if let Some(t) = &self.tense_tag {
            parts.push(tense_en(t));
        }
        if let Some(v) = &self.voice_tag {
            parts.push(voice_en(v));
        }
        if let Some(m) = &self.mood_tag {
            parts.push(mood_en(m));
        }
        if let Some(p) = &self.person_tag {
            parts.push(person_en(p));
        }
        if let Some(n) = &self.number_tag {
            parts.push(number_en(n));
        }
        if let Some(c) = &self.case_tag {
            parts.push(case_en(c));
        }
        if let Some(g) = &self.gender_tag {
            parts.push(gender_en(g));
        }
        if let Some(d) = &self.degree_tag {
            parts.push(degree_en(d));
        }
        parts.join(" · ")
    }
}

/// Decode DB `part_type` values like "pres_act", "aor1_pass" into (tense_label, voice_label).
fn part_type_label_ru(pt: &str) -> (&'static str, &'static str) {
    match pt {
        "pres_act"      => ("Наст.",   "акт."),
        "pres_mid"      => ("Наст.",   "мед."),
        "pres_pass"     => ("Наст.",   "пас."),
        "pres_mid_pass" => ("Наст.",   "мед./пас."),
        "imperf_act"    => ("Импф.",   "акт."),
        "aor1_act"      => ("Аор. I",  "акт."),
        "aor1_mid"      => ("Аор. I",  "мед."),
        "aor1_pass"     => ("Аор. I",  "пас."),
        "aor2_act"      => ("Аор. II", "акт."),
        "aor2_mid"      => ("Аор. II", "мед."),
        "aor2_pass"     => ("Аор. II", "пас."),
        "aor_pass"      => ("Аор.",    "пас."),
        "perf_act"      => ("Перф.",   "акт."),
        "perf_mid_pass" => ("Перф.",   "мед./пас."),
        "fut_act"       => ("Буд.",    "акт."),
        "fut_mid"       => ("Буд.",    "мед."),
        "fut_pass"      => ("Буд.",    "пас."),
        _               => ("причастие", ""),
    }
}

fn tense_ru(s: &str) -> &str {
    match s {
        "pres" => "Наст.",
        "imperf" => "Импф.",
        "fut" => "Буд.",
        "aor1" => "Аор. I",
        "aor2" => "Аор. II",
        "aor_pass" => "Аор. пас.",
        "perf" => "Перф.",
        "pluperf" => "Плюскв.",
        "futperf" => "Буд. перф.",
        _ => s,
    }
}
fn voice_ru(s: &str) -> &str {
    match s {
        "act" => "Акт.",
        "mid" => "Мед.",
        "pass" => "Пас.",
        "mid_pass" => "Мед./Пас.",
        _ => s,
    }
}
fn mood_ru(s: &str) -> &str {
    match s {
        "ind" => "Изъяв.",
        "subj" => "Сослаг.",
        "opt" => "Жел.",
        "imp" => "Повел.",
        "inf" => "Инфин.",
        "part" => "Прич.",
        _ => s,
    }
}
fn person_ru(s: &str) -> &str {
    match s {
        "1" => "1-е л.",
        "2" => "2-е л.",
        "3" => "3-е л.",
        _ => s,
    }
}
fn number_ru(s: &str) -> &str {
    match s {
        "sg" => "Ед.ч.",
        "du" => "Дв.ч.",
        "pl" => "Мн.ч.",
        _ => s,
    }
}
fn case_ru(s: &str) -> &str {
    match s {
        "nom" => "Именит.",
        "gen" => "Родит.",
        "dat" => "Датель.",
        "acc" => "Винит.",
        "voc" => "Звател.",
        _ => s,
    }
}
fn gender_ru(s: &str) -> &str {
    match s {
        "m" => "М.р.",
        "f" => "Ж.р.",
        "n" => "С.р.",
        _ => s,
    }
}
fn degree_ru(s: &str) -> &str {
    match s {
        "pos" => "Полож.",
        "comp" => "Сравн.",
        "superl" => "Превосх.",
        _ => s,
    }
}
#[allow(dead_code)]
fn tense_en(s: &str) -> &str {
    match s {
        "pres" => "Pres",
        "imperf" => "Impf",
        "fut" => "Fut",
        "aor1" => "Aor I",
        "aor2" => "Aor II",
        "aor_pass" => "Aor Pass",
        "perf" => "Perf",
        "pluperf" => "Plupf",
        "futperf" => "Fut Pf",
        _ => s,
    }
}
#[allow(dead_code)]
fn voice_en(s: &str) -> &str {
    match s {
        "act" => "Act",
        "mid" => "Mid",
        "pass" => "Pass",
        "mid_pass" => "Mid/Pass",
        _ => s,
    }
}
#[allow(dead_code)]
fn mood_en(s: &str) -> &str {
    match s {
        "ind" => "Ind",
        "subj" => "Subj",
        "opt" => "Opt",
        "imp" => "Imp",
        "inf" => "Inf",
        "part" => "Part",
        _ => s,
    }
}
#[allow(dead_code)]
fn person_en(s: &str) -> &str {
    match s {
        "1" => "1st",
        "2" => "2nd",
        "3" => "3rd",
        _ => s,
    }
}
#[allow(dead_code)]
fn number_en(s: &str) -> &str {
    match s {
        "sg" => "Sg",
        "du" => "Du",
        "pl" => "Pl",
        _ => s,
    }
}
#[allow(dead_code)]
fn case_en(s: &str) -> &str {
    match s {
        "nom" => "Nom",
        "gen" => "Gen",
        "dat" => "Dat",
        "acc" => "Acc",
        "voc" => "Voc",
        _ => s,
    }
}
#[allow(dead_code)]
fn gender_en(s: &str) -> &str {
    match s {
        "m" => "M",
        "f" => "F",
        "n" => "N",
        _ => s,
    }
}
#[allow(dead_code)]
fn degree_en(s: &str) -> &str {
    match s {
        "pos" => "Pos",
        "comp" => "Comp",
        "superl" => "Superl",
        _ => s,
    }
}

fn strip_leading_article(text: &str, _pos: Option<&str>) -> String {
    // Strip for all parts of speech — uses the known-articles list in diacritics
    // so only genuine article prefixes (ὁ, ἡ, τό, τοῦ, …) are removed.
    crate::logic::diacritics::strip_leading_article(text).to_string()
}

// ── Category ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: i64,
    pub parent_id: Option<i64>,
    pub code: Option<String>,
    pub name_ru: String,
    pub name_en: String,
    pub name_gr: Option<String>,
    pub depth: i32,
    pub sort_order: i32,
    pub description: Option<String>,
    #[serde(default)]
    pub children: Vec<Category>,
}

#[allow(dead_code)]
impl Category {
    pub fn name(&self, lang_ru: bool) -> &str {
        if lang_ru { &self.name_ru } else { &self.name_en }
    }
}

// ── Tag ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: i64,
    pub group_name: String,
    pub tag_value: String,
    pub label_ru: Option<String>,
    pub label_en: Option<String>,
    pub sort_order: i32,
}
