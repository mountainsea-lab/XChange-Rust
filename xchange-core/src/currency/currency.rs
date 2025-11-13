use crate::currency::CurrencyAttributes;

/**
 * A Currency class roughly modeled after {@link java.util.Currency}. Each object retains the code
 * it was acquired with -- so {@link #getInstance}("BTC").{@link #getCurrencyCode}() will always be
 * "BTC", even though the proposed ISO 4217 code is "XBT"
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Currency {
    pub code: &'static str,
    pub attrs: &'static CurrencyAttributes,
}
