use serde::{Deserialize, Serialize};
use std::hash::Hash;
use std::io::{Error, ErrorKind};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Question {
    pub id: QuestionID,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

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
