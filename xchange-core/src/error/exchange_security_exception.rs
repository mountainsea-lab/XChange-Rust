use std::{error::Error, fmt};

use crate::error::{ExchangeError, ExchangeErrorDetail};

/// Indicates that the cause of the error were wrong credentials or insufficient privileges.
///
/// This error is returned only when the exchange API does not provide detailed enough
/// information to distinguish this case from other error types. When the API is capable of
/// producing a more specific classification, `ExchangeError::Security(String)` should be used.
///
/// Mirrors Java's `ExchangeSecurityException`.
#[derive(Debug)]
pub struct ExchangeSecurityError {
    /// Human-readable error description.
    pub message: String,

    /// Optional underlying cause (equivalent to Java Throwable cause).
    pub source: Option<Box<dyn Error  + Send + Sync + 'static>>,

}

impl ExchangeSecurityError {
    /// Default message when no additional info is provided.
    pub const DEFAULT_MESSAGE: &'static str = "Wrong credentials or insufficient privileges";

    /// Equivalent to Java: new ExchangeSecurityException()
    pub fn new() -> Self {
        Self {
            message: Self::DEFAULT_MESSAGE.to_string(),
            source: None,
        }
    }

    /// Equivalent to Java: new ExchangeSecurityException(String message)
    pub fn with_message(msg: impl Into<String>) -> Self {
        Self {
            message: msg.into(),
            source: None,
        }
    }

    /// Equivalent to Java: new ExchangeSecurityException(Throwable cause)
    pub fn with_source(cause: impl Error + Send + Sync + 'static) -> Self {
        Self {
            message: Self::DEFAULT_MESSAGE.to_string(),
            source: Some(Box::new(cause)),
        }
    }

    /// Equivalent to Java: new ExchangeSecurityException(String message, Throwable cause)
    pub fn with_message_and_source(
        msg: impl Into<String>,
        cause: impl Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            message: msg.into(),
            source: Some(Box::new(cause)),
        }
    }
}

impl fmt::Display for ExchangeSecurityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Security error: {}", self.message)
    }
}

impl Error for ExchangeSecurityError {
    fn source(&self) -> Option<&(dyn Error  + Send + Sync + 'static)> {
        self.source.as_deref()
    }

}

impl ExchangeErrorDetail for ExchangeSecurityError {}

impl From<ExchangeSecurityError> for ExchangeError {
    fn from(err: ExchangeSecurityError) -> Self {
        ExchangeError::Custom(Box::new(err))
    }
}
