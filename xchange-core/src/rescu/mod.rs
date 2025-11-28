use reqwest::StatusCode;
use std::time::SystemTimeError;
use std::{fmt, io};

pub mod params_digest;
#[derive(Debug)]
pub enum HttpError {
    Reqwest(reqwest::Error),
    InvalidProxy(String),
    Io(io::Error),
    Status(StatusCode),
    InvalidKey(String),
    InvalidTimestamp(SystemTimeError),
    UnsupportedMethod(String),
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpError::Reqwest(e) => write!(f, "reqwest error: {}", e),
            HttpError::InvalidProxy(p) => write!(f, "invalid proxy: {}", p),
            HttpError::Io(e) => write!(f, "I/O error: {}", e),
            HttpError::InvalidKey(d) => write!(f, "Params Digest Invalid: {}", d),
            HttpError::InvalidTimestamp(t) => write!(f, "Timestamp Invalid: {}", t),
            HttpError::UnsupportedMethod(u) => write!(f, "Unsupported Method: {}", u),
            HttpError::Status(s) => write!(f, "Status : {}", s),
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
