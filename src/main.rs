use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;
use std::io::{Error, ErrorKind};
use std::str::FromStr;
use warp::{
    filters::cors::CorsForbidden, http::Method, http::StatusCode, reject::Reject, Filter,
    Rejection, Reply,
};

#[derive(Debug)]
struct InvalidID;
impl Reject for InvalidID {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Question {
    id: QuestionID,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}
impl Question {
    fn new(id: QuestionID, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
struct QuestionID(String);
impl FromStr for QuestionID {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionID(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
        }
    }
}

struct Store {
    questions: HashMap<QuestionID, Question>,
}
impl Store {
    fn new() -> Self {
        Store {
            questions: Store::init(),
        }
    }
    fn init() -> HashMap<QuestionID, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("Can't read file")
    }
    fn add_question(&mut self, question: &Question) -> &Self {
        self.questions.insert(question.id.clone(), question.clone());
        self
    }
}

async fn return_err(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(InvalidID) = r.find() {
        Ok(warp::reply::with_status(
            "No ID provided".to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}

async fn get_questions() -> Result<impl Reply, Rejection> {
    let question = Question::new(
        QuestionID::from_str("1").expect("No ID provided"),
        "First question".to_string(),
        "What is warp used for?".to_string(),
        Some(vec!["FAQ".to_string()]),
    );

    match question.id.0.is_empty() {
        true => Err(warp::reject::custom(InvalidID)),
        false => Ok(warp::reply::json(&question)),
    }
}

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(&[Method::PUT, Method::DELETE]);
    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and_then(get_questions)
        .recover(return_err);
    let routes = get_items.with(cors);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
