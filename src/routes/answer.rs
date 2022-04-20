use tracing::{event, instrument, Level};

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

#[instrument]
pub async fn add_answer(
    store: stores::Store,
    answer: types::answer::Answer,
) -> Result<impl Reply, Rejection> {
    event!(target: "collab", Level::INFO, "adding one answer");

    match store.add_answer(answer).await {
        Ok(_) => Ok(warp::reply::with_status(
            "Answer added",
            StatusCode::CREATED,
        )),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
