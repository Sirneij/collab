use serde::{Deserialize, Serialize};
use std::hash::Hash;

/// `Question` struct that models all questions being asked
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Question {
    /// Unique ID for the question being asked
    pub id: QuestionID,
    /// Title of the asked question
    pub title: String,
    /// The real content of the question
    pub content: String,
    /// Some tags, useful for filtering and categorizing questions.
    pub tags: Option<Vec<String>>,
}

/// `QuestionID` struct for each question ID
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct QuestionID(pub i32);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewQuestion {
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}
