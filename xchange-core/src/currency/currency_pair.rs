use crate::currency::currency::Currency;
use crate::instrument::Instrument;
use once_cell::sync::Lazy;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

///  Value object to provide the following to API:
///
///   <ul>
///     <li>Provision of major currency symbol pairs (EUR/USD, GBP/USD etc)
///     <li>Provision of arbitrary symbol pairs for exchange index trading, notional currencies etc
///   </ul>
///
///   <p>Symbol pairs are quoted, for example, as EUR/USD 1.25 such that 1 EUR can be purchased with
///   1.25 USD
#[derive(Debug, Clone)]
pub struct CurrencyPair {
    pub base: Arc<Currency>,
    pub counter: Arc<Currency>,
}

impl CurrencyPair {
    ///    Full constructor In general the CurrencyPair.base is what you're wanting to buy/sell. The
    ///    CurrencyPair.counter is what currency you want to use to pay/receive for your purchase/sale.
    ///
    ///    @param base The base currency is what you're wanting to buy/sell
    ///    @param counter The counter currency is what currency you want to use to pay/receive for your
    ///       purchase/sale.
    pub fn new(base: Arc<Currency>, counter: Arc<Currency>) -> Self {
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

    /// Parse currency pair from a string in the same format as returned by toString() method - ABC/XYZ or "ETH-USD"
    pub fn from_str_pair(s: &str) -> Result<Self, String> {
        let delimiter = if s.contains('-') {
            '-'
        } else if s.contains('/') {
            '/'
        } else {
            return Err(format!("Could not parse currency pair from '{}'", s));
        };

        let parts: Vec<&str> = s.splitn(2, delimiter).collect();
        if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
            return Err(format!("Could not parse currency pair from '{}'", s));
        }

        Ok(Self::from_symbols(parts[0].trim(), parts[1].trim()))
    }

    /// return "EUR/USD" ..format
    pub fn symbol(&self) -> String {
        format!("{}/{}", self.base.code, self.counter.code)
    }

    pub fn contains(&self, currency: &Arc<Currency>) -> bool {
        Arc::ptr_eq(&self.base, currency)
            || Arc::ptr_eq(&self.counter, currency)
            || self.base.code == currency.code
            || self.counter.code == currency.code
    }
}

impl Instrument for CurrencyPair {
    fn base(&self) -> Arc<Currency> {
        Arc::clone(&self.base)
    }

    fn counter(&self) -> Arc<Currency> {
        Arc::clone(&self.counter)
    }

    fn symbol(&self) -> String {
        self.symbol()
    }
}

/// Display output
impl fmt::Display for CurrencyPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.base.code, self.counter.code)
    }
}

/// impl PartialEq / Eq
impl PartialEq for CurrencyPair {
    fn eq(&self, other: &Self) -> bool {
        // Both Arc Direct Same Currency
        let base_eq = Arc::ptr_eq(&self.base, &other.base) || self.base.code == other.base.code;
        let counter_eq =
            Arc::ptr_eq(&self.counter, &other.counter) || self.counter.code == other.counter.code;
        base_eq && counter_eq
    }
}

impl Eq for CurrencyPair {}

/// impl Hash
impl Hash for CurrencyPair {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // use code field，void depth hash attributes
        self.base.code.hash(state);
        self.counter.code.hash(state);
    }
}

impl Ord for CurrencyPair {
    fn cmp(&self, other: &Self) -> Ordering {
        // 将 base 和 counter 的比较结果转换为 -1/0/1
        let base_cmp = match self.base.code.cmp(&other.base.code) {
            Ordering::Less => -1i32,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        };

        let counter_cmp = match self.counter.code.cmp(&other.counter.code) {
            Ordering::Less => -1i32,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        };

        // Simulate Java's 16-bit left shift logic
        let java_like_cmp = (base_cmp << 16) + counter_cmp;

        // Convert Rust Ordering
        if java_like_cmp < 0 {
            Ordering::Less
        } else if java_like_cmp > 0 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for CurrencyPair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Serialize for CurrencyPair {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}/{}", self.base.code, self.counter.code);
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for CurrencyPair {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() != 2 {
            return Err(serde::de::Error::custom("Invalid CurrencyPair string"));
        }
        Ok(Self::from_symbols(parts[0], parts[1]))
    }
}

/// Currency_pair const todo
pub static EUR_USD: Lazy<Arc<CurrencyPair>> =
    Lazy::new(|| Arc::new(CurrencyPair::from_symbols("EUR", "USD")));
