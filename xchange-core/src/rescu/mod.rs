use std::time::SystemTimeError;
use std::{fmt, io};

pub mod params_digest;
#[derive(Debug)]
pub enum HttpError {
    InvalidProxy(String),
    Io(io::Error),
    InvalidKey(String),
    InvalidTimestamp(SystemTimeError),
    UnsupportedMethod(String),
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpError::InvalidProxy(p) => write!(f, "invalid proxy: {}", p),
            HttpError::Io(e) => write!(f, "I/O error: {}", e),
            HttpError::InvalidKey(d) => write!(f, "Params Digest Invalid: {}", d),
            HttpError::InvalidTimestamp(t) => write!(f, "Timestamp Invalid: {}", t),
            HttpError::UnsupportedMethod(u) => write!(f, "Unsupported Method: {}", u),
        }
    }
}

impl std::error::Error for HttpError {}

impl From<io::Error> for HttpError {
    fn from(err: io::Error) -> Self {
        HttpError::Io(err)
    }
}
