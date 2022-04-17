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
enum CustomError {
    ParseError(std::num::ParseIntError),
    MissingParameters,
}
impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CustomError::ParseError(ref err) => write!(f, "Cannot parse parameter: {}", err),
            CustomError::MissingParameters => write!(f, "Missing parameter"),
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
        let res: Vec<Question> = store.questions.values().cloned().collect();
        let res = &res[pagination.start..pagination.end];
        Ok(warp::reply::json(&res))
    } else {
        let res: Vec<Question> = store.questions.values().cloned().collect();
        Ok(warp::reply::json(&res))
    }
}

#[tokio::main]
async fn main() {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(&[Method::PUT, Method::DELETE]);
    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(get_questions)
        .recover(return_err);
    let routes = get_items.with(cors);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
