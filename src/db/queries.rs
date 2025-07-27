use bigdecimal::BigDecimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::db::models::{BitWallet, Transaction};

/// Fetches a single player's bit wallet from the database.
///
/// # Arguments
/// * pool: the postgres connection pool
/// * player_id: the id of the player whose wallet we are fetching.
///
/// # Returns
/// The player's bit wallet.
pub async fn get_wallet(pool: &PgPool, player_id: Uuid) -> Result<BitWallet, sqlx::Error> {
    sqlx::query_as!(
        BitWallet,
        r#"SELECT * FROM bit_wallets WHERE player_id = $1;"#,
        player_id
    )
    .fetch_one(pool)
    .await
}

/// Updates the balance of an existing wallet in the database. This function performs a single
/// transaction consisting of two queries: it first checks the balance, then updates the value
/// **ONLY IF** the resulting balance is non-negative.
///
/// # Notes
/// * This function does **not** record the transaction! It is important that all transactions
/// (successful or otherwise) should be recorded by the `commit_transaction` function.
///
/// # Arguments
/// * pool - the postgres connection pool
/// * player_id - the id of the player whose wallet is to be updated
/// * amount - the amount of money to update the balance by (negative values will subtract)
///
/// # Returns
/// The resulting bit wallet with the new balance.
///
/// # Errors
/// This function fails if any of the queries fail, **or** when the resulting amount in the
/// player's wallet would be negative.
pub async fn update_wallet_balance(
    pool: &PgPool,
    player_id: Uuid,
    amount: BigDecimal,
) -> Result<BitWallet, sqlx::Error> {
    let mut pg_transaction = pool.begin().await?;

    let wallet = sqlx::query_as!(
        BitWallet,
        r#"
        SELECT * FROM bit_wallets 
        WHERE player_id = $1 FOR UPDATE;
        "#,
        player_id
    )
    .fetch_one(&mut *pg_transaction)
    .await?;

    let new_amount = wallet.balance + amount;

    if new_amount < BigDecimal::from(0) {
        return Err(sqlx::Error::InvalidArgument(String::from(
            "Cannot create negative balance.",
        )));
    }

    let wallet = sqlx::query_as!(
        BitWallet,
        r#"
        UPDATE bit_wallets 
        SET balance = $1 
        WHERE player_id = $2
        RETURNING *
        "#,
        new_amount,
        player_id,
    )
    .fetch_one(&mut *pg_transaction)
    .await?;

    pg_transaction.commit().await?;

    Ok(wallet)
}

/// Attempts to create a new wallet for a new player account. It generates the wallet with the
/// initial amount of bits.
///
/// # Arguments
/// * pool - the postgres connection pool
/// * player_id - the id of the newly created player.
///
/// # Returns
/// The player's new bit wallet on success; else an sqlx error.
pub async fn create_wallet(pool: &PgPool, player_id: Uuid) -> Result<BitWallet, sqlx::Error> {
    sqlx::query_as!(
        BitWallet,
        r#"
        INSERT INTO bit_wallets (player_id)
        VALUES ($1)
        RETURNING *
        "#,
        player_id
    )
    .fetch_one(pool)
    .await
}

/// Adds a new transaction to the database. This should be done for both
/// **successful *and* unsuccessful** transactions.
///
/// # Arguments
/// * pool - the postgres connection pool
/// * player_id - the id of the player involved
/// * amount - the amount of the (attempted) transaction
/// * reason - the reason for attempting the transaction
/// * error_message - **if** the transaction failed, this is the reason
pub async fn commit_transaction(
    pool: &PgPool,
    player_id: Uuid,
    amount: BigDecimal,
    reason: String,
    error_message: Option<String>,
) -> Result<Transaction, sqlx::Error> {
    match error_message {
        Some(em) => {
            return sqlx::query_as!(
                Transaction,
                r#"
                INSERT INTO transactions (player_id, amount, reason, success, error_message)
                VALUES ($1, $2, $3, false, $4)
                RETURNING *;
                "#,
                player_id,
                amount,
                reason,
                em
            )
            .fetch_one(pool)
            .await;
        }
        None => {
            return sqlx::query_as!(
                Transaction,
                r#"
                INSERT INTO TRANSACTIONS (player_id, amount, reason, success)
                VALUES ($1, $2, $3, true)
                RETURNING *;
                "#,
                player_id,
                amount,
                reason
            )
            .fetch_one(pool)
            .await;
        }
    }
}
