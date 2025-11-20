// use std::{error::Error, fmt, sync::Arc};
//
// use crate::error::{ExchangeError, ExchangeErrorDetail};
//
// /// Indicates wrong credentials or insufficient privileges.
// ///
// /// Mirrors Java's `ExchangeSecurityException`.
// #[derive(Debug, Clone)]
// pub struct ExchangeSecurityError {
//     /// Human-readable error description.
//     pub message: String,
//
//     /// Optional underlying cause (chainable error).
//     source: Option<Arc<dyn Error + Send + Sync + 'static>>,
// }
//
// impl ExchangeSecurityError {
//     /// Default message
//     pub const DEFAULT_MESSAGE: &'static str = "Wrong credentials or insufficient privileges";
//
//     /// Equivalent to Java: new ExchangeSecurityException()
//     pub fn new() -> Self {
//         Self {
//             message: Self::DEFAULT_MESSAGE.to_string(),
//             source: None,
//         }
//     }
//
//     /// Equivalent to Java: new ExchangeSecurityException(String message)
//     pub fn with_message(msg: impl Into<String>) -> Self {
//         Self {
//             message: msg.into(),
//             source: None,
//         }
//     }
//
//     /// Equivalent to Java: new ExchangeSecurityException(Throwable cause)
//     pub fn with_source<E>(cause: E) -> Self
//     where
//         E: Error + Send + Sync + 'static,
//     {
//         Self {
//             message: Self::DEFAULT_MESSAGE.to_string(),
//             source: Some(Arc::new(cause)),
//         }
//     }
//
//     /// Equivalent to Java: new ExchangeSecurityException(String message, Throwable cause)
//     pub fn with_message_and_source<E>(msg: impl Into<String>, cause: E) -> Self
//     where
//         E: Error + Send + Sync + 'static,
//     {
//         Self {
//             message: msg.into(),
//             source: Some(Arc::new(cause)),
//         }
//     }
// }
//
// impl fmt::Display for ExchangeSecurityError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Security error: {}", self.message)
//     }
// }
//
// // Implement standard Error trait
// impl Error for ExchangeSecurityError {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         self.source
//             .as_ref()
//             .map(|e| e.as_ref() as &(dyn Error + 'static))
//     }
// }
//
// // Implement custom ExchangeErrorDetail trait
// impl ExchangeErrorDetail for ExchangeSecurityError {}
//
// // Convert to generic ExchangeError
// impl From<ExchangeSecurityError> for ExchangeError {
//     fn from(err: ExchangeSecurityError) -> Self {
//         ExchangeError::Custom(Box::new(err))
//     }
// }
