use crate::stores;
use crate::types;

use std::collections::HashMap;
use tracing::{event, instrument, Level};
use warp::{http::StatusCode, Rejection, Reply};

#[instrument]
pub async fn get_questions(
    params: HashMap<String, String>,
    store: stores::Store,
) -> Result<impl Reply, Rejection> {
    event!(target: "collab", Level::INFO, "querying questions");
    let mut pagination = types::pagination::Pagination::default();
    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = types::pagination::extract_pagination(params)?;
    }
    let res: Vec<types::question::Question> = match store
        .get_all_questions(pagination.offset, pagination.limit)
        .await
    {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };
    Ok(warp::reply::json(&res))
}

#[instrument]
pub async fn get_one_question(id: i32, store: stores::Store) -> Result<impl Reply, Rejection> {
    event!(target: "collab", Level::INFO, "querying one question");

    match store.get_a_question(id).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

#[instrument]
pub async fn add_question(
    store: stores::Store,
    new_question: types::question::NewQuestion,
) -> Result<impl Reply, Rejection> {
    event!(target: "collab", Level::INFO, "adding one question");
    match store.add_question(new_question).await {
        Ok(_) => Ok(warp::reply::with_status(
            "Question successfully added",
            StatusCode::CREATED,
        )),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

#[instrument]
pub async fn update_question(
    id: i32,
    store: stores::Store,
    question: types::question::Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    let res: types::question::Question = match store.update_question(question, id).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };
    Ok(warp::reply::json(&res))
}

#[instrument]
pub async fn delete_question(
    id: i32,
    store: stores::Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(e) = store.delete_question(id).await {
        return Err(warp::reject::custom(e));
    }
    Ok(warp::reply::with_status(
        format!("Question {} deleted", id),
        StatusCode::OK,
    ))
}
