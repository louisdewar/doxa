use doxa_core::{
    impl_respondable_error,
    redis::{RedisError, RedisPoolError},
    RespondableError,
};

use derive_more::{Display, Error, From};
use doxa_db::DieselError;

#[derive(Debug, Display, Error)]
pub struct UserNotAdmin;

impl_respondable_error!(
    UserNotAdmin,
    UNAUTHORIZED,
    "USER_NOT_ADMIN",
    "You must be an admin to perform this action."
);

#[derive(Debug, Display, Error)]
pub struct InvalidAuthenticationHeader;

impl_respondable_error!(
    InvalidAuthenticationHeader,
    BAD_REQUEST,
    "INVALID_AUTHENTICATION",
    "The authentication header was invalid"
);

#[derive(Debug, Display, Error)]
pub struct MissingAuthentication;

impl_respondable_error!(
    MissingAuthentication,
    BAD_REQUEST,
    "MISSING_AUTHENTICATION",
    "The authentication was not provided"
);

#[derive(Debug, Display, Error)]
pub struct UserAlreadyExists;

impl_respondable_error!(
    UserAlreadyExists,
    BAD_REQUEST,
    "USER_ALREADY_EXISTS",
    "A user with that username already exists"
);

#[derive(Debug, Display, Error)]
pub struct RegistrationDisabled;

impl_respondable_error!(
    RegistrationDisabled,
    BAD_REQUEST,
    "REGISTRATION_DISABLED",
    "New users are not currently allowed to sign up without an invite"
);

#[derive(Debug, Display, Error, RespondableError, From)]
pub enum UpsertUserError {
    #[from]
    Diesel(DieselError),
}

#[derive(Debug, Display, Error)]
pub struct UserNotFound;

impl_respondable_error!(
    UserNotFound,
    BAD_REQUEST,
    "USER_NOT_FOUND",
    "No user with that username exists"
);

#[derive(Debug, Display, Error)]
pub struct UserNotFoundAuth;

impl_respondable_error!(
    UserNotFoundAuth,
    UNAUTHORIZED,
    "USER_NOT_FOUND",
    "This account does not appear to exist anymore"
);

#[derive(Debug, Display, Error)]
pub struct IncorrectPassword;

impl_respondable_error!(
    IncorrectPassword,
    BAD_REQUEST,
    "INCORRECT_PASSWORD",
    "The password does not match the username"
);

#[derive(Debug, Display, Error)]
pub struct NotAccessToken;

impl_respondable_error!(
    NotAccessToken,
    UNAUTHORIZED,
    "INVALID_TOKEN",
    "This token does not have permission to access this resource"
);

#[derive(Debug, Display, Error, RespondableError, From)]
pub enum LoginError {
    #[from]
    Diesel(DieselError),
    #[from]
    NotFound(UserNotFound),
    #[from]
    IncorrectPassword(IncorrectPassword),
}

#[derive(Debug, Display, Error)]
pub struct CompetitionNotFound;

impl_respondable_error!(
    CompetitionNotFound,
    NOT_FOUND,
    "COMPETITION_NOT_FOUND",
    "The competition does not exist"
);

#[derive(Debug, Display, Error)]
pub struct UserNotEnrolled;

impl_respondable_error!(
    UserNotEnrolled,
    UNAUTHORIZED,
    "NOT_ENROLLED",
    "You are not enrolled in the competition"
);

#[derive(Debug, Display, Error)]
pub struct SystemAccountsNotAllowed;

impl_respondable_error!(
    SystemAccountsNotAllowed,
    NOT_FOUND,
    "SYSTEM_ACCOUNT_NOT_ALLOWED",
    "This endpoint cannot be authenticated by a system account"
);

#[derive(Debug, Display, Error, RespondableError, From)]
pub enum CheckEnrollmentError {
    #[from]
    Diesel(DieselError),
    #[from]
    NotEnrolled(UserNotEnrolled),
    #[from]
    CompetitionNotFound(CompetitionNotFound),
}

#[derive(Debug, Display, Error)]
pub struct RateLimitReached {
    /// Time until the next permit is available.
    pub ttl: u64,
}

#[derive(Debug, Display, Error, RespondableError, From)]
pub enum GetLimiterPermitError {
    Redis(RedisError),
    RedisPool(RedisPoolError),
}

#[derive(Debug, Display, Error, RespondableError, From)]
pub enum DelegatedAuthError {
    Redis(RedisError),
    RedisPool(RedisPoolError),
    InvalidSecret(InvalidDelegatedAuthSecret),
    Expired(DelegatedAuthExpired),
}

#[derive(Debug, Display, Error, From)]
pub struct InvalidDelegatedAuthSecret;

impl_respondable_error!(InvalidDelegatedAuthSecret, BAD_REQUEST, "INVALID_SECRET");

#[derive(Debug, Display, Error, From)]
pub struct DelegatedAuthExpired;

impl_respondable_error!(
    DelegatedAuthExpired,
    BAD_REQUEST,
    "EXPIRED",
    "The authentication flow has expired or this verification code is incorrect"
);

// TODO: find a way to include the ttl in the error message (some kind of formatting with automatic
// conversion to a human readable time period), also find a way to include in the HTTP response
// header (this would allow some more interesting UI stuff probably).
//
// Maybe create some optional functions of `impl_respondable_error`, doesn't need to be generic
// i.e. get_rate_limit_ttl(), or it could be generic e.g. append_http_headers(&mut headers)
#[macro_export]
macro_rules! create_rate_limit_error {
    ($name:ident, $error_message:expr) => {
        #[derive(Debug, derive_more::Display, derive_more::Error, derive_more::From)]
        pub struct $name {
            source: $crate::error::RateLimitReached,
        }

        impl doxa_core::RespondableError for $name {
            fn error_code(&self) -> String {
                "NO_PERMITS".into()
            }

            fn error_message(&self) -> Option<String> {
                let message: Option<_> = $error_message.into();

                message.map(|inner| inner.into())
            }

            fn status_code(&self) -> actix_web::http::StatusCode {
                actix_web::http::StatusCode::TOO_MANY_REQUESTS
            }

            fn inject_headers(&self, builder: &mut doxa_core::error::HttpResponseBuilder) {
                builder.insert_header((actix_web::http::header::RETRY_AFTER, self.source.ttl));
            }
        }
    };
}

create_rate_limit_error!(TooManyLoginAttempts, "There have been too many login attempts to your account please wait a while and then try again");
