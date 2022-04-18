use std::collections::HashMap;

use crate::stores;
use crate::types;
use warp::{http::StatusCode, Rejection, Reply};

pub async fn add_answer(
    store: stores::Store,
    params: HashMap<String, String>,
) -> Result<impl Reply, Rejection> {
    let answer = types::answer::Answer {
        id: "C10001".to_string(),
        content: params.get("content").unwrap().to_string(),
        question_id: params.get("questionID").unwrap().to_string(),
    };

    store.answers.write().insert(answer.clone().id, answer);
    Ok(warp::reply::with_status(
        "Answer successfully added.",
        StatusCode::CREATED,
    ))
}
