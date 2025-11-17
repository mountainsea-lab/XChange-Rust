use crate::currency::currency::Currency;
use crate::currency::currency_pair::CurrencyPair;
use crate::derivative::Derivative;
use crate::instrument::Instrument;
use regex::Regex;
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct FuturesContract {
    // The CurrencyPair the FuturesContract is based upon
    currency_pair: Arc<CurrencyPair>,
    // The Date when the FuturesContract expires, when null it is perpetual
    pub(crate) prompt: Option<String>, // null ⇒ perpetual
}

impl FuturesContract {
    /// Equivalent to Java constructor FuturesContract(CurrencyPair cp, String prompt)
    pub fn new(currency_pair: Arc<CurrencyPair>, prompt: Option<String>) -> Self {
        Self {
            currency_pair,
            prompt,
        }
    }

    /// Rust-recommended version: returns Result instead of panic
    pub fn from_symbol(symbol: &str) -> Result<Self, String> {
        let parts: Vec<&str> = symbol.split('/').collect();

        if parts.len() < 3 {
            return Err(format!(
                "Could not parse futures contract from '{}'",
                symbol
            ));
        }

        let base = parts[0];
        let counter = parts[1];
        let prompt = parts[2].to_string();

        Ok(Self {
            currency_pair: Arc::new(CurrencyPair::from_symbols(base, counter)),
            prompt: Some(prompt),
        })
    }

    pub fn prompt(&self) -> Option<&str> {
        self.prompt.as_deref()
    }

    /// return this.prompt.matches("(?i)PERP|SWAP|PERPETUAL");
    pub fn is_perpetual(&self) -> bool {
        static RE: once_cell::sync::Lazy<Regex> =
            once_cell::sync::Lazy::new(|| Regex::new("(?i)^(PERP|SWAP|PERPETUAL)$").unwrap());

        match &self.prompt {
            Some(p) => RE.is_match(p),
            None => false,
        }
    }

    pub fn symbol(&self) -> String {
        format!("{}/{:?}", self.currency_pair, self.prompt)
    }
}

impl Instrument for FuturesContract {
    fn base(&self) -> Arc<Currency> {
        self.currency_pair.base()
    }

    fn counter(&self) -> Arc<Currency> {
        self.currency_pair.counter()
    }

    /// return "EUR/USD" ..format
    fn symbol(&self) -> String {
        self.symbol()
    }
}

impl Derivative for FuturesContract {
    fn currency_pair(&self) -> &CurrencyPair {
        &self.currency_pair
    }
}

/* ---- Eq / PartialEq ---- */
impl PartialEq for FuturesContract {
    fn eq(&self, other: &Self) -> bool {
        self.currency_pair == other.currency_pair && self.prompt == other.prompt
    }
}

impl Eq for FuturesContract {}

/* ---- Hash ---- */
impl Hash for FuturesContract {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.currency_pair.hash(state);
        self.prompt.hash(state);
    }
}

impl Ord for FuturesContract {
    fn cmp(&self, other: &Self) -> Ordering {
        (&self.currency_pair, &self.prompt).cmp(&(&other.currency_pair, &other.prompt))
    }
}

impl PartialOrd for FuturesContract {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Display call symbol()，keep to_string() and symbol() same
impl fmt::Display for FuturesContract {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}
