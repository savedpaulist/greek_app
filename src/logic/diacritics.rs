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
    let a = normalize(answer.trim(), ignore_diacritics);
    let e = normalize(expected.trim(), ignore_diacritics);
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
    let a_norm: Vec<char> = normalize(answer, ignore_diacritics).chars().collect();
    let e_norm: Vec<char> = normalize(expected, ignore_diacritics).chars().collect();
    let a_raw: Vec<char> = answer.chars().collect();

    a_raw
        .iter()
        .enumerate()
        .map(|(i, &c)| {
            let correct = a_norm.get(i) == e_norm.get(i);
            (c, correct)
        })
        .collect()
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
