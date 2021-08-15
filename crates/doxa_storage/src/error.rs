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
    BAD_REQUEST,
    "COULD_NOT_READ_FILE",
    "There was an error receiving the upload"
);

#[derive(Debug, Display, Error)]
pub struct FileTooLarge;

impl_respondable_error!(
    FileTooLarge,
    BAD_REQUEST,
    "FILE_TOO_LARGE",
    "The upload exceeds the maximum size for this user"
);

#[derive(Debug, Display, Error)]
pub struct FileMissing;

impl_respondable_error!(
    FileMissing,
    BAD_REQUEST,
    "FILE_MISSING",
    "The upload did not contain a file"
);

#[derive(Debug, Display, Error, RespondableError, From)]
pub enum AgentUploadError {
    #[from(forward)]
    IOError(CouldNotWriteFile),
    #[from]
    MultipartError(UploadMultipartError),
    #[from]
    FileTooLarge(FileTooLarge),
}
