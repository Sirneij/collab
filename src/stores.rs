use crate::types::{answer::Answer, question::Question, question::QuestionID};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Store {
    pub questions: Arc<RwLock<HashMap<QuestionID, Question>>>,
    pub answers: Arc<RwLock<HashMap<String, Answer>>>,
}
impl Store {
    pub fn new() -> Self {
        Store {
            questions: Arc::new(RwLock::new(Store::init())),
            answers: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    fn init() -> HashMap<QuestionID, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("Can't read file")
    }
}
