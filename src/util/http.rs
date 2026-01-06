use std::{any::type_name, str::FromStr};

use axum::http::{HeaderMap, StatusCode};

/// # Errors
/// * when the request header is missing or invalid
pub fn parse_header<T>(headers: &HeaderMap, header: &str) -> Result<T, (StatusCode, String)>
where
    T: FromStr,
{
    let value: T = headers
        .get(header)
        .ok_or_else(|| (StatusCode::BAD_REQUEST, format!("{header} header missing")))?
        .to_str()
        .map_err(|_| (StatusCode::BAD_REQUEST, format!("{header} header invalid")))?
        .parse()
        .map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                format!("{header} header is not a valid {}", type_name::<T>()),
            )
        })?;

    Ok(value)
}
