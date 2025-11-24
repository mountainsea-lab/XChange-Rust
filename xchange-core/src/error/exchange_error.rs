use crate::currency::currency_pair::CurrencyPair;
use crate::define_exchange_error;
use crate::error::ExchangeErrorDetail;
use thiserror::Error;

/// Core ExchangeError for all exchange-related errors
#[derive(Debug, Error)]
pub enum ExchangeError {
    /// Simple string message (lightweight)
    #[error("Exchange error: {0}")]
    Message(String),

    /// Any custom error implementing ExchangeErrorDetail
    #[error("{0}")]
    Custom(Box<dyn ExchangeErrorDetail>),
}

// Indicates that the cause the error ware wrong credentials or insufficient privileges.
//
//  <p>We throw this exception only for exchanges where we can’t clearly distinguish this cause from
//  other error types. If an API does not provide proper error information or the modules
//  implementation is lacking then an ExchangeException will be thrown in this situation.
define_exchange_error!(
    ExchangeSecurityError,
    "Wrong credentials or insufficient privileges"
);

// An exception indicating that the server is not available, possibly due to downtime
define_exchange_error!(ExchangeUnavailableError, "Service unavailable");

// An exception indicating the request rate limit has been exceeded
define_exchange_error!(
    FrequencyLimitExceededError,
    "Too many attempts made in a given time window."
);

// An exception indicating there are not enough funds for the action requested
define_exchange_error!(FundsExceededError, "Not enough funds are available.");

// Error indicating that a request was made with an Instrument that is not supported
// on this exchange.
define_exchange_error!(
    InstrumentNotValidError,
    "Invalid currency pair for this operation",
    instrument,
    String
);

// Error indicating there was an internal server error.
define_exchange_error!(InternalServerError, "Internal Server Error.");

// Error indicating the Instrument was recognized by the exchange but its market is suspended,
// either temporarily or permanently.
//
// Note: This does not mean the entire exchange is down (see `ExchangeUnavailableError` for that).
define_exchange_error!(MarketSuspendedError, "Market is suspended");

// Error indicating there was an issue with using the provided Nonce.
define_exchange_error!(
    NonceError,
    "Something went wrong with using the provided Nonce."
);

// Error indicating that a requested deposit address was not found.
define_exchange_error!(DepositAddressNotFoundError, "Deposit Address Not Found");

// Error indicating that a deposit address could not be created.
define_exchange_error!(
    DepositAddressCreationError,
    "Deposit Address Could Not Be Created"
);

// Error indicating that a requested deposit address has multiple networks and a specific network is required.
define_exchange_error!(
    DepositAddressAmbiguousError,
    "Deposit Address Not Found",
    networks,
    Vec<String>
);

// Exception indicating that a request was made with a `CurrencyPair`
// that is not supported on this exchange.
//
// This corresponds to the Java `CurrencyPairNotValidException`.
//
// # Example
// ```
// let err = CurrencyPairNotValidError::with_field(my_pair);
// println!("{}", err);
// ```
define_exchange_error!(
    CurrencyPairNotValidError,
    "Invalid currency pair for this operation",
    currency_pair,
    CurrencyPair
);

define_exchange_error!(
    NotYetImplementedForExchangeError,
    "Feature not yet implemented for exchange."
);

define_exchange_error!(
    NotAvailableFromExchangeError,
    "Requested information or function from exchange is not available."
);

// Exception indicating that an order placed or verified was not valid.
define_exchange_error!(OrderNotValidError, "Invalid order");

// Exception indicating that an order placed or verified was not valid.
define_exchange_error!(OrderAmountUnderMinimumError, "Orders amount under minimum");

// An exception indicating that the rate limit for making requests has been exceeded.
define_exchange_error!(
    RateLimitExceededError,
    "Rate limit for making requests exceeded!"
);
