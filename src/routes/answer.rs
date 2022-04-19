use std::collections::HashMap;

use crate::stores;
use crate::types;
use warp::{http::StatusCode, Rejection, Reply};

/// Handler for providing answers to a specific question. The content-type expected is `application/x-www-form-urlencoded`.
/// returns a text `"Answer successfully added."` and HTTP `201` code if successful.
///
/// # Expected parameters
/// `content` and `questionID`
///
/// # Example request
/// POST requests with the following signature is expected:
/// `/answers content="Answer content"&questionID="reference to the question being answered"`
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
