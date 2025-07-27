use axum::{
    Json,
    extract::State,
    http::HeaderMap,
    response::{IntoResponse, Response},
};
use bigdecimal::BigDecimal;
use reqwest::StatusCode;
use serde::Serialize;
use sqlx::PgPool;

use crate::{
    db::queries::get_wallet,
    handlers::{helper::extract_authn_token, responses::MessageResponse},
    requests::player::authenticate_player_token,
};

#[derive(Serialize)]
pub struct Balance {
    pub balance: BigDecimal,
}

pub async fn handle_find_player_wallet(State(pool): State<PgPool>, headers: HeaderMap) -> Response {
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
        Err(e) => return (StatusCode::UNAUTHORIZED, Json(e)).into_response(),
    };

    let wallet = match get_wallet(&pool, id).await {
        Ok(w) => w,
        Err(e) => match e {
            sqlx::Error::RowNotFound => return StatusCode::NOT_FOUND.into_response(),
            _ => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(MessageResponse::new("Get wallet query failed.")),
                )
                    .into_response();
            }
        },
    };

    (
        StatusCode::OK,
        Json(Balance {
            balance: wallet.balance,
        }),
    )
        .into_response()
}
