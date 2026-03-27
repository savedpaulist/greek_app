#![allow(dead_code)]

use serde::{Deserialize, Serialize};

// ── Part of Speech ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Pos {
    #[default]
    Noun,
    Verb,
    Adj,
    Pronoun,
    Article,
    Num,
    Participle,
    #[serde(rename = "letter")]
    Letter,
    Term,
    #[serde(other)]
    Other,
}

impl Pos {
    pub fn all() -> &'static [&'static str] {
        &["noun", "verb", "adj", "pronoun", "article", "num", "participle", "letter", "term"]
    }
    pub fn label_ru(&self) -> &'static str {
        match self {
            Pos::Noun => "Существительное",
            Pos::Verb => "Глагол",
            Pos::Adj => "Прилагательное",
            Pos::Pronoun => "Местоимение",
            Pos::Article => "Артикль",
            Pos::Num => "Числительное",
            Pos::Participle => "Причастие",
            Pos::Letter => "Буква",
            Pos::Term => "Термин",
            Pos::Other => "Другое",
        }
    }
    pub fn label_en(&self) -> &'static str {
        match self {
            Pos::Noun => "Noun",
            Pos::Verb => "Verb",
            Pos::Adj => "Adjective",
            Pos::Pronoun => "Pronoun",
            Pos::Article => "Article",
            Pos::Num => "Numeral",
            Pos::Participle => "Participle",
            Pos::Letter => "Letter",
            Pos::Term => "Term",
            Pos::Other => "Other",
        }
    }
    pub fn from_str(s: &str) -> Self {
        match s {
            "noun" => Pos::Noun,
            "verb" => Pos::Verb,
            "adj" => Pos::Adj,
            "pronoun" => Pos::Pronoun,
            "article" => Pos::Article,
            "num" => Pos::Num,
            "participle" => Pos::Participle,
            "letter" => Pos::Letter,
            "term" => Pos::Term,
            _ => Pos::Other,
        }
    }
    pub fn to_db(&self) -> &'static str {
        match self {
            Pos::Noun => "noun",
            Pos::Verb => "verb",
            Pos::Adj => "adj",
            Pos::Pronoun => "pronoun",
            Pos::Article => "article",
            Pos::Num => "num",
            Pos::Participle => "participle",
            Pos::Letter => "letter",
            Pos::Term => "term",
            Pos::Other => "",
        }
    }
}

// ── Case ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Case {
    Nom,
    Gen,
    Dat,
    Acc,
    Voc,
}

impl Case {
    pub fn all() -> &'static [Case] {
        &[Case::Nom, Case::Gen, Case::Dat, Case::Acc, Case::Voc]
    }
    pub fn label_ru(&self) -> &'static str {
        match self {
            Case::Nom => "Именит.",
            Case::Gen => "Родит.",
            Case::Dat => "Датель.",
            Case::Acc => "Винит.",
            Case::Voc => "Звател.",
        }
    }
    pub fn label_en(&self) -> &'static str {
        match self {
            Case::Nom => "Nom",
            Case::Gen => "Gen",
            Case::Dat => "Dat",
            Case::Acc => "Acc",
            Case::Voc => "Voc",
        }
    }
    pub fn to_db(&self) -> &'static str {
        match self {
            Case::Nom => "nom",
            Case::Gen => "gen",
            Case::Dat => "dat",
            Case::Acc => "acc",
            Case::Voc => "voc",
        }
    }
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "nom" => Some(Case::Nom),
            "gen" => Some(Case::Gen),
            "dat" => Some(Case::Dat),
            "acc" => Some(Case::Acc),
            "voc" => Some(Case::Voc),
            _ => None,
        }
    }
}

// ── Number ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GNumber {
    Sg,
    Du,
    Pl,
}

