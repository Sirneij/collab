use handle_errors;
use std::collections::HashMap;

/// Pagination struct which gets extracted from query paramenters
#[derive(Debug, Default)]
pub struct Pagination {
    /// The index of the first item which has to be returned
    pub offset: i32,
    /// The index of the last item which has to be returned
    pub limit: Option<i32>,
}

/// Extract query paramenters from the `/questions` URL
/// # Example request
/// GET requests to `/questions` are paginated by the `offset` and `limit` parameters. None is optional if used.
///
/// `/questions?offset=1&limit=10`
/// # Example usage
/// ```rust
/// let q = HashMap::new();
/// q.push("offset", 1);
/// q.push("limit", 10);
/// let p = types::pagination::extract_pagination(q).unwrap();
/// assert_eq!(p.offset, 1);
/// assert_eq!(p.limit, 10);
/// ```
pub fn extract_pagination(
    params: HashMap<String, String>,
) -> Result<Pagination, handle_errors::CustomError> {
    // Could be improved in the future
    if params.contains_key("offset") && params.contains_key("limit") {
        return Ok(Pagination {
            // Takes the "offset" parameter in the query and tries to convert it to a number
            offset: params
                .get("offset")
                .unwrap()
                .parse()
                .map_err(handle_errors::CustomError::ParseError)?,
            // Takes the "limit" parameter in the query and tries to convert it to a number
            limit: Some(
                params
                    .get("limit")
                    .unwrap()
                    .parse()
                    .map_err(handle_errors::CustomError::ParseError)?,
            ),
        });
    }
    Err(handle_errors::CustomError::MissingParameters)
}
