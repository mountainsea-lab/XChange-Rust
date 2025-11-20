mod exchange_security_exception;
mod macros;

use std::{error::Error, fmt};
use thiserror::Error;

/// Trait for all custom, exchange-specific errors.
/// All implementors are required to be Send + Sync + 'static.
pub trait ExchangeErrorDetail: fmt::Debug + fmt::Display + Send + Sync + 'static {
    /// Returns the underlying cause of this error, if any.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug, Error)]
pub enum ExchangeError {
    /// Generic plain message (lightweight)
    #[error("Exchange error: {0}")]
    Message(String),

    /// Wrong credentials, missing permission, IP banned, etc.
    #[error("Security error: {0}")]
    Security(String),

    /// Network issue, timeout, rate limit, etc.
    #[error("Network error: {0}")]
    Network(String),

    /// Unsupported operation, invalid argument, etc.
    #[error("Invalid request: {0}")]
    Invalid(String),

    /// Any custom exchange-specific error
    #[error("Custom error: {0}")]
    Custom(Box<dyn ExchangeErrorDetail>),
}
