use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: Role,
    pub iss: usize,
    pub exp: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    Superuser,
}

pub fn generate(key: &EncodingKey, role: Role, sub: String) -> Result<String, jsonwebtoken::errors::Error> {
    let now = unix_timestamp() as usize;

    let claims = Claims {
        sub,
        role,
        iss: now,
        exp: now + 300
    };

    encode(
        &Header::default(),
        &claims,
        key
    )
}

fn unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}
