use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SessionMode {
    ParadigmView,
    Flashcard,
    FlashcardReverse,
    FillIn,
    BuildParadigm,
    Spaced,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: u64,
    pub mode: SessionMode,
    pub started_at: i64,
    pub finished_at: Option<i64>,
    pub total_shown: u32,
    pub correct: u32,
    pub incorrect: u32,
    /// JSON-serialised FilterParams snapshot.
    pub filter_json: String,
}

impl Session {
    #[allow(dead_code)]
    pub fn accuracy(&self) -> f32 {
        let total = self.correct + self.incorrect;
        if total == 0 { 0.0 } else { self.correct as f32 / total as f32 }
    }
}
