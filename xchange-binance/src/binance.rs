use crate::dto::BinanceException;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use xchange_core::currency::currency::Currency;
use xchange_core::derivative::Derivative;
use xchange_core::error::exchange_error::{
    CurrencyPairNotValidError, ExchangeError, ExchangeSecurityError, ExchangeUnavailableError,
    FundsExceededError, OrderAmountUnderMinimumError, OrderNotValidError, RateLimitExceededError,
};
use xchange_core::instrument::{Instrument, InstrumentKind};

/// --------------------------
/// BinanceErrorAdapter
/// --------------------------
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

/// --------------------------
/// BinanceAdapters
/// --------------------------
pub struct BinanceAdapters;

impl BinanceAdapters {
    /// "yyyy-MM-dd HH:mm:ss" UTC string → DateTime<Utc>
    pub fn to_utc_datetime(s: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
        let fmt = "%Y-%m-%d %H:%M:%S";
        let naive_dt = NaiveDateTime::parse_from_str(s, fmt)?;
        Ok(Utc.from_utc_datetime(&naive_dt))
    }

    /// Same as Java toSymbol(pair)
    pub fn to_symbol(inst: &InstrumentKind) -> String {
        Self::to_symbol_with_inverse(inst, false)
    }

    /// Same as Java toInverseSymbol(pair)
    pub fn to_inverse_symbol(inst: &InstrumentKind) -> String {
        Self::to_symbol_with_inverse(inst, true)
    }

    /// Same as Java isInverse(pair)
    pub fn is_inverse(inst: &InstrumentKind) -> bool {
        match inst {
            InstrumentKind::FuturesContract(fut) => fut.counter().code == "USD",
            _ => false,
        }
    }

    /// Java toSymbol(pair, isInverse)
    pub fn to_symbol_with_inverse(inst: &InstrumentKind, is_inverse: bool) -> String {
        match inst {
            InstrumentKind::CurrencyPair(cp) => {
                // Special case IOTA/BTC → IOTABTC
                if cp.base.code == "IOTA" && cp.counter.code == "BTC" {
                    return "IOTABTC".to_string();
                }

                format!("{}{}", cp.base.code, cp.counter.code)
            }

            InstrumentKind::FuturesContract(fc) => {
                let cp = fc.currency_pair();

                let mut symbol = format!("{}{}", cp.base.code, cp.counter.code);

                if is_inverse {
                    if let Some(prompt) = &fc.prompt {
                        format!("{}_{}", symbol, prompt)
                    } else {
                        symbol
                    }
                } else {
                    symbol
                }
            }

            InstrumentKind::OptionsContract(op) => {
                let cp = op.currency_pair();
                format!("{}{}", cp.base.code, cp.counter.code)
            }
        }
    }

    /// toSymbol(Currency)
    pub fn currency_symbol(cur: &Currency) -> String {
        if cur.code == "IOT" {
            "IOTA".to_string()
        } else {
            cur.code.clone()
        }
    }
}
