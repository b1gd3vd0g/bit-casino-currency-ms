use serde::Serialize;

#[derive(Serialize)]
pub struct MessageResponse {
    pub message: String,
}

impl MessageResponse {
    pub fn new(message: &str) -> Self {
        MessageResponse {
            message: String::from(message),
        }
    }

    pub fn token_auth_failure() -> Self {
        MessageResponse::new("Token authentication failed.")
    }
}
