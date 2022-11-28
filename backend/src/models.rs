use std::time::{SystemTime, UNIX_EPOCH, Duration};

use blake3::hash;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct User {
    username: String,
    pass_hash: Vec<u8>,
    admin: bool,
}

#[derive(Deserialize)]
pub(crate) struct LoginData {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub(crate) struct TokenResponse {
    pub token: [u8; 32],
}

#[derive(Deserialize)]
pub(crate) struct AccessToken {
    token: String,
    granted: String,
    expiration: u64,
}

impl AccessToken {
    pub fn new(user: &str) -> Self {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
        // 4 is completely random! randomly chosen from dice roll I mean
        Self {
            token: hash(format!("{time}{}{}", &user, time % 4).as_bytes()).to_hex().to_string(),
            granted: user.to_owned(),
            expiration: time + 86400,
        }
    }
    pub fn blank() -> Self {
        Self {
            token: [0; 32],
            granted: String::new(),
            expiration: 0,
        }
    }
}

pub(crate) struct NewUser {
    token: [u8; 32],
}
