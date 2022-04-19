use handle_errors;
use std::collections::HashMap;

/// Pagination struct which gets extracted from query paramenters
#[derive(Debug)]
pub struct Pagination {
    /// The index of the first item which has to be returned
    pub start: usize,
    /// The index of the last item which has to be returned
    pub end: usize,
}

/// Extract query paramenters from the `/questions` URL
/// # Example request
/// GET requests to `/questions` are paginated by the `start` and `end` parameters. None is optional if used.
///
/// `/questions?start=1&end=10`
/// # Example usage
/// ```rust
/// let q = HashMap::new();
/// q.push("start", 1);
/// q.push("end", 10);
/// let p = types::pagination::extract_pagination(q).unwrap();
/// assert_eq!(p.start, 1);
/// assert_eq!(p.end, 10);
/// ```
pub fn extract_pagination(
    params: HashMap<String, String>,
) -> Result<Pagination, handle_errors::CustomError> {
    // Could be improved in the future
    if params.contains_key("start") && params.contains_key("end") {
        return Ok(Pagination {
            // Takes the "start" parameter in the query and tries to convert it to a number
            start: params
                .get("start")
                .unwrap()
                .parse::<usize>()
                .map_err(handle_errors::CustomError::ParseError)?,
            // Takes the "end" parameter in the query and tries to convert it to a number
            end: params
                .get("end")
                .unwrap()
                .parse::<usize>()
                .map_err(handle_errors::CustomError::ParseError)?,
        });
    }
    Err(handle_errors::CustomError::MissingParameters)
}
