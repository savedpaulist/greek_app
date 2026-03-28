use crate::models::form::{Form, Lemma};
use crate::state::settings::UiLanguage;

/// A single cell in a paradigm table.
#[derive(Debug, Clone)]
pub struct ParadigmCell {
    pub form: Option<Form>,
}

/// A fully built paradigm table ready for rendering.
#[derive(Debug, Clone)]
pub struct ParadigmTable {
    pub lemma: Lemma,
    /// For verb sub-tables: "Pres, Ind, Act" etc. None for nominal paradigms.
    pub title: Option<String>,
    /// Raw DB keys for filtering (verbs only).
    pub tense_key: Option<String>,
    pub mood_key: Option<String>,
    pub voice_key: Option<String>,
    pub col_headers: Vec<String>,
    pub row_headers: Vec<String>,
    pub cells: Vec<Vec<ParadigmCell>>, // [row][col]
}

/// Build a noun/adjective paradigm table (rows = cases, cols = number × gender).
pub fn build_nominal_paradigm(
    lemma: Lemma,
    forms: &[Form],
    include_dual: bool,
    genders: &[&str],
    lang: &UiLanguage,
) -> ParadigmTable {
    let cases = ["nom", "gen", "dat", "acc", "voc"];
    let numbers: Vec<&str> = if include_dual {
        vec!["sg", "du", "pl"]
    } else {
        vec!["sg", "pl"]
    };

    let mut col_headers = vec![];
    let mut col_keys: Vec<(&str, &str)> = vec![];
    for &num in &numbers {
        for &gen in genders {
            col_headers.push(format!("{} {}", num_label(num, lang), gender_label(gen, lang)));
            col_keys.push((num, gen));
        }
    }

    let row_headers: Vec<String> = cases.iter().map(|c| case_label(c, lang).to_owned()).collect();

    let cells: Vec<Vec<ParadigmCell>> = cases
        .iter()
        .map(|&case| {
            col_keys
                .iter()
                .map(|&(num, gen)| {
                    let form = forms
                        .iter()
                        .find(|f| {
                            f.case_tag.as_deref() == Some(case)
                                && f.number_tag.as_deref() == Some(num)
                                && (genders.len() == 1
                                    || f.gender_tag.as_deref() == Some(gen))
                        })
                        .cloned();
                    ParadigmCell { form }
                })
                .collect()
        })
        .collect();

    ParadigmTable {
        lemma,
        title: None,
        tense_key: None,
        mood_key: None,
        voice_key: None,
        col_headers,
        row_headers,
        cells,
    }
}

