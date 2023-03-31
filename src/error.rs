use thiserror::Error;
use crate::api;

#[derive(Error, Debug)]
pub enum Error {
    #[error("NovelAI API returned Error {}: {}", .0.status, .0.message)]
    Api(#[from] api::ErrorResponse),
    #[error("Bad Arguments: {0}")]
    BadArguments(String),
    #[error("Invalid password. Make sure it is at least 6 characters long\
             and only comprised of alphanumeric characters.")]
    BadPassword,
    #[error("Hash algorithm failed. This is a catastrophic failure and should be reported. Error: {0}")]
    CryptoError(#[from] argon2::Error),
    #[error("Error when contacting NovelAI service: {}", .0)]
    ClientError(#[from] reqwest::Error),
    #[error("An unknown error has occured. This may mean that the API is no longer compatible with this crate version")]
    UnknownError,
}

/* TODO: Remove this in favor of #[from] macro (see above)
impl From<api::ErrorResponse> for Error {
    fn from(e: api::ErrorResponse) -> Self {
        Error::Api(e)
    }
}
*/

/*
impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.status, self.message)
    }
}

// Unspoken shitty part of Rust
impl std::error::Error for ApiError {}
*/
