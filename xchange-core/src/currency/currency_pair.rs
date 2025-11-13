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