/// Build multiple verb paradigm tables — one per tense × mood × voice combination.
/// Rows = person (2nd/3rd for imperative, 1st/2nd/3rd otherwise).
/// Cols = number (sg, [du,] pl).
/// Infinitives are collected into one extra table at the end.
pub fn build_verb_paradigms(
    lemma: Lemma,
    forms: &[Form],
    include_dual: bool,
    lang: &UiLanguage,
) -> Vec<ParadigmTable> {
    // ── Finite forms: ind / subj / opt / imp ──────────────────────────────
    let finite_moods = ["ind", "subj", "opt", "imp"];

    let mut tmv_set: Vec<(String, String, String)> = vec![];
    for f in forms {
        let m = f.mood_tag.as_deref().unwrap_or_default();
        if !finite_moods.contains(&m) {
            continue;
        }
        let t = f.tense_tag.clone().unwrap_or_default();
        let v = f.voice_tag.clone().unwrap_or_default();
        let key = (t, m.to_string(), v);
        if !tmv_set.contains(&key) {
            tmv_set.push(key);
        }
    }
    tmv_set.sort_by_key(|(t, m, v)| (tense_order(t), mood_order(m), voice_order(v)));

    let numbers: Vec<&str> = if include_dual {
        vec!["sg", "du", "pl"]
    } else {
        vec!["sg", "pl"]
    };

    let col_headers: Vec<String> = numbers.iter()
        .map(|n| num_label(n, lang).to_string())
        .collect();

    let mut tables: Vec<ParadigmTable> = vec![];

    for (tense, mood, voice) in &tmv_set {
        // Forms belonging to this tense/mood/voice combination.
        let group: Vec<&Form> = forms.iter()
            .filter(|f| {
                f.tense_tag.as_deref() == Some(tense.as_str())
                    && f.mood_tag.as_deref() == Some(mood.as_str())
                    && f.voice_tag.as_deref() == Some(voice.as_str())
            })
            .collect();

        // Imperative has only 2nd and 3rd person; all others have 1st/2nd/3rd.
        let candidate_persons: &[&str] = if mood == "imp" {
            &["2", "3"]
        } else {
            &["1", "2", "3"]
        };

        // Keep only persons that actually appear in the data.
        let persons: Vec<&str> = candidate_persons.iter()
            .copied()
            .filter(|&p| group.iter().any(|f| f.person_tag.as_deref() == Some(p)))
            .collect();

        if persons.is_empty() {
            continue;
        }

        let row_headers: Vec<String> = persons.iter()
            .map(|p| person_label(p, lang).to_string())
            .collect();

        let cells: Vec<Vec<ParadigmCell>> = persons.iter()
            .map(|&person| {
                numbers.iter()
                    .map(|&number| {
                        let form = group.iter()
                            .find(|f| {
                                f.person_tag.as_deref() == Some(person)
                                    && f.number_tag.as_deref() == Some(number)
                            })
                            .map(|f| (*f).clone());
                        ParadigmCell { form }
                    })
                    .collect()
            })
            .collect();

        let title = format!(
            "{}, {}, {}",
            tense_label(tense, lang),
            mood_label(mood, lang),
            voice_label(voice, lang),
        );

        tables.push(ParadigmTable {
            lemma: lemma.clone(),
            title: Some(title),
            tense_key: Some(tense.clone()),
            mood_key: Some(mood.clone()),
            voice_key: Some(voice.clone()),
            col_headers: col_headers.clone(),
            row_headers,
            cells,
        });
    }

    // ── Infinitives ────────────────────────────────────────────────────────
    let inf_forms: Vec<&Form> = forms.iter()
        .filter(|f| f.mood_tag.as_deref() == Some("inf"))
        .collect();

    if !inf_forms.is_empty() {
        let mut tv_set: Vec<(String, String)> = vec![];
        for f in &inf_forms {
            let t = f.tense_tag.clone().unwrap_or_default();
            let v = f.voice_tag.clone().unwrap_or_default();
            let key = (t, v);
            if !tv_set.contains(&key) {
                tv_set.push(key);
            }
        }
        tv_set.sort_by_key(|(t, v)| (tense_order(t), voice_order(v)));

        let inf_col_headers: Vec<String> = tv_set.iter()
            .map(|(t, v)| format!("{} {}", tense_label(t, lang), voice_label(v, lang)))
            .collect();

        let cells: Vec<Vec<ParadigmCell>> = vec![
            tv_set.iter()
                .map(|(t, v)| {
                    let form = inf_forms.iter()
                        .find(|f| {
                            f.tense_tag.as_deref() == Some(t.as_str())
                                && f.voice_tag.as_deref() == Some(v.as_str())
                        })
                        .map(|f| (*f).clone());
                    ParadigmCell { form }
                })
                .collect()
        ];

        let inf_title = match lang {
            UiLanguage::Ru => "Инфинитив".to_string(),
            UiLanguage::En => "Infinitive".to_string(),
        };

        tables.push(ParadigmTable {
            lemma: lemma.clone(),
            title: Some(inf_title),
            tense_key: None,
            mood_key: Some("inf".to_string()),
            voice_key: None,
            col_headers: inf_col_headers,
            row_headers: vec!["".to_string()],
            cells,
        });
    }

    tables
}

// ── Label helpers ──────────────────────────────────────────────────────────

