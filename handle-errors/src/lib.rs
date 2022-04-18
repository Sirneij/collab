use warp::{
    filters::body::BodyDeserializeError, filters::cors::CorsForbidden, http::StatusCode,
    reject::Reject, Rejection, Reply,
};
#[derive(Debug)]
pub enum CustomError {
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
            CustomError::OutOfBound => {
                write!(f, "Out of bound. The index end is out of range.")
            }
        }
    }
}
impl Reject for CustomError {}

pub async fn return_err(r: Rejection) -> Result<impl Reply, Rejection> {
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
