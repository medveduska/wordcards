use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct Flashcard {
    pub word: String,
    pub pinyin: Option<String>,
    pub translation: String,
    #[serde(default)]
    pub known: bool,
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct Dataset {
    pub name: String,
    pub flashcards: Vec<Flashcard>,
    pub known_cards: Vec<Flashcard>,
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FlashcardStage {
    First,
    Second,
    Third,
}

impl Default for FlashcardStage {
    fn default() -> Self {
        Self::First
    }
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum StudyDirection {
    Normal,
    Reverse,
}

impl Default for StudyDirection {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct PersistedState {
    pub flashcards: Vec<Flashcard>,
    pub known_cards: Vec<Flashcard>,
    pub current_index: usize,
    pub stage: FlashcardStage,
    pub direction: StudyDirection,
    #[serde(default)]
    pub current_dataset: String,
}
