use serde::Deserialize;
use uuid::Uuid;

/// This can be used to pull the `id` field from a JSON response body (for example, `GET
/// <player-ms>/authn`).
#[derive(Deserialize)]
pub struct PlayerId {
    pub id: Uuid,
}