impl GNumber {
    pub fn all() -> &'static [GNumber] {
        &[GNumber::Sg, GNumber::Du, GNumber::Pl]
    }
    pub fn label_ru(&self) -> &'static str {
        match self {
            GNumber::Sg => "Ед.",
            GNumber::Du => "Дв.",
            GNumber::Pl => "Мн.",
        }
    }
    pub fn label_en(&self) -> &'static str {
        match self {
            GNumber::Sg => "Sg",
            GNumber::Du => "Du",
            GNumber::Pl => "Pl",
        }
    }
    pub fn to_db(&self) -> &'static str {
        match self {
            GNumber::Sg => "sg",
            GNumber::Du => "du",
            GNumber::Pl => "pl",
        }
    }
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "sg" => Some(GNumber::Sg),
            "du" => Some(GNumber::Du),
            "pl" => Some(GNumber::Pl),
            _ => None,
        }
    }
}

// ── Gender ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    M,
    F,
    N,
}

impl Gender {
    pub fn all() -> &'static [Gender] {
        &[Gender::M, Gender::F, Gender::N]
    }
    pub fn label_ru(&self) -> &'static str {
        match self {
            Gender::M => "М.р.",
            Gender::F => "Ж.р.",
            Gender::N => "С.р.",
        }
    }
    pub fn label_en(&self) -> &'static str {
        match self {
            Gender::M => "M",
            Gender::F => "F",
            Gender::N => "N",
        }
    }
    pub fn to_db(&self) -> &'static str {
        match self {
            Gender::M => "m",
            Gender::F => "f",
            Gender::N => "n",
        }
    }
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "m" => Some(Gender::M),
            "f" => Some(Gender::F),
            "n" => Some(Gender::N),
            _ => None,
        }
    }
}

// ── Tense ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Tense {
    Pres,
    Imperf,
    Fut,
    Aor1,
    Aor2,
    AorPass,
    Perf,
    Pluperf,
    Futperf,
}

impl Tense {
    pub fn all() -> &'static [Tense] {
        &[
            Tense::Pres,
            Tense::Imperf,
            Tense::Fut,
            Tense::Aor1,
            Tense::Aor2,
            Tense::AorPass,
            Tense::Perf,
            Tense::Pluperf,
            Tense::Futperf,
        ]
    }
    pub fn label_ru(&self) -> &'static str {
        match self {
            Tense::Pres => "Наст.",
            Tense::Imperf => "Импф.",
            Tense::Fut => "Буд.",
            Tense::Aor1 => "Аор. I",
            Tense::Aor2 => "Аор. II",
            Tense::AorPass => "Аор. пас.",
            Tense::Perf => "Перф.",
            Tense::Pluperf => "Плюскв.",
            Tense::Futperf => "Буд.перф.",
        }
    }
    pub fn label_en(&self) -> &'static str {
        match self {
            Tense::Pres => "Pres",
            Tense::Imperf => "Impf",
            Tense::Fut => "Fut",
            Tense::Aor1 => "Aor I",
            Tense::Aor2 => "Aor II",
            Tense::AorPass => "Aor Pass",
            Tense::Perf => "Perf",
            Tense::Pluperf => "Plupf",
            Tense::Futperf => "Fut Pf",
        }
    }
    pub fn to_db(&self) -> &'static str {
        match self {
            Tense::Pres => "pres",
            Tense::Imperf => "imperf",
            Tense::Fut => "fut",
            Tense::Aor1 => "aor1",
            Tense::Aor2 => "aor2",
            Tense::AorPass => "aor_pass",
            Tense::Perf => "perf",
            Tense::Pluperf => "pluperf",
            Tense::Futperf => "futperf",
        }
    }
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pres" => Some(Tense::Pres),
            "imperf" => Some(Tense::Imperf),
            "fut" => Some(Tense::Fut),
            "aor1" => Some(Tense::Aor1),
            "aor2" => Some(Tense::Aor2),
            "aor_pass" => Some(Tense::AorPass),
            "perf" => Some(Tense::Perf),
            "pluperf" => Some(Tense::Pluperf),
            "futperf" => Some(Tense::Futperf),
            _ => None,
        }
    }
}

// ── Voice ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Voice {
    Act,
    Mid,
    Pass,
    MidPass,
}

