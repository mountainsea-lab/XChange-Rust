use crate::dto::BinanceException;
use xchange_core::error::exchange_error::{
    CurrencyPairNotValidError, ExchangeError, ExchangeSecurityError, ExchangeUnavailableError,
    FundsExceededError, OrderAmountUnderMinimumError, OrderNotValidError, RateLimitExceededError,
};

pub struct BinanceErrorAdapter;

impl BinanceErrorAdapter {
    pub fn adapt(e: BinanceException) -> ExchangeError {
        let message = if e.msg.is_empty() {
            "Operation failed without any error message".to_string()
        } else {
            e.msg.clone()
        };

        match e.code {
            -1002 => ExchangeSecurityError::with_message(message).into(),
            -1003 => RateLimitExceededError::with_message(message).into(),

            -1010 | -2010 | -2011 => {
                if e.msg.contains("insufficient balance") {
                    FundsExceededError::with_message(message).into()
                } else {
                    ExchangeError::Message(message)
                }
            }

            -1013 => Self::create_order_not_valid_exception(message, e),

            -1016 => ExchangeUnavailableError::with_message(message).into(),
            -1021 => ExchangeError::Message("Time out".to_string()),
            -1121 => CurrencyPairNotValidError::with_message(message).into(),
            -1122 => ExchangeSecurityError::with_message(message).into(),

            _ => ExchangeError::Message(message),
        }
    }

    fn create_order_not_valid_exception(message: String, e: BinanceException) -> ExchangeError {
        if e.msg.contains("MIN_NOTIONAL") {
            OrderAmountUnderMinimumError::with_message(message).into()
        } else {
            OrderNotValidError::with_message(message).into()
        }
    }
}
