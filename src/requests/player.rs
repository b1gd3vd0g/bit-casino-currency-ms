use reqwest::{Client, StatusCode, header::HeaderMap};
use uuid::Uuid;

use crate::{handlers::responses::MessageResponse, requests::structs::PlayerId};

/// Make a request to the player microservice attempting to validate a player authentication token.
///
/// **Request made:** `GET <player-ms>/authn`
///
/// # Arguments
/// * token - The player's authentication JWT.
///
/// # Returns
/// * The player_id of the authenticated player on success; otherwise, a MessageResponse object
///   explaining what went wrong.
pub async fn authenticate_player_token(token: String) -> Result<Uuid, MessageResponse> {
    let client = Client::new();

    let mut hm = HeaderMap::new();
    let hv = format!("Bearer {}", token);
    hm.insert("Authorization", hv.parse().unwrap());

    let response = client
        .get("http://player-ms:3000/authn")
        .headers(hm)
        .send()
        .await;

    let response = match response {
        Ok(r) => r,
        Err(_) => return Err(MessageResponse::new("Token authentication request failed")),
    };

    match response.status() {
        StatusCode::OK => {
            let player: PlayerId = response.json().await.unwrap();
            Ok(player.id)
        }
        _ => return Err(MessageResponse::token_auth_failure()),
    }
}
