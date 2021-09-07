use doxa_core::{impl_respondable_error, RespondableError};

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

#[derive(Debug, Display, Error, RespondableError, From)]
pub enum CreateUserError {
    #[from]
    Diesel(DieselError),
    #[from]
    AlreadyExists(UserAlreadyExists),
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
