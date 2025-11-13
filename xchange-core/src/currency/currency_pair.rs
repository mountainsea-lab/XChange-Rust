use crate::currency::currency::Currency;
use serde::{Deserialize, Serialize};

///  Value object to provide the following to API:
///
///   <ul>
///     <li>Provision of major currency symbol pairs (EUR/USD, GBP/USD etc)
///     <li>Provision of arbitrary symbol pairs for exchange index trading, notional currencies etc
///   </ul>
///
///   <p>Symbol pairs are quoted, for example, as EUR/USD 1.25 such that 1 EUR can be purchased with
///   1.25 USD
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyPair {
    pub base: Currency,
    pub counter: Currency,
}

impl CurrencyPair {
    ///    Full constructor In general the CurrencyPair.base is what you're wanting to buy/sell. The
    ///    CurrencyPair.counter is what currency you want to use to pay/receive for your purchase/sale.
    ///
    ///    @param base The base currency is what you're wanting to buy/sell
    ///    @param counter The counter currency is what currency you want to use to pay/receive for your
    ///       purchase/sale.
    pub fn new(base: Currency, counter: Currency) -> Self {
        Self { base, counter }
    }

    ///  String constructor In general the CurrencyPair.base is what you're wanting to buy/sell. The
    ///    CurrencyPair.counter is what currency you want to use to pay/receive for your purchase/sale.
    ///
    ///    @param baseSymbol The base symbol is what you're wanting to buy/sell
    ///    @param counterSymbol The counter symbol is what currency you want to use to pay/receive for
    ///       your purchase/sale.
    pub fn from_symbols(base_symbol: &str, counter_symbol: &str) -> Self {
        let base = Currency::instance(base_symbol);
        let counter = Currency::instance(counter_symbol);
        Self::new(base, counter)
    }

    /// 返回 "EUR/USD" 等格式
    pub fn symbol(&self) -> String {
        format!("{}/{}", self.base.code, self.counter.code)
    }
}
