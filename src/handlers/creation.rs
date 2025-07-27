use axum::{
    Json,
    extract::State,
    http::HeaderMap,
    response::{IntoResponse, Response},
};
use reqwest::StatusCode;
use sqlx::PgPool;

use crate::{
    db::queries::create_wallet,
    handlers::{helper::extract_authn_token, responses::MessageResponse},
    requests::player::authenticate_player_token,
};

/// Handles the request for creating a new bit wallet in the database. This request should only ever
/// be called by the player microservice immediately following the creation of a new player account.
///
/// # Arguments
/// * pool - The postgres connection pool.
/// * headers - The HTTP response headers. Should include an Authorization header with a valid
///   player authentication token.
///
/// # Returns
/// An HTTP response to be send back to the client.
pub async fn handle_wallet_creation(State(pool): State<PgPool>, headers: HeaderMap) -> Response {
    let auth_failure_response = (
        StatusCode::UNAUTHORIZED,
        Json(MessageResponse::token_auth_failure()),
    )
        .into_response();

    let auth_token = match extract_authn_token(headers) {
        Ok(t) => t,
        Err(_) => return auth_failure_response,
    };

    let id = match authenticate_player_token(auth_token).await {
        Ok(v) => v,
        Err(response) => return (StatusCode::UNAUTHORIZED, Json(response)).into_response(),
    };

    match create_wallet(&pool, id).await {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(MessageResponse::new("Wallet could not be created.")),
        )
            .into_response(),
    }
}
