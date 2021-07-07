use doxa_core::impl_respondable_error;

use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub struct InvalidAuthentication;

impl_respondable_error!(
    InvalidAuthentication,
    BAD_REQUEST,
    "INVALID_AUTHENTICATION",
    "The authentication was invalid"
);

#[derive(Debug, Display, Error)]
pub struct MissingAuthentication;

impl_respondable_error!(
    MissingAuthentication,
    BAD_REQUEST,
    "MISSING_AUTHENTICATION",
    "The authentication was not provided"
);
