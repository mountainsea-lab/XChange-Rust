use crate::currency::currency::Currency;
use std::fmt;
use std::sync::Arc;

/// `Instrument` trait:
///  Base object for financial instruments supported in xchange such as CurrencyPair, Future or Option
pub trait Instrument: fmt::Debug + Send + Sync {
    /// （base currency）
    fn base(&self) -> Arc<Currency>;

    /// （counter currency）
    fn counter(&self) -> Arc<Currency>;

    /// default symbol pair，example "BTC/USDT"
    fn symbol(&self) -> String {
        let base = self.base();
        let counter = self.counter();
        format!("{}/{}", base.code, counter.code)
    }
}
