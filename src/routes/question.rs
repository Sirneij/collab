use crate::stores;
use crate::types;
use handle_errors;
use std::collections::HashMap;
use warp::{http::StatusCode, Rejection, Reply};
pub async fn get_questions(
    params: HashMap<String, String>,
    store: stores::Store,
) -> Result<impl Reply, Rejection> {
    if params.len() > 0 {
        let pagination = types::pagination::extract_pagination(params)?;
        let res: Vec<types::question::Question> =
            store.questions.read().values().cloned().collect();
        // let res = &res[pagination.start..pagination.end];
        match res.get(pagination.start..pagination.end) {
            Some(v) => return Ok(warp::reply::json(&v)),
            None => return Err(warp::reject::custom(handle_errors::CustomError::OutOfBound)),
        }
    } else {
        let res: Vec<types::question::Question> =
            store.questions.read().values().cloned().collect();
        Ok(warp::reply::json(&res))
    }
}

pub async fn get_one_question(id: String, store: stores::Store) -> Result<impl Reply, Rejection> {
    match store.questions.read().get(&types::question::QuestionID(id)) {
        Some(q) => return Ok(warp::reply::json(q)),
        None => {
            return Err(warp::reject::custom(
                handle_errors::CustomError::QuestionNotFound,
            ))
        }
    }
}

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
pub async fn delete_question(id: String, store: stores::Store) -> Result<impl Reply, Rejection> {
    match store
        .questions
        .write()
        .remove(&types::question::QuestionID(id))
    {
        Some(_) => return Ok(warp::reply::with_status("Question deleted", StatusCode::OK)),
        None => {
            return Err(warp::reject::custom(
                handle_errors::CustomError::QuestionNotFound,
            ))
        }
    }
}
