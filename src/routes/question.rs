use crate::stores;
use crate::types;
use handle_errors;

use std::collections::HashMap;
use tracing::{event, instrument, Level};
use warp::{http::StatusCode, Rejection, Reply};

#[instrument]
pub async fn get_questions(
    params: HashMap<String, String>,
    store: stores::Store,
) -> Result<impl Reply, Rejection> {
    event!(target: "collab", Level::INFO, "querying questions");
    if !params.is_empty() {
        let pagination = types::pagination::extract_pagination(params)?;
        event!(Level::INFO, pagination = true);
        let res: Vec<types::question::Question> =
            store.questions.read().values().cloned().collect();
        // let res = &res[pagination.start..pagination.end];
        match res.get(pagination.start..pagination.end) {
            Some(v) => Ok(warp::reply::json(&v)),
            None => Err(warp::reject::custom(handle_errors::CustomError::OutOfBound)),
        }
    } else {
        event!(Level::INFO, pagination = false);
        let res: Vec<types::question::Question> =
            store.questions.read().values().cloned().collect();
        Ok(warp::reply::json(&res))
    }
}

#[instrument]
pub async fn get_one_question(id: String, store: stores::Store) -> Result<impl Reply, Rejection> {
    event!(target: "collab", Level::INFO, "querying one question");
    match store.questions.read().get(&types::question::QuestionID(id)) {
        Some(q) => {
            event!(Level::INFO, success = true);
            Ok(warp::reply::json(q))
        }
        None => {
            event!(Level::ERROR, success = false);
            Err(warp::reject::custom(
                handle_errors::CustomError::QuestionNotFound,
            ))
        }
    }
}

#[instrument]
pub async fn add_question(
    store: stores::Store,
    question: types::question::Question,
) -> Result<impl Reply, Rejection> {
    store
        .questions
        .write()
        .insert(question.clone().id, question);

    Ok(warp::reply::with_status(
        "Question successfully added",
        StatusCode::CREATED,
    ))
}

#[instrument]
pub async fn update_question(
    id: String,
    store: stores::Store,
    question: types::question::Question,
) -> Result<impl Reply, Rejection> {
    match store
        .questions
        .write()
        .get_mut(&types::question::QuestionID(id))
    {
        Some(q) => *q = question,
        None => {
            return Err(warp::reject::custom(
                handle_errors::CustomError::QuestionNotFound,
            ))
        }
    }
    Ok(warp::reply::with_status("Question updated", StatusCode::OK))
}

#[instrument]
pub async fn delete_question(id: String, store: stores::Store) -> Result<impl Reply, Rejection> {
    match store
        .questions
        .write()
        .remove(&types::question::QuestionID(id))
    {
        Some(_) => Ok(warp::reply::with_status("Question deleted", StatusCode::OK)),
        None => Err(warp::reject::custom(
            handle_errors::CustomError::QuestionNotFound,
        )),
    }
}
