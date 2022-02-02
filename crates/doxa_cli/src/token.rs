use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    error::AuthorizeError,
    request::{send_request_and_parse, to_url, Settings},
};

fn get_expiration_from_token(token: &str) -> Result<DateTime<Utc>, AuthorizeError> {
    #[derive(Deserialize)]
    struct TokenClaims {
        exp: u64,
    }

    let parts = token.split('.').collect::<Vec<_>>();

    if parts.len() < 3 {
        return Err(AuthorizeError::TooFewParts);
    }

    let claims_section = base64::decode(parts[1])?;

    let claims: TokenClaims =
        serde_json::from_slice(&claims_section).map_err(AuthorizeError::DeserializeToken)?;

    let expiration = Utc.timestamp(claims.exp as i64, 0);

    Ok(expiration)
}

#[derive(Serialize, Deserialize)]
pub struct RefreshToken {
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccessToken {
    pub access_token: String,
}

pub async fn authorize(
    refresh_token: String,
    settings: &Settings,
) -> Result<AccessToken, AuthorizeError> {
    let expiration = get_expiration_from_token(&refresh_token)?;
    if expiration < Utc::now() {
        return Err(AuthorizeError::TokenExpired);
    }

    let access_token: AccessToken = send_request_and_parse(
        settings
            .client
            .post(to_url(settings, "auth/authorize"))
            .json(&RefreshToken { refresh_token }),
    )
    .await?;

    Ok(access_token)
}
