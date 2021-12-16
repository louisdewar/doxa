use std::path::PathBuf;

use reqwest::{Client, RequestBuilder, Response, StatusCode, Url};
use serde::de::DeserializeOwned;
use serde::Deserialize;

use crate::{
    config::UserProfile,
    error::{BaseURLFormatError, DoxaError, NoDefaultUserProfile, PlainError, RequestError},
};

pub struct Settings {
    pub user_profile: Result<UserProfile, NoDefaultUserProfile>,
    pub base_url: Url,
    pub config_dir: PathBuf,
    pub client: Client,
    pub verbose: bool,
}

impl Settings {
    pub fn new(
        user_profile: Result<UserProfile, NoDefaultUserProfile>,
        base_url: Url,
        config_dir: PathBuf,
        verbose: bool,
    ) -> Settings {
        Settings {
            user_profile,
            base_url,
            client: Client::new(),
            config_dir,
            verbose,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
struct DoxaErrorRaw {
    error_code: String,
    error: Option<String>,
}

impl DoxaErrorRaw {
    fn into_doxa_error(self, status_code: StatusCode) -> DoxaError {
        DoxaError {
            error_code: self.error_code,
            message: self.error,
            status_code,
        }
    }
}

pub fn parse_base_url(base_url: &str) -> Result<Url, BaseURLFormatError> {
    let url = Url::parse(base_url)?.join("api/")?;
    Ok(url)
}

fn to_url(settings: &Settings, endpoint: &str) -> Url {
    settings.base_url.join(endpoint).unwrap()
}

fn maybe_add_auth(
    settings: &Settings,
    builder: RequestBuilder,
    never_auth: bool,
) -> RequestBuilder {
    if never_auth {
        builder
    } else if let Ok(user) = &settings.user_profile {
        builder.bearer_auth(user.auth_token.clone())
    } else {
        builder
    }
}

pub fn post(settings: &Settings, endpoint: &str, never_auth: bool) -> RequestBuilder {
    maybe_add_auth(
        settings,
        settings.client.post(to_url(settings, endpoint)),
        never_auth,
    )
}

pub fn get(settings: &Settings, endpoint: &str, never_auth: bool) -> RequestBuilder {
    maybe_add_auth(
        settings,
        settings.client.get(to_url(settings, endpoint)),
        never_auth,
    )
}

/// Sends the request without reading the response or checking the status code
async fn send_request_raw(builder: RequestBuilder) -> Result<Response, RequestError> {
    builder.send().await.map_err(|e| e.into())
}

/// Same as `send_request_and_parse` except if the status code is `OK` it will simply return the
/// response without parsing.
pub async fn send_request(builder: RequestBuilder) -> Result<Response, RequestError> {
    let response = send_request_raw(builder).await?;

    let status = response.status();

    if status.is_success() {
        Ok(response)
    } else {
        let bytes = response.bytes().await?;
        match serde_json::from_slice::<DoxaErrorRaw>(&bytes) {
            Err(_) => Err(PlainError {
                status_code: status,
                error_message: String::from_utf8_lossy(&bytes).to_string(),
            }
            .into()),
            Ok(v) => Err(v.into_doxa_error(status).into()),
        }
    }
}

/// Sends a request and serializes the response to `T` if the status code is `OK` otherwise it
/// treats the response as an error and handles the various cases.
pub async fn send_request_and_parse<T: DeserializeOwned>(
    builder: RequestBuilder,
) -> Result<T, RequestError> {
    let response = send_request(builder).await?;
    let bytes = response.bytes().await?;
    serde_json::from_slice(&bytes).map_err(Into::into)
}
