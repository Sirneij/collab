use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;
use std::io::{Error, ErrorKind};
use std::str::FromStr;
use std::sync::Arc;

use warp::{
    filters::body::BodyDeserializeError, filters::cors::CorsForbidden, http::Method,
    http::StatusCode, reject::Reject, Filter, Rejection, Reply,
};

#[derive(Debug)]
enum CustomError {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    QuestionNotFound,
    OutOfBound,
}
impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CustomError::ParseError(ref err) => write!(f, "Cannot parse parameter: {}", err),
            CustomError::MissingParameters => write!(f, "Missing parameter"),
            CustomError::QuestionNotFound => write!(f, "Question not found"),
            CustomError::OutOfBound => write!(f, "Out of bound. The index end is out of range."),
        }
    }
}
impl Reject for CustomError {}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Question {
    id: QuestionID,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
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

#[derive(Clone)]
struct Store {
    questions: Arc<RwLock<HashMap<QuestionID, Question>>>,
}
impl Store {
    fn new() -> Self {
        Store {
            questions: Arc::new(RwLock::new(Store::init())),
        }
    }
    fn init() -> HashMap<QuestionID, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("Can't read file")
    }
}

#[derive(Debug)]
struct Pagination {
    start: usize,
    end: usize,
}

async fn return_err(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(error) = r.find::<CustomError>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::RANGE_NOT_SATISFIABLE,
        ))
    } else if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(error) = r.find::<BodyDeserializeError>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}

fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, CustomError> {
    if params.contains_key("start") && params.contains_key("end") {
        return Ok(Pagination {
            start: params
                .get("start")
                .unwrap()
                .parse::<usize>()
                .map_err(CustomError::ParseError)?,
            end: params
                .get("end")
                .unwrap()
                .parse::<usize>()
                .map_err(CustomError::ParseError)?,
        });
    }
    Err(CustomError::MissingParameters)
}

async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl Reply, Rejection> {
    if params.len() > 0 {
        let pagination = extract_pagination(params)?;
        let res: Vec<Question> = store.questions.read().values().cloned().collect();
        // let res = &res[pagination.start..pagination.end];
        match res.get(pagination.start..pagination.end) {
            Some(v) => return Ok(warp::reply::json(&v)),
            None => return Err(warp::reject::custom(CustomError::OutOfBound)),
        }
    } else {
        let res: Vec<Question> = store.questions.read().values().cloned().collect();
        Ok(warp::reply::json(&res))
    }
}

async fn get_one_question(id: String, store: Store) -> Result<impl Reply, Rejection> {
    match store.questions.read().get(&QuestionID(id)) {
        Some(q) => return Ok(warp::reply::json(q)),
        None => return Err(warp::reject::custom(CustomError::QuestionNotFound)),
    }
}

async fn add_question(store: Store, question: Question) -> Result<impl Reply, Rejection> {
    store
        .questions
        .write()
        .insert(question.clone().id, question);

    Ok(warp::reply::with_status(
        "Question successfully added",
        StatusCode::OK,
    ))
}

async fn update_question(
    id: String,
    store: Store,
    question: Question,
) -> Result<impl Reply, Rejection> {
    match store.questions.write().get_mut(&QuestionID(id)) {
        Some(q) => *q = question,
        None => return Err(warp::reject::custom(CustomError::QuestionNotFound)),
    }
    Ok(warp::reply::with_status("Question updated", StatusCode::OK))
}
async fn delete_question(id: String, store: Store) -> Result<impl Reply, Rejection> {
    match store.questions.write().remove(&QuestionID(id)) {
        Some(_) => return Ok(warp::reply::with_status("Question deleted", StatusCode::OK)),
        None => return Err(warp::reject::custom(CustomError::QuestionNotFound)),
    }
}
#[tokio::main]
async fn main() {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(&[Method::PUT, Method::DELETE]);
    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(get_questions);

    let get_question = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_one_question);

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(add_question);

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(update_question);

    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(delete_question);
    let routes = get_questions
        .or(get_question)
        .or(add_question)
        .or(update_question)
        .or(delete_question)
        .with(cors)
        .recover(return_err);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
