use std::time::Duration;

use doxa_core::{
    chrono::{self, DateTime, Utc},
    redis::{redis::AsyncCommands, RedisPool},
};
use serde::{Deserialize, Serialize};

use crate::error::{DelegatedAuthError, DelegatedAuthExpired, InvalidDelegatedAuthSecret};

const AUTH_SECRET_LEN: usize = 20;
const VERIFICATION_CODE_LEN: usize = 24;
const AUTH_EXPIRATION: Duration = Duration::from_secs(60 * 60 * 3);
const POST_AUTH_EXPIRATION: Duration = Duration::from_secs(60 * 30);

#[derive(Serialize)]
pub struct DelegatedAuthCreation {
    verification_code: String,
    verify_url: String,
    auth_secret: String,
    expires: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
struct DelegatedAuthRecord {
    auth_secret: String,
    // Starts at None and then gets set to the user ID when they authenticate.
    authenticated_user_id: Option<i32>,
}

fn generate_secret(len: usize) -> String {
    use rand::Rng;

    let generation: Vec<u8> = rand::thread_rng()
        .sample_iter(rand::distributions::Standard)
        .take(len)
        .collect();

    base64::encode_config(generation, base64::URL_SAFE_NO_PAD)
}

pub struct DelegatedAuthManager {
    redis: RedisPool,
}

fn delegated_auth_key(verification_code: &str) -> String {
    format!("DELEGATED_AUTH-{}", verification_code)
}

impl DelegatedAuthManager {
    pub fn new(redis: RedisPool) -> Self {
        DelegatedAuthManager { redis }
    }

    pub async fn create(
        &self,
        mut verify_base_url: url::Url,
    ) -> Result<DelegatedAuthCreation, DelegatedAuthError> {
        let auth_secret = generate_secret(AUTH_SECRET_LEN);
        let verification_code = generate_secret(VERIFICATION_CODE_LEN);

        let key = delegated_auth_key(&verification_code);

        let mut conn = self.redis.get().await?;
        conn.set(
            &key,
            serde_json::to_string(&DelegatedAuthRecord {
                auth_secret: auth_secret.clone(),
                authenticated_user_id: None,
            })
            .unwrap(),
        )
        .await?;
        let expiration = Utc::now() + chrono::Duration::from_std(AUTH_EXPIRATION).unwrap();
        conn.expire(&key, AUTH_EXPIRATION.as_secs() as usize)
            .await?;

        verify_base_url
            .query_pairs_mut()
            .append_pair("verification_code", &verification_code);

        Ok(DelegatedAuthCreation {
            verification_code,
            verify_url: verify_base_url.to_string(),
            auth_secret,
            expires: expiration,
        })
    }

    /// Checks if the verification_code has been authenticated and that the auth_secret matches.
    /// If this is true then it will return `Ok(Some(authenticated_user_id))`.
    /// If the auth_secret matches but it is not authenticated yet then `Ok(None)` is returned.
    pub async fn check_authenticated(
        &self,
        verification_code: &str,
        auth_secret: &str,
    ) -> Result<Option<i32>, DelegatedAuthError> {
        let key = delegated_auth_key(verification_code);

        let mut conn = self.redis.get().await?;

        let record: Option<String> = conn.get(&key).await?;
        let record: DelegatedAuthRecord =
            serde_json::from_str(&record.ok_or(DelegatedAuthExpired)?).unwrap();

        if record.auth_secret != auth_secret {
            return Err(InvalidDelegatedAuthSecret.into());
        }

        if let Some(user_id) = record.authenticated_user_id {
            conn.del(&key).await?;

            Ok(Some(user_id))
        } else {
            Ok(None)
        }
    }

    /// Sets the user id field for the specified verification url.
    pub async fn authenticate(
        &self,
        verification_code: &str,
        user_id: i32,
    ) -> Result<(), DelegatedAuthError> {
        let key = delegated_auth_key(verification_code);

        let mut conn = self.redis.get().await?;

        let record: Option<String> = conn.get(&key).await?;
        let mut record: DelegatedAuthRecord =
            serde_json::from_str(&record.ok_or(DelegatedAuthExpired)?).unwrap();

        record.authenticated_user_id = Some(user_id);
        conn.set(&key, serde_json::to_string(&record).unwrap())
            .await?;
        conn.expire(&key, POST_AUTH_EXPIRATION.as_secs() as usize)
            .await?;

        Ok(())
    }
}
