use doxa_core::{impl_respondable_error, RespondableError};

// #[derive(RespondableError)]
// pub enum UploadError {
//     AuthenticationError,
// }

use actix_multipart::MultipartError;
use derive_more::{Display, Error, From};

#[derive(Debug, Display, Error, From)]
pub struct CouldNotWriteFile {
    reason: std::io::Error,
}

impl_respondable_error!(
    CouldNotWriteFile,
    INTERNAL_SERVER_ERROR,
    "COULD_NOT_WRITE_FILE",
    "There was an error writing the artifact"
);

#[derive(Debug, Display, Error, From)]
pub struct UploadMultipartError {
    reason: MultipartError,
}

impl_respondable_error!(
    UploadMultipartError,
    INTERNAL_SERVER_ERROR,
    "COULD_NOT_REEAD_FILE",
    "There was an error receiving the artifact"
);
