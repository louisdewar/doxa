use std::fmt;

use actix_web::{dev::HttpResponseBuilder, error::BlockingError};

pub use actix_web::http::StatusCode;
pub use actix_web::HttpResponse;

use serde::Serialize;

pub type EndpointResult = Result<HttpResponse, HttpResponse>;

#[derive(Serialize)]
struct ErrorResponse {
    error_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

pub trait RespondableError: fmt::Debug {
    fn error_code(&self) -> String;
    fn error_message(&self) -> Option<String>;

    fn status_code(&self) -> StatusCode;

    fn into_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .json(ErrorResponse {
                error_code: self.error_code(),
                error: self.error_message(),
            })
            .into()
    }

    fn display_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(message) = self.error_message() {
            write!(f, "({}) {}", self.error_code(), message)
        } else {
            write!(f, "{}", self.error_code())
        }
    }
}

#[macro_export]
macro_rules! impl_respondable_error {
    ($struct:ident, $status_code:ident, $error_code:expr, option: $error_message:expr) => {
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

    ($struct:ident, $status_code:ident, $error_code:expr, $error_message:expr) => {
        impl_respondable_error!($struct, $status_code, $error_code, option: Some($error_message.into()));
    };

    ($struct:ident, $status_code:ident, $error_code:expr) => {
        impl_respondable_error!($struct, $status_code, $error_code, option: None);
    };
}

impl<I: RespondableError> RespondableError for BlockingError<I> {
    fn error_code(&self) -> String {
        match self {
            BlockingError::Canceled => "INTERNAL_SERVER_ERROR".into(),
            BlockingError::Error(e) => e.error_code(),
        }
    }

    fn error_message(&self) -> Option<String> {
        match self {
            BlockingError::Canceled => None,
            BlockingError::Error(e) => e.error_message(),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            BlockingError::Canceled => StatusCode::INTERNAL_SERVER_ERROR,
            BlockingError::Error(e) => e.status_code(),
        }
    }
}

impl RespondableError for diesel::result::Error {
    fn error_code(&self) -> String {
        "INTERNAL_SERVER_ERROR".to_string()
    }

    fn error_message(&self) -> Option<String> {
        None
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

impl RespondableError for diesel::r2d2::PoolError {
    fn error_code(&self) -> String {
        "INTERNAL_SERVER_ERROR".to_string()
    }

    fn error_message(&self) -> Option<String> {
        None
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[macro_export]
macro_rules! handle_doxa_error {
    ($e:expr) => {
        match $e {
            Ok(val) => val,
            Err(e) => {
                // TODO: logging
                println!("[{}:{}] error {}", file!(), line!(), e);

                return Err($crate::error::RespondableError::into_response(&e));
            }
        }
    };
}
