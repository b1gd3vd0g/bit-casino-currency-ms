use axum::http::HeaderMap;

use crate::handlers::responses::MessageResponse;

pub fn extract_authn_token(headers: HeaderMap) -> Result<String, MessageResponse> {
    let error = Err(MessageResponse::token_auth_failure());

    let auth_header = match headers.get("Authorization") {
        Some(value) => value.to_str(),
        None => return error,
    };

    let token = match auth_header {
        Ok(s) => s.strip_prefix("Bearer "),
        Err(_) => return error,
    };

    match token {
        Some(token) => Ok(String::from(token)),
        None => return error,
    }
}
