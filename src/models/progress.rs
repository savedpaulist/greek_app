use serde::{Deserialize, Serialize};

/// SM-2 progress record per form.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProgressRecord {
    pub form_id: i64,
    /// Total correct answers.
    pub correct: u32,
    /// Total incorrect answers.
    pub incorrect: u32,
    /// Current correct streak.
    pub streak: u32,
    /// SM-2 ease factor (min 1.3).
    pub ease_factor: f32,
    /// Current interval in days.
    pub interval_days: u32,
    /// Unix timestamp (seconds) for next scheduled review.
    pub next_review: i64,
    /// Unix timestamp of last time this form was shown.
    pub last_seen: i64,
}

impl ProgressRecord {
    pub fn new(form_id: i64) -> Self {
        Self { form_id, ease_factor: 2.5, interval_days: 1, ..Default::default() }
    }

    #[allow(dead_code)]
    pub fn accuracy(&self) -> f32 {
        let total = self.correct + self.incorrect;
        if total == 0 { 0.0 } else { self.correct as f32 / total as f32 }
    }

    pub fn is_learned(&self) -> bool {
        self.streak >= 5
    }

    pub fn is_due(&self) -> bool {
        let now = now_secs();
        self.next_review == 0 || self.next_review <= now
    }
}

pub fn now_secs() -> i64 {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::prelude::*;
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_namespace = Date)]
            fn now() -> f64;
        }
        (now() / 1000.0) as i64
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64
    }
}
