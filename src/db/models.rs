use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

/// The Transaction model represents a row from the `transactions` table.
/// These are successful
#[derive(FromRow)]
pub struct Transaction {
    /// The unique id of the transaction.
    pub id: Uuid,
    /// The id of the involved player.
    pub player_id: Uuid,
    /// The amount (positive or negative) of the transaction.
    pub amount: BigDecimal,
    /// The timestamp of the transaction request.
    pub timestamp: DateTime<Utc>,
    /// The reason for the transaction.
    pub reason: String,
    /// Was this transaction successful?
    pub success: bool,
    /// The error message, in case the transaction failed.
    pub error_message: Option<String>,
}

/// The BitWallet model represents a row from the `bit_wallets` table.
#[derive(FromRow)]
pub struct BitWallet {
    /// The unique id of the player (and their bit wallet).
    pub player_id: Uuid,
    /// The current balance of the bit wallet.
    pub balance: BigDecimal,
}
