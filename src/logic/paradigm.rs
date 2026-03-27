use crate::models::form::{Form, Lemma};

/// A single cell in a paradigm table.
#[derive(Debug, Clone)]
pub struct ParadigmCell {
    pub form: Option<Form>,
}

/// A fully built paradigm table ready for rendering.
#[derive(Debug, Clone)]
pub struct ParadigmTable {
    pub lemma: Lemma,
    pub col_headers: Vec<String>,
    pub row_headers: Vec<String>,
    pub cells: Vec<Vec<ParadigmCell>>, // [row][col]
}

/// Build a noun/adjective paradigm table (rows = cases, cols = numbers).
pub fn build_nominal_paradigm(
    lemma: Lemma,
    forms: &[Form],
    include_dual: bool,
    genders: &[&str],
) -> ParadigmTable {
    let cases = ["nom", "gen", "dat", "acc", "voc"];
    let numbers: Vec<&str> = if include_dual {
        vec!["sg", "du", "pl"]
    } else {
        vec!["sg", "pl"]
    };

    // Column headers: number × gender combinations
    let mut col_headers = vec![];
    let mut col_keys: Vec<(&str, &str)> = vec![];
    for &num in &numbers {
        for &gen in genders {
            col_headers.push(format!("{} {}", num_label(num), gender_label(gen)));
            col_keys.push((num, gen));
        }
    }

    let row_headers: Vec<String> = cases.iter().map(|c| case_label(c).to_owned()).collect();

    let cells: Vec<Vec<ParadigmCell>> = cases
        .iter()
        .enumerate()
        .map(|(row, &case)| {
            col_keys
                .iter()
                .enumerate()
                .map(|(col, &(num, gen))| {
                    let form = forms
                        .iter()
                        .find(|f| {
                            f.case_tag.as_deref() == Some(case)
                                && f.number_tag.as_deref() == Some(num)
                                && (genders.len() == 1
                                    || f.gender_tag.as_deref() == Some(gen))
                        })
                        .cloned();
                    let _ = (row, col);
                    ParadigmCell { form }
                })
                .collect()
        })
        .collect();

    ParadigmTable { lemma, col_headers, row_headers, cells }
}

/// Build a verb paradigm table (rows = person×number, cols = tense×voice×mood).
pub fn build_verb_paradigm(lemma: Lemma, forms: &[Form]) -> ParadigmTable {
    // Find all tense+voice+mood combinations actually present in forms
    let mut tmv_set: Vec<(String, String, String)> = vec![];
    for f in forms {
        let t = f.tense_tag.clone().unwrap_or_default();
        let v = f.voice_tag.clone().unwrap_or_default();
        let m = f.mood_tag.clone().unwrap_or_default();
        let key = (t.clone(), m.clone(), v.clone());
        if !tmv_set.contains(&key) {
            tmv_set.push(key);
        }
    }
    // Sort by canonical order
    tmv_set.sort_by_key(|(t, m, v)| (tense_order(t), mood_order(m), voice_order(v)));

    let person_numbers = [
        ("1", "sg"),
        ("2", "sg"),
        ("3", "sg"),
        ("1", "du"),
        ("2", "du"),
        ("3", "du"),
        ("1", "pl"),
        ("2", "pl"),
        ("3", "pl"),
    ];

    // Remove combos with no forms
    let person_numbers: Vec<_> = person_numbers
        .iter()
        .filter(|&&(p, n)| {
            forms.iter().any(|f| {
                f.person_tag.as_deref() == Some(p) && f.number_tag.as_deref() == Some(n)
            })
        })
        .collect();

    let row_headers: Vec<String> = person_numbers
        .iter()
        .map(|&&(p, n)| format!("{} {}", person_label(p), num_label(n)))
        .collect();

    let col_headers: Vec<String> = tmv_set
        .iter()
        .map(|(t, m, v)| format!("{}, {}, {}", tense_label(t), mood_label(m), voice_label(v)))
        .collect();

    let cells: Vec<Vec<ParadigmCell>> = person_numbers
        .iter()
        .enumerate()
        .map(|(row, &&(person, number))| {
            tmv_set
                .iter()
                .enumerate()
                .map(|(col, (tense, mood, voice))| {
                    let form = forms
                        .iter()
                        .find(|f| {
                            f.person_tag.as_deref() == Some(person)
                                && f.number_tag.as_deref() == Some(number)
                                && f.tense_tag.as_deref() == Some(tense.as_str())
                                && f.mood_tag.as_deref() == Some(mood.as_str())
                                && f.voice_tag.as_deref() == Some(voice.as_str())
                        })
                        .cloned();
                    let _ = (row, col);
                    ParadigmCell { form }
                })
                .collect()
        })
        .collect();

    ParadigmTable { lemma, col_headers, row_headers, cells }
}

// ── Label helpers ──────────────────────────────────────────────────────────

fn case_label(s: &str) -> &'static str {
    match s {
        "nom" => "Им.",
        "gen" => "Рд.",
        "dat" => "Дт.",
        "acc" => "Вн.",
        "voc" => "Зв.",
        _ => "?",
    }
}

fn num_label(s: &str) -> &'static str {
    match s {
        "sg" => "Ед.",
        "du" => "Дв.",
        "pl" => "Мн.",
        _ => "?",
    }
}

fn gender_label(s: &str) -> &'static str {
    match s {
        "m" => "М.р.",
        "f" => "Ж.р.",
        "n" => "Ср.р.",
        _ => "?",
    }
}

fn person_label(s: &str) -> &'static str {
    match s {
        "1" => "1-е",
        "2" => "2-е",
        "3" => "3-е",
        _ => "?",
    }
}

fn tense_label(s: &str) -> &str {
    match s {
        "pres" => "Наст.",
        "imperf" => "Импф.",
        "fut" => "Буд.",
        "aor1" => "Аор.I",
        "aor2" => "Аор.II",
        "aor_pass" => "Аор.пас.",
        "perf" => "Перф.",
        "pluperf" => "Плкв.",
        _ => s,
    }
}

fn voice_label(s: &str) -> &str {
    match s {
        "act" => "Акт.",
        "mid" => "Мед.",
        "pass" => "Пас.",
        "mid_pass" => "М/П",
        _ => s,
    }
}

fn tense_order(s: &str) -> usize {
    match s {
        "pres" => 0,
        "imperf" => 1,
        "fut" => 2,
        "aor1" => 3,
        "aor2" => 4,
        "aor_pass" => 5,
        "perf" => 6,
        "pluperf" => 7,
        "futperf" => 8,
        _ => 99,
    }
}

fn voice_order(s: &str) -> usize {
    match s {
        "act" => 0,
        "mid" => 1,
        "pass" => 2,
        "mid_pass" => 3,
        _ => 4,
    }
}

fn mood_label(s: &str) -> &str {
    match s {
        "ind" => "Изъяв.",
        "subj" => "Сослаг.",
        "opt" => "Желат.",
        "imp" => "Повел.",
        _ => s,
    }
}

fn mood_order(s: &str) -> usize {
    match s {
        "ind" => 0,
        "subj" => 1,
        "opt" => 2,
        "imp" => 3,
        _ => 4,
    }
}
