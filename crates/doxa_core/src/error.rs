use std::fmt::{self, Display};

use actix_web::{error::BlockingError, HttpResponseBuilder};

pub use actix_web::http::StatusCode;
pub use actix_web::HttpResponse;

use serde::Serialize;

pub type EndpointResult = Result<HttpResponse, RespondableErrorWrapper>;

#[derive(Serialize)]
struct ErrorResponse {
    error_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

pub trait RespondableError: fmt::Debug + std::error::Error {
    fn error_code(&self) -> String;
    fn error_message(&self) -> Option<String>;

    fn status_code(&self) -> StatusCode;

    fn as_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code()).json(ErrorResponse {
            error_code: self.error_code(),
            error: self.error_message(),
        })
    }
}

pub struct RespondableErrorWrapper(Box<dyn RespondableError>);

impl Display for RespondableErrorWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl fmt::Debug for RespondableErrorWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl actix_web::ResponseError for RespondableErrorWrapper {
    fn status_code(&self) -> StatusCode {
        dbg!(self.0.status_code())
    }

    fn error_response(&self) -> HttpResponse {
        self.0.as_response()
    }
}

impl<T: RespondableError + 'static> From<T> for RespondableErrorWrapper {
    fn from(error: T) -> Self {
        RespondableErrorWrapper(Box::new(error))
    }
}

#[macro_export]
macro_rules! impl_respondable_error {
    ($struct:ty, $status_code:ident, $error_code:expr, option: $error_message:expr) => {
        impl $crate::RespondableError for $struct {
            fn error_code(&self) -> String {
                $error_code.into()
            }

            fn error_message(&self) -> Option<String> {
                $error_message.into()
            }

            fn status_code(&self) -> actix_web::http::StatusCode {
                actix_web::http::StatusCode::$status_code
            }
        }
    };

    ($struct:ty, $status_code:ident, $error_code:expr, $error_message:expr) => {
        impl_respondable_error!($struct, $status_code, $error_code, option: Some($error_message.into()));
    };

    ($struct:ty, $status_code:ident, $error_code:expr) => {
        impl_respondable_error!($struct, $status_code, $error_code, option: None);
    };
}

impl_respondable_error!(
    BlockingError,
    INTERNAL_SERVER_ERROR,
    "INTERNAL_SERVER_ERROR"
);

impl_respondable_error!(
    diesel::result::Error,
    INTERNAL_SERVER_ERROR,
    "INTERNAL_SERVER_ERROR"
);

impl_respondable_error!(
    diesel::r2d2::PoolError,
    INTERNAL_SERVER_ERROR,
    "INTERNAL_SERVER_ERROR"
);

impl_respondable_error!(
    deadpool_lapin::PoolError,
    INTERNAL_SERVER_ERROR,
    "INTERNAL_SERVER_ERROR"
);

impl_respondable_error!(
    tokio::task::JoinError,
    INTERNAL_SERVER_ERROR,
    "INTERNAL_SERVER_ERROR"
);

impl_respondable_error!(
    crate::redis::RedisError,
    INTERNAL_SERVER_ERROR,
    "INTERNAL_SERVER_ERROR"
);

impl_respondable_error!(
    crate::redis::RedisPoolError,
    INTERNAL_SERVER_ERROR,
    "INTERNAL_SERVER_ERROR"
);

impl_respondable_error!(lapin::Error, INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR");
