use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Answer {
    pub id: String,
    pub content: String,
    pub question_id: String,
}
