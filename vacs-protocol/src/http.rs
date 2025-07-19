use serde::{Deserialize, Serialize};

pub mod auth;
pub mod ws;

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub message: String,
}
