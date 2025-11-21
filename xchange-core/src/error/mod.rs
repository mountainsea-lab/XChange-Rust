pub mod exchange_error;
pub mod macros;

use std::{error::Error, fmt};

/// Trait for all custom, exchange-specific errors.
/// All implementors are required to be Send + Sync + 'static.
pub trait ExchangeErrorDetail: fmt::Debug + fmt::Display + Send + Sync + 'static {
    /// Returns the underlying cause of this error, if any.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
