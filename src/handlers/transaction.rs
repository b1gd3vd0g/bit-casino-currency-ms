use axum::{
    Json,
    extract::State,
    http::HeaderMap,
    response::{IntoResponse, Response},
};
use bigdecimal::{BigDecimal, ToPrimitive};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    db::queries::{commit_transaction, update_wallet_balance},
    handlers::{helper::extract_authn_token, responses::MessageResponse},
    requests::player::authenticate_player_token,
};

#[derive(Deserialize)]
pub struct RequestBody {
    pub amount: BigDecimal,
    pub reason: String,
}

#[derive(Serialize)]
pub struct BalanceResponse {
    pub balance: u128,
}

pub async fn handle_transaction(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Json(body): Json<RequestBody>,
) -> Response {
    let auth_failure_response = (
        StatusCode::UNAUTHORIZED, //401
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

    let amount = body.amount;

    let updated_wallet = update_wallet_balance(&pool, id, amount.clone()).await;

    let log_transaction = match updated_wallet {
        Ok(_) => commit_transaction(&pool, id, amount, body.reason, None).await,
        Err(_) => {
            let message = String::from("Failed to update the balance.");
            commit_transaction(&pool, id, amount, body.reason, Some(message)).await
        }
    };

    match (updated_wallet, log_transaction) {
        (Ok(w), Ok(_)) => (
            StatusCode::OK,
            Json(BalanceResponse {
                balance: w.balance.to_u128().unwrap(),
            }),
        )
            .into_response(), //200
        (Err(_), Ok(_)) => (
            StatusCode::CONFLICT, // 409
            Json(MessageResponse::new(
                "Could not complete the transaction. Likely a balance failure.",
            )),
        )
            .into_response(),
        (Ok(_), Err(_)) => (
            StatusCode::INTERNAL_SERVER_ERROR, //500
            Json(MessageResponse::new(
                "Transaction completed but not logged! Unlikely and bad!",
            )),
        )
            .into_response(),
        (Err(_), Err(_)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(MessageResponse::new(
                "Transaction not completed or logged. Likely a database issue.",
            )),
        )
            .into_response(),
    }
}
