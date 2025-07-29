use axum::{Router, routing::post};
use sqlx::PgPool;

use crate::handlers::{
    creation::handle_wallet_creation, find::handle_find_player_wallet,
    transaction::handle_transaction,
};

pub fn router() -> Router<PgPool> {
    Router::new()
        .route(
            "/",
            post(handle_wallet_creation).get(handle_find_player_wallet),
        )
        .route("/transaction", post(handle_transaction))
}
