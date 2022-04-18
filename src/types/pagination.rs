use handle_errors;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Pagination {
    pub start: usize,
    pub end: usize,
}

pub fn extract_pagination(
    params: HashMap<String, String>,
) -> Result<Pagination, handle_errors::CustomError> {
    if params.contains_key("start") && params.contains_key("end") {
        return Ok(Pagination {
            start: params
                .get("start")
                .unwrap()
                .parse::<usize>()
                .map_err(handle_errors::CustomError::ParseError)?,
            end: params
                .get("end")
                .unwrap()
                .parse::<usize>()
                .map_err(handle_errors::CustomError::ParseError)?,
        });
    }
    Err(handle_errors::CustomError::MissingParameters)
}
