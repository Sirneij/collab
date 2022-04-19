use serde::{Deserialize, Serialize};
use std::hash::Hash;
use std::io::{Error, ErrorKind};
use std::str::FromStr;

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
pub struct QuestionID(pub String);
impl FromStr for QuestionID {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionID(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
        }
    }
}
