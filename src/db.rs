pub mod models;
pub mod queries;

use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

/// Connect to the database.
///
/// # Returns
/// A connection to our postgresql database.
///
/// # Errors
/// Panics if the environment is not properly set up, or if the database cannot be connected to.
pub async fn connect() -> PgPool {
    let db_url =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable is not configured!");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to the database.")
}
