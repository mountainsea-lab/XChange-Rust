use crate::define_exchange_error;
use crate::error::ExchangeErrorDetail;
use thiserror::Error;

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

define_exchange_error!(
    ExchangeSecurityError,
    "Wrong credentials or insufficient privileges"
);

define_exchange_error!(ExchangeUnavailableError, "Service unavailable");
