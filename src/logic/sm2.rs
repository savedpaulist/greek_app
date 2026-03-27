use crate::models::progress::{now_secs, ProgressRecord};

/// Answer quality: 0–5
/// 5 = instant recall, 3 = correct with effort, 0 = complete failure
pub type Quality = u8;

/// Apply one SM-2 step to a progress record.
pub fn sm2_update(rec: &mut ProgressRecord, quality: Quality) {
    debug_assert!(quality <= 5);
    let q = quality as f32;

    if quality >= 3 {
        // Correct answer
        rec.correct += 1;
        rec.streak += 1;

        rec.interval_days = match rec.streak {
            1 => 1,
            2 => 6,
            _ => (rec.interval_days as f32 * rec.ease_factor).round() as u32,
        };

        // Update ease factor: EF' = EF + (0.1 - (5-q)*(0.08 + (5-q)*0.02))
        rec.ease_factor += 0.1 - (5.0 - q) * (0.08 + (5.0 - q) * 0.02);
        rec.ease_factor = rec.ease_factor.max(1.3);
    } else {
        // Incorrect — reset streak and interval
        rec.incorrect += 1;
        rec.streak = 0;
        rec.interval_days = 1;
    }

    rec.last_seen = now_secs();
    rec.next_review = rec.last_seen + rec.interval_days as i64 * 86_400;
}

/// Map answer correctness + hint usage to SM-2 quality.
pub fn quality_from_answer(correct: bool, used_hint: bool) -> Quality {
    match (correct, used_hint) {
        (true, false) => 5,
        (true, true) => 3,
        (false, _) => 1,
    }
}
