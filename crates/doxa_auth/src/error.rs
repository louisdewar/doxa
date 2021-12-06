use doxa_core::{
    impl_respondable_error,
    redis::{RedisError, RedisPoolError},
    RespondableError,
};

use derive_more::{Display, Error, From};
use doxa_db::DieselError;

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

#[derive(Debug, Display, Error, From)]
pub struct InvalidToken {
    source: jwt::Error,
}

impl_respondable_error!(
    InvalidToken,
    UNAUTHORIZED,
    "INVALID_TOKEN",
    "The provided token was not valid"
);

#[derive(Debug, Display, Error)]
pub struct ExpiredToken;

impl_respondable_error!(
    ExpiredToken,
    UNAUTHORIZED,
    "EXPIRED_TOKEN",
    "The provided token has expired"
);

#[derive(Debug, Display, Error)]
pub struct IncorrectTokenGeneration;

impl_respondable_error!(
    IncorrectTokenGeneration,
    UNAUTHORIZED,
    "INCORRECT_TOKEN_GENERATION",
    "The login token is outdated so you need to login again"
);

#[derive(Debug, Display, Error, RespondableError, From)]
pub enum TokenError {
    #[from]
    Expired(ExpiredToken),
    #[from(forward)]
    Invalid(InvalidToken),
}

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

#[derive(Debug, Display, Error)]
pub struct InviteNotFound;

impl_respondable_error!(
    InviteNotFound,
    BAD_REQUEST,
    "INVITE_NOT_FOUND",
    "This invite has been used already, never existed or expired"
);

#[derive(Debug, Display, Error)]
pub struct InviteExpired;

impl_respondable_error!(
    InviteExpired,
    BAD_REQUEST,
    "INVITE_EXPIRED",
    "This invite has expired"
);

#[derive(Debug, Display, Error)]
pub struct RegistrationInviteMismatch;

impl_respondable_error!(
    RegistrationInviteMismatch,
    BAD_REQUEST,
    "REGISTRATION_INVITE_MISMATCH",
    "The fields in the registration does not match those specified as part of the invite"
);

// TODO: in future this will be an enum as `is_allowed` will do some more advanced checking and
// return this error.
#[derive(Debug, Display, Error)]
pub struct InvalidPassword;

impl_respondable_error!(
    InvalidPassword,
    BAD_REQUEST,
    "INVALID_PASSWORD",
    "Your password failed to meet the length requirements (not too long and not too short)"
);

#[derive(Debug, Display, Error, RespondableError, From)]
pub enum CreateUserError {
    #[from]
    Diesel(DieselError),
    #[from]
    AlreadyExists(UserAlreadyExists),
    #[from]
    InvalidPassword(InvalidPassword),
}

#[derive(Debug, Display, Error, RespondableError, From)]
pub enum AcceptInviteError {
    #[from]
    Mismatch(RegistrationInviteMismatch),
    #[from]
    InviteExpired(InviteExpired),
    #[from]
    InviteNotFound(InviteNotFound),
    #[from]
    Diesel(DieselError),
    #[from]
    AlreadyExists(UserAlreadyExists),
    #[from]
    InvalidPassword(InvalidPassword),
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
pub struct IncorrectPassword;

impl_respondable_error!(
    IncorrectPassword,
    BAD_REQUEST,
    "INCORRECT_PASSWORD",
    "The password does not match the username"
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

        impl_respondable_error!($name, TOO_MANY_REQUESTS, "NO_PERMITS", $error_message);
    };
}

create_rate_limit_error!(TooManyLoginAttempts, "There have been too many login attempts to your account please wait a while and then try again");
