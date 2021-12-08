use doxa_auth::{create_rate_limit_error, error::CompetitionNotFound};
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
    "There was an error writing the file"
);

#[derive(Debug, Display, Error, From)]
pub struct AgentGone;

impl_respondable_error!(
    AgentGone,
    GONE,
    "AGENT_GONE",
    "The ID matches an agent that was uploaded but it does not meet the criteria or has been deleted."
);

#[derive(Debug, Display, Error, From)]
pub struct CouldNotReadFile {
    reason: std::io::Error,
}

impl_respondable_error!(
    CouldNotReadFile,
    INTERNAL_SERVER_ERROR,
    "COULD_NOT_READ_FILE",
    "There was an error reading the file"
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
#[display(fmt = "Invalid extension (ext = `{}`)", extension)]
pub struct InvalidExtension {
    pub extension: String,
}
impl_respondable_error!(
    InvalidExtension,
    BAD_REQUEST,
    "INVALID_FILE_EXTENSION",
    "The provided file extension is not supported"
);

#[derive(Debug, Display, Error)]
pub struct FileMissing;

impl_respondable_error!(
    FileMissing,
    BAD_REQUEST,
    "FILE_MISSING",
    "The upload did not contain a file"
);

#[derive(Debug, Display, Error)]
pub struct ExtensionMissing;

impl_respondable_error!(
    ExtensionMissing,
    BAD_REQUEST,
    "FILE_EXTENSION_MISSING",
    "Could not find the file name and/or extension"
);

#[derive(Debug, Display, Error)]
pub struct AgentNotFound;

impl_respondable_error!(
    AgentNotFound,
    NOT_FOUND,
    "AGENT_NOT_FOUND",
    "There isn't an agent with that ID within this competition"
);

#[derive(Debug, Display, Error, RespondableError, From)]
pub enum AgentUploadError {
    #[from(forward)]
    IOError(CouldNotWriteFile),
    #[from]
    MultipartError(UploadMultipartError),
    #[from]
    FileTooLarge(FileTooLarge),
    #[from]
    InvalidExtension(InvalidExtension),
    #[from]
    ExtensionMissing(ExtensionMissing),
}

#[derive(Debug, Display, Error, RespondableError, From)]
pub enum AgentDownloadError {
    #[from(forward)]
    IOError(CouldNotReadFile),
    #[from]
    CompetitionNotFound(CompetitionNotFound),
    #[from]
    AgentNotFound(AgentNotFound),
}

create_rate_limit_error!(TooManyUploadAttempts, "There have been too many agent upload attempts by your account to this competition, please wait and try again later");
