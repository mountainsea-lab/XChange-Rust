use crate::currency::CurrencyAttributes;
use serde::{Deserialize, Serialize};

/**
 * A Currency class roughly modeled after {@link java.util.Currency}. Each object retains the code
 * it was acquired with -- so {@link #getInstance}("BTC").{@link #getCurrencyCode}() will always be
 * "BTC", even though the proposed ISO 4217 code is "XBT"
 */
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Currency {
    /// 货币代码(如 "BTC", "USD", "ETH")
    code: String,

    /// 货币属性
    attributes: CurrencyAttributes,
}