impl Voice {
    pub fn all() -> &'static [Voice] {
        &[Voice::Act, Voice::Mid, Voice::Pass, Voice::MidPass]
    }
    pub fn label_ru(&self) -> &'static str {
        match self {
            Voice::Act => "Акт.",
            Voice::Mid => "Мед.",
            Voice::Pass => "Пас.",
            Voice::MidPass => "Мед./Пас.",
        }
    }
    pub fn label_en(&self) -> &'static str {
        match self {
            Voice::Act => "Act",
            Voice::Mid => "Mid",
            Voice::Pass => "Pass",
            Voice::MidPass => "Mid/Pass",
        }
    }
    pub fn to_db(&self) -> &'static str {
        match self {
            Voice::Act => "act",
            Voice::Mid => "mid",
            Voice::Pass => "pass",
            Voice::MidPass => "mid_pass",
        }
    }
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "act" => Some(Voice::Act),
            "mid" => Some(Voice::Mid),
            "pass" => Some(Voice::Pass),
            "mid_pass" | "midpass" => Some(Voice::MidPass),
            _ => None,
        }
    }
}

// ── Mood ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mood {
    Ind,
    Subj,
    Opt,
    Imp,
    Inf,
    Part,
}

impl Mood {
    pub fn all() -> &'static [Mood] {
        &[Mood::Ind, Mood::Subj, Mood::Opt, Mood::Imp, Mood::Inf, Mood::Part]
    }
    pub fn label_ru(&self) -> &'static str {
        match self {
            Mood::Ind => "Изъяв.",
            Mood::Subj => "Сослаг.",
            Mood::Opt => "Жел.",
            Mood::Imp => "Повел.",
            Mood::Inf => "Инфин.",
            Mood::Part => "Прич.",
        }
    }
    pub fn label_en(&self) -> &'static str {
        match self {
            Mood::Ind => "Ind",
            Mood::Subj => "Subj",
            Mood::Opt => "Opt",
            Mood::Imp => "Imp",
            Mood::Inf => "Inf",
            Mood::Part => "Part",
        }
    }
    pub fn to_db(&self) -> &'static str {
        match self {
            Mood::Ind => "ind",
            Mood::Subj => "subj",
            Mood::Opt => "opt",
            Mood::Imp => "imp",
            Mood::Inf => "inf",
            Mood::Part => "part",
        }
    }
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "ind" => Some(Mood::Ind),
            "subj" => Some(Mood::Subj),
            "opt" => Some(Mood::Opt),
            "imp" => Some(Mood::Imp),
            "inf" => Some(Mood::Inf),
            "part" => Some(Mood::Part),
            _ => None,
        }
    }
}

// ── Person ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Person {
    P1,
    P2,
    P3,
}

impl Person {
    pub fn all() -> &'static [Person] {
        &[Person::P1, Person::P2, Person::P3]
    }
    pub fn label_ru(&self) -> &'static str {
        match self {
            Person::P1 => "1-е",
            Person::P2 => "2-е",
            Person::P3 => "3-е",
        }
    }
    pub fn label_en(&self) -> &'static str {
        match self {
            Person::P1 => "1st",
            Person::P2 => "2nd",
            Person::P3 => "3rd",
        }
    }
    pub fn to_db(&self) -> &'static str {
        match self {
            Person::P1 => "1",
            Person::P2 => "2",
            Person::P3 => "3",
        }
    }
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "1" => Some(Person::P1),
            "2" => Some(Person::P2),
            "3" => Some(Person::P3),
            _ => None,
        }
    }
}

// ── Degree ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Degree {
    Pos,
    Comp,
    Superl,
}

impl Degree {
    pub fn all() -> &'static [Degree] {
        &[Degree::Pos, Degree::Comp, Degree::Superl]
    }
    pub fn label_ru(&self) -> &'static str {
        match self {
            Degree::Pos => "Полож.",
            Degree::Comp => "Сравн.",
            Degree::Superl => "Превосх.",
        }
    }
    pub fn label_en(&self) -> &'static str {
        match self {
            Degree::Pos => "Pos",
            Degree::Comp => "Comp",
            Degree::Superl => "Superl",
        }
    }
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pos" => Some(Degree::Pos),
            "comp" => Some(Degree::Comp),
            "superl" => Some(Degree::Superl),
            _ => None,
        }
    }
}