fn case_label(s: &str, lang: &UiLanguage) -> &'static str {
    match lang {
        UiLanguage::En => match s {
            "nom" => "Nom", "gen" => "Gen", "dat" => "Dat", "acc" => "Acc", "voc" => "Voc",
            _ => "?",
        },
        UiLanguage::Ru => match s {
            "nom" => "Им.", "gen" => "Рд.", "dat" => "Дт.", "acc" => "Вн.", "voc" => "Зв.",
            _ => "?",
        },
    }
}

pub fn num_label(s: &str, lang: &UiLanguage) -> &'static str {
    match lang {
        UiLanguage::En => match s {
            "sg" => "Sg", "du" => "Du", "pl" => "Pl", _ => "?",
        },
        UiLanguage::Ru => match s {
            "sg" => "Ед.", "du" => "Дв.", "pl" => "Мн.", _ => "?",
        },
    }
}

fn gender_label(s: &str, lang: &UiLanguage) -> &'static str {
    match lang {
        UiLanguage::En => match s {
            "m" => "M", "f" => "F", "n" => "N", _ => "?",
        },
        UiLanguage::Ru => match s {
            "m" => "М.р.", "f" => "Ж.р.", "n" => "Ср.р.", _ => "?",
        },
    }
}

fn person_label(s: &str, lang: &UiLanguage) -> &'static str {
    match lang {
        UiLanguage::En => match s {
            "1" => "1st", "2" => "2nd", "3" => "3rd", _ => "?",
        },
        UiLanguage::Ru => match s {
            "1" => "1-е", "2" => "2-е", "3" => "3-е", _ => "?",
        },
    }
}

pub fn tense_label<'a>(s: &'a str, lang: &UiLanguage) -> &'a str {
    match lang {
        UiLanguage::En => match s {
            "pres" => "Pres", "imperf" => "Impf", "fut" => "Fut",
            "aor1" => "Aor.I", "aor2" => "Aor.II", "aor_pass" => "Aor.Pass",
            "perf" => "Perf", "pluperf" => "Plupf", _ => s,
        },
        UiLanguage::Ru => match s {
            "pres" => "Наст.", "imperf" => "Импф.", "fut" => "Буд.",
            "aor1" => "Аор.I", "aor2" => "Аор.II", "aor_pass" => "Аор.пас.",
            "perf" => "Перф.", "pluperf" => "Плкв.", _ => s,
        },
    }
}

pub fn voice_label<'a>(s: &'a str, lang: &UiLanguage) -> &'a str {
    match lang {
        UiLanguage::En => match s {
            "act" => "Act", "mid" => "Mid", "pass" => "Pass", "mid_pass" => "M/P", _ => s,
        },
        UiLanguage::Ru => match s {
            "act" => "Акт.", "mid" => "Мед.", "pass" => "Пас.", "mid_pass" => "М/П", _ => s,
        },
    }
}

pub fn mood_label<'a>(s: &'a str, lang: &UiLanguage) -> &'a str {
    match lang {
        UiLanguage::En => match s {
            "ind" => "Ind", "subj" => "Subj", "opt" => "Opt", "imp" => "Imp", _ => s,
        },
        UiLanguage::Ru => match s {
            "ind" => "Изъяв.", "subj" => "Сослаг.", "opt" => "Желат.", "imp" => "Повел.", _ => s,
        },
    }
}

pub fn tense_order(s: &str) -> usize {
    match s {
        "pres" => 0, "imperf" => 1, "fut" => 2, "aor1" => 3, "aor2" => 4,
        "aor_pass" => 5, "perf" => 6, "pluperf" => 7, "futperf" => 8, _ => 99,
    }
}

pub fn voice_order(s: &str) -> usize {
    match s { "act" => 0, "mid" => 1, "pass" => 2, "mid_pass" => 3, _ => 4 }
}

pub fn mood_order(s: &str) -> usize {
    match s { "ind" => 0, "subj" => 1, "opt" => 2, "imp" => 3, _ => 4 }
}
