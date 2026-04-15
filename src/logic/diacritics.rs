use unicode_normalization::UnicodeNormalization;

/// All forms of the Greek definite article, used for optional-article comparison.
const GREEK_ARTICLES: &[&str] = &[
    // singular nominative
    "ὁ", "ἡ", "τό", "τὸ",
    // singular genitive
    "τοῦ", "τῆς",
    // singular dative
    "τῷ", "τῇ",
    // singular accusative
    "τόν", "τήν", "τὸν", "τὴν",
    // plural nominative
    "οἱ", "αἱ", "τά", "τὰ",
    // plural genitive
    "τῶν",
    // plural dative
    "τοῖς", "ταῖς",
    // plural accusative
    "τούς", "τάς", "τοὺς", "τὰς",
    // dual
    "τώ", "τοῖν",
];

/// If `s` starts with a Greek definite article followed by a space,
/// returns the remainder (trimmed). Otherwise returns `s` unchanged.
pub fn strip_leading_article(s: &str) -> &str {
    let s = s.trim();
    for art in GREEK_ARTICLES {
        if let Some(rest) = s.strip_prefix(art) {
            // Require whitespace after the article so that a word like "τῆσδε"
            // is not mistaken for article "τῆς" + remainder "δε".
            if rest.starts_with(char::is_whitespace) {
                let rest = rest.trim_start();
                if !rest.is_empty() {
                    return rest;
                }
            }
        }
    }
    s
}

/// Normalise a Greek string for comparison.
/// Vowel-length marks (macron U+0304, breve U+0306) are always stripped —
/// they are not required in answers regardless of the diacritics setting.
/// If `strip` is true, all other combining diacritical marks (accents,
/// breathings, iota subscript, etc.) are also stripped, leaving only base
/// letters.
pub fn normalize(s: &str, strip: bool) -> String {
    // First decompose to NFD so combining marks are separate code points.
    let nfd: String = s.nfd().collect();
    // Always remove vowel-length marks.
    let without_length: String = nfd.chars().filter(|c| !is_length_mark(*c)).collect();
    if strip {
        // Remove all remaining combining characters.
        without_length
            .chars()
            .filter(|c| !is_combining(*c))
            .collect::<String>()
            .nfc()
            .collect()
    } else {
        without_length.nfc().collect()
    }
}

/// Compare two Greek strings.
/// `ignore_diacritics` strips all combining marks before comparing.
/// The leading definite article in `expected` is optional — the answer is
/// accepted both with and without it.
pub fn compare_greek(answer: &str, expected: &str, ignore_diacritics: bool) -> bool {
    let a = normalize(answer.trim(), ignore_diacritics).to_lowercase();
    let e = normalize(expected.trim(), ignore_diacritics).to_lowercase();
    if a == e {
        return true;
    }
    // Also accept answer without the leading article.
    let stripped = strip_leading_article(expected.trim());
    if stripped != expected.trim() {
        let e2 = normalize(stripped, ignore_diacritics);
        if a == e2 {
            return true;
        }
    }
    false
}

/// Returns a list of (char, correct: bool) pairs for character-level diff
/// highlighting in fill-in mode.
pub fn diff_chars(answer: &str, expected: &str, ignore_diacritics: bool) -> Vec<(char, bool)> {
    // Build expected char list from the normalised+lowercased expected string.
    // Normalising the whole expected string is safe here — it doesn't need to
    // stay index-aligned with anything.
    let e_norm: Vec<char> = normalize(expected.trim(), ignore_diacritics)
        .to_lowercase()
        .chars()
        .collect();

    // For each RAW character of the answer we normalise it individually so
    // the index in `answer.chars()` always matches the index in `e_norm`.
    // Normalising the whole answer string at once can change the char count
    // (e.g. a length-mark combining character gets stripped, collapsing two
    // code-points into one) and would shift every subsequent comparison.
    answer
        .trim()
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let c_key: String = normalize(&c.to_string(), ignore_diacritics)
                .to_lowercase();
            let e_key: String = e_norm.get(i).map(|ch| ch.to_string()).unwrap_or_default();
            (c, c_key == e_key)
        })
        .collect()
}

/// Strip only accent marks (acute, grave, circumflex/perispomeni), leaving
/// other diacritics (breathings, iota subscript, diaeresis) in place.
/// Length marks are also stripped (they are never required in answers).
pub fn strip_only_accents(s: &str) -> String {
    let nfd: String = s.nfd().collect();
    nfd.chars()
        .filter(|c| !is_length_mark(*c) && !is_accent_mark(*c))
        .collect::<String>()
        .nfc()
        .collect()
}

/// Classic Levenshtein edit distance on char slices.
pub fn levenshtein(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let (n, m) = (a.len(), b.len());
    if n == 0 { return m; }
    if m == 0 { return n; }
    let mut prev: Vec<usize> = (0..=m).collect();
    let mut curr: Vec<usize> = vec![0; m + 1];
    for i in 1..=n {
        curr[0] = i;
        for j in 1..=m {
            let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            curr[j] = (prev[j] + 1)
                .min(curr[j - 1] + 1)
                .min(prev[j - 1] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[m]
}

/// Similarity score between a candidate Greek form and the correct answer.
/// Lower = more similar. Tuple is sorted lexicographically:
///   (base-letter edit distance, non-accent diacritic distance)
/// So `(0, 0)` means identical letters and only accents differ (best tier for
/// a wrong-but-very-similar distractor).
pub fn similarity_score(candidate: &str, correct: &str) -> (usize, usize) {
    let cand_letters = normalize(candidate, true).to_lowercase();
    let corr_letters = normalize(correct, true).to_lowercase();
    let d_letters = levenshtein(&cand_letters, &corr_letters);

    let cand_no_accent = strip_only_accents(candidate).to_lowercase();
    let corr_no_accent = strip_only_accents(correct).to_lowercase();
    let d_no_accent = levenshtein(&cand_no_accent, &corr_no_accent);

    // d_no_accent >= d_letters always (stripping less info can't decrease edits).
    let diacritic_only = d_no_accent.saturating_sub(d_letters);
    (d_letters, diacritic_only)
}

fn is_accent_mark(c: char) -> bool {
    matches!(c as u32,
        0x0300  // COMBINING GRAVE ACCENT
        | 0x0301  // COMBINING ACUTE ACCENT
        | 0x0342  // COMBINING GREEK PERISPOMENI (circumflex)
    )
}

fn is_length_mark(c: char) -> bool {
    matches!(c as u32,
        0x0304  // COMBINING MACRON  (long vowel: ᾱ, ῑ, ῡ)
        | 0x0306  // COMBINING BREVE   (short vowel: ᾰ, ῐ, ῠ)
    )
}

fn is_combining(c: char) -> bool {
    // Unicode combining diacritical marks blocks
    matches!(c as u32,
        0x0300..=0x036F  // Combining Diacritical Marks
        | 0x1DC0..=0x1DFF  // Combining Diacritical Marks Supplement
    )
}