// ── DeclType ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeclType {
    // 2nd declension
    #[serde(rename = "2a_os")]
    Os,
    #[serde(rename = "2b_on")]
    On,
    #[serde(rename = "2c_attic")]
    Attic,
    #[serde(rename = "2d_contract")]
    Contract,
    // 3rd declension
    #[serde(rename = "3a_sonant")]
    Sonant,
    #[serde(rename = "3b_labial")]
    Labial,
    #[serde(rename = "3c_velar")]
    Velar,
    #[serde(rename = "3d_dental")]
    Dental,
    #[serde(rename = "3e_nt")]
    Nt,
    #[serde(rename = "3f_sigma")]
    Sigma,
    #[serde(rename = "3g_vowel_iu")]
    VowelIU,
    #[serde(rename = "3h_diphth")]
    Diphth,
    #[serde(rename = "3i_irreg_r")]
    IrregR,
    #[serde(rename = "3_unknown")]
    Unknown3,
    #[serde(other)]
    Other,
}

impl DeclType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "2a_os" => Some(DeclType::Os),
            "2b_on" => Some(DeclType::On),
            "2c_attic" => Some(DeclType::Attic),
            "2d_contract" => Some(DeclType::Contract),
            "3a_sonant" => Some(DeclType::Sonant),
            "3b_labial" => Some(DeclType::Labial),
            "3c_velar" => Some(DeclType::Velar),
            "3d_dental" => Some(DeclType::Dental),
            "3e_nt" => Some(DeclType::Nt),
            "3f_sigma" => Some(DeclType::Sigma),
            "3g_vowel_iu" => Some(DeclType::VowelIU),
            "3h_diphth" => Some(DeclType::Diphth),
            "3i_irreg_r" => Some(DeclType::IrregR),
            "3_unknown" => Some(DeclType::Unknown3),
            _ => None,
        }
    }
    pub fn label_ru(&self) -> &'static str {
        match self {
            DeclType::Os => "2-е скл. (-ος)",
            DeclType::On => "2-е скл. (-ον)",
            DeclType::Attic => "2-е скл. аттич.",
            DeclType::Contract => "2-е скл. контракт.",
            DeclType::Sonant => "3-е скл. сонант.",
            DeclType::Labial => "3-е скл. губной",
            DeclType::Velar => "3-е скл. задненёбный",
            DeclType::Dental => "3-е скл. зубной",
            DeclType::Nt => "3-е скл. (-ντ)",
            DeclType::Sigma => "3-е скл. (-σ)",
            DeclType::VowelIU => "3-е скл. (-ι/-υ)",
            DeclType::Diphth => "3-е скл. дифтонг",
            DeclType::IrregR => "3-е скл. нерег. (-ρ)",
            DeclType::Unknown3 => "3-е скл. (неизв.)",
            DeclType::Other => "Другое",
        }
    }
}

// ── ConjType ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConjType {
    Thematic,
    ThematicCons,
    ContractAo,
    ContractEo,
    ContractOo,
    MiVerb,
}

impl ConjType {
    pub fn all() -> &'static [ConjType] {
        &[
            ConjType::Thematic,
            ConjType::ThematicCons,
            ConjType::ContractAo,
            ConjType::ContractEo,
            ConjType::ContractOo,
            ConjType::MiVerb,
        ]
    }
    pub fn label_ru(&self) -> &'static str {
        match self {
            ConjType::Thematic => "Тематич. (-ω)",
            ConjType::ThematicCons => "Тематич. согл.",
            ConjType::ContractAo => "Контракт. (-άω)",
            ConjType::ContractEo => "Контракт. (-έω)",
            ConjType::ContractOo => "Контракт. (-όω)",
            ConjType::MiVerb => "Атематич. (-μι)",
        }
    }
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "thematic" => Some(ConjType::Thematic),
            "thematic_cons" => Some(ConjType::ThematicCons),
            "contract_ao" => Some(ConjType::ContractAo),
            "contract_eo" => Some(ConjType::ContractEo),
            "contract_oo" => Some(ConjType::ContractOo),
            "mi_verb" => Some(ConjType::MiVerb),
            _ => None,
        }
    }
}
