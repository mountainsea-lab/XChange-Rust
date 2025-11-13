use crate::currency::currency::Currency;
use std::fmt;

/// `Instrument` trait:
///  Base object for financial instruments supported in xchange such as CurrencyPair, Future or Option
pub trait Instrument: fmt::Debug + Send + Sync {
    /// （base currency）
    fn base(&self) -> &Currency;

    /// （counter currency）
    fn counter(&self) -> &Currency;

    /// default symbol pair，example "BTC/USDT"
    fn symbol(&self) -> String {
        format!("{}/{}", self.base().code, self.counter().code)
    }
}
