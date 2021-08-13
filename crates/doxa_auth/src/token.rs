use std::time::{Duration, SystemTime, UNIX_EPOCH};

use hmac::Hmac;
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use crate::error::{ExpiredToken, TokenError};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Token {
    user: i32,
    expires_at: u64,
}

impl Token {
    pub fn new_with_duration(user: i32, duration: Duration) -> Token {
        let expires_at =
            (SystemTime::now().duration_since(UNIX_EPOCH).unwrap() + duration).as_secs();

        Token { user, expires_at }
    }

    pub fn user(&self) -> i32 {
        self.user
    }

    pub fn expires_at(&self) -> u64 {
        self.expires_at
    }
}

pub fn parse_token(token_str: &str, key: &Hmac<Sha256>) -> Result<Token, TokenError> {
    let token: Token = token_str.verify_with_key(key)?;

    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    if token.expires_at > current_time {
        return Err(ExpiredToken.into());
    }

    Ok(token)
}

pub fn generate_jwt(token: &Token, key: &Hmac<Sha256>) -> String {
    token.sign_with_key(key).unwrap()
}
