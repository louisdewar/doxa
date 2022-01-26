use std::time::Duration;

use derive_more::{Display, Error, From};
use indicatif::HumanDuration;
use reqwest::{
    header::{HeaderMap, RETRY_AFTER},
    StatusCode,
};

#[derive(Error, Display, From, Debug)]
pub enum CliError {
    Command(CommandError),
    BaseURLFormat(BaseURLFormatError),
    #[display(fmt = "failed to load user profiles: {}", _0)]
    LoadProfileConfig(LoadProfileConfigError),
    UserNotLoggedIn(UserNotLoggedIn),
}

#[derive(Error, Display, From, Debug)]
pub enum CommandError {
    #[display(fmt = "{}", _0)]
    Request(RequestError),
    #[display(fmt = "io error: {}", _0)]
    IO(std::io::Error),
    #[display(fmt = "{}", _0)]
    Upload(UploadError),
    #[display(fmt = "{}", _0)]
    /// Only commands that require authentication will use this error
    NoUserProfile(NoDefaultUserProfile),
    /// This also exists here (and in CLI error) because some commands modify the profile
    #[display(fmt = "failed to load user profiles: {}", _0)]
    LoadProfileConfig(LoadProfileConfigError),
    #[display(fmt = "{}", _0)]
    DelegatedAuthTimeout(DelegatedAuthTimeout),
}

#[derive(Error, Display, From, Debug)]
pub enum RequestError {
    #[display(fmt = "server returned error: {}", _0)]
    Doxa(DoxaError),
    #[display(fmt = "server returned improperly formatted error: {}", _0)]
    Plain(PlainError),
    #[display(fmt = "failed to make request: {}", _0)]
    Request(reqwest::Error),
    #[display(fmt = "failed to parse response: {}", _0)]
    Json(serde_json::Error),
}

#[derive(Error, Display, From, Debug)]
#[display(fmt = "You took too long to login, this authentication has timed out")]
pub struct DelegatedAuthTimeout;

#[derive(Display, Error, Debug, Clone)]
#[display(
    fmt = "{} ({}) {}",
    error_code,
    status_code,
    "message.as_ref().map(|s| s.as_str()).unwrap_or(\"\")"
)]
pub struct DoxaError {
    pub error_code: String,
    pub status_code: StatusCode,
    pub message: Option<String>,
    pub headers: HeaderMap,
}

impl DoxaError {
    pub fn retry_after_message(&self) -> Option<String> {
        if let Some(header) = self.headers.get(RETRY_AFTER) {
            let ttl: u64 = header.to_str().unwrap().parse().unwrap();

            Some(format!(
                "Please try again after {}.",
                HumanDuration(Duration::from_secs(ttl))
            ))
        } else {
            None
        }
    }
}

#[derive(Display, Error, Debug, Clone)]
#[display(fmt = "`{}` ({})", error_message, status_code)]
pub struct PlainError {
    pub status_code: StatusCode,
    pub error_message: String,
}

#[derive(Error, Display, Debug)]
pub enum UploadError {
    #[display(fmt = "failed to read agent: {}", _0)]
    ReadAgentError(std::io::Error),
    #[display(fmt = "the path was a folder but there was no doxa.yaml file")]
    MissingExecutionConfig,
    #[display(fmt = "agents must have an extension of either .tar or .tar.gz")]
    IncorrectExtension,
}

#[derive(Display, Error, Debug, Clone, From)]
pub struct BaseURLFormatError {
    pub source: url::ParseError,
}

#[derive(Display, Error, Debug, Clone, From)]
#[display(fmt = "there is no default user profile, either log a user in or set a default profile")]
pub struct NoDefaultUserProfile;

#[derive(Display, Error, Debug, From)]
pub enum LoadProfileConfigError {
    IO(std::io::Error),
    Format(serde_json::Error),
}

#[derive(Display, Error, Debug, From)]
#[display(fmt = "user `{}` not logged in", username)]
pub struct UserNotLoggedIn {
    pub username: String,
}
