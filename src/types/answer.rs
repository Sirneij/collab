use serde::{Deserialize, Serialize};

/// Answer `struct` for storing answers to each question.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Answer {
    /// The unique identification for the answer provided
    pub id: String,
    /// The real content of the provided answer
    pub content: String,
    /// The `ID` of the answered question.
    pub question_id: String,
}
