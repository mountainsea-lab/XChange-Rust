use std::time::SystemTimeError;
use std::{fmt, io};

pub mod params_digest;
pub mod resilient_http_client;

#[derive(Debug)]
pub enum HttpError {
    Reqwest(reqwest::Error),
    InvalidProxy(String),
    Io(io::Error),
    InvalidKey(String),
    InvalidTimestamp(SystemTimeError),
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpError::Reqwest(e) => write!(f, "reqwest error: {}", e),
            HttpError::InvalidProxy(p) => write!(f, "invalid proxy: {}", p),
            HttpError::Io(e) => write!(f, "I/O error: {}", e),
            HttpError::InvalidKey(d) => write!(f, "Params Digest Invalid: {}", d),
            HttpError::InvalidTimestamp(t) => write!(f, "Timestamp Invalid: {}", t),
        }
    }
}

impl std::error::Error for HttpError {}

impl From<reqwest::Error> for HttpError {
    fn from(err: reqwest::Error) -> Self {
        HttpError::Reqwest(err)
    }
}

impl From<io::Error> for HttpError {
    fn from(err: io::Error) -> Self {
        HttpError::Io(err)
    }
}
