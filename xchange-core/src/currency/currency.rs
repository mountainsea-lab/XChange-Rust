use crate::currency::CurrencyAttributes;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::hash::{Hash, Hasher};
use std::sync::Mutex;

static CURRENCIES: Lazy<Mutex<BTreeMap<String, Currency>>> =
    Lazy::new(|| Mutex::new(BTreeMap::new()));

///  A Currency class roughly modeled after {@link java.util.Currency}. Each object retains the code
///  it was acquired with -- so {@link #getInstance}("BTC").{@link #getCurrencyCode}() will always be
///  "BTC", even though the proposed ISO 4217 code is "XBT"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Currency {
    /// Currency code (for example, "BTC", "USD", "ETH")
    code: String,

    /// Currency attributes
    attributes: CurrencyAttributes,
}

impl PartialEq for Currency {
    fn eq(&self, other: &Self) -> bool {
        self.attributes == other.attributes
    }
}

impl Eq for Currency {}

impl Hash for Currency {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.attributes.hash(state)
    }
}

impl PartialOrd for Currency {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Currency {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.attributes == other.attributes {
            return Ordering::Equal;
        }

        let mut ordering = self.code.cmp(&other.code);
        if ordering == Ordering::Equal {
            ordering = self.attributes.name.cmp(&other.attributes.name);
        }
        if ordering == Ordering::Equal {
            // 用 hash 值作为最后的 tie-breaker，类似 Java 的 hashCode - other.hashCode
            let self_hash = {
                use std::collections::hash_map::DefaultHasher;
                let mut hasher = DefaultHasher::new();
                self.hash(&mut hasher);
                hasher.finish()
            };
            let other_hash = {
                use std::collections::hash_map::DefaultHasher;
                let mut hasher = DefaultHasher::new();
                other.hash(&mut hasher);
                hasher.finish()
            };
            ordering = self_hash.cmp(&other_hash);
        }

        ordering
    }
}

// toString()
impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code)
    }
}

impl Currency {
    ///  Public constructor. Links to an existing currency.
    pub fn new(code: &str) -> Self {
        // get_instance 逻辑：如果存在返回已有实例，否则创建新的
        let existing = Self::instance(code);
        Self {
            code: code.to_uppercase(),
            attributes: existing.attributes.clone(),
        }
    }

    fn new_with_code_attributes(alternative_code: &str, attributes: CurrencyAttributes) -> Self {
        Self {
            code: alternative_code.to_uppercase(),
            attributes,
        }
    }

    /// Returns a Currency instance for the given currency code.
    pub fn instance(code: &str) -> Currency {
        if let Some(currency) = Self::instance_no_create(code) {
            currency
        } else {
            Self::create_currency(code, None, None, &[])
        }
    }

    /// Returns the Currency instance for the given currency code only if one already exists.
    pub fn instance_no_create(code: &str) -> Option<Currency> {
        let map = CURRENCIES.lock().unwrap();
        map.get(&code.to_uppercase()).cloned()
    }

    /// Factory
    /// @param commonCode commonly used code for this currency: "BTC"
    /// @param name Name of the currency: "Bitcoin"
    /// @param unicode Unicode symbol for the currency: "\u20BF" or "฿"
    /// @param alternativeCodes Alternative codes for the currency: "XBT"
    pub fn create_currency(
        common_code: &str,
        name: Option<String>,
        unicode: Option<String>,
        alternative_codes: &[String],
    ) -> Currency {
        let common_code = common_code.to_uppercase();

        // 1. construct CurrencyAttributes
        let mut codes = BTreeSet::new();
        codes.insert(common_code.clone());
        for code in alternative_codes {
            codes.insert(code.to_uppercase());
        }

        let attributes =
            CurrencyAttributes::new(common_code.clone(), name, unicode, alternative_codes);

        // 2. create Currency instance
        let common_currency = Currency {
            code: common_code.clone(),
            attributes: attributes.clone(),
        };

        // 3. insert global map
        let mut map = CURRENCIES.lock().unwrap();
        for code in &codes {
            let common_code = common_code.clone();
            if *code == common_code {
                // common code allways insert map
                map.insert(code.clone(), common_currency.clone());
            } else if !map.contains_key(code) {
                // alternative code insert map if only not contains key
                map.insert(
                    code.clone(),
                    Currency {
                        code: code.clone(),
                        attributes: attributes.clone(),
                    },
                );
            }
        }

        common_currency
    }

    ///  returns the original code used to create this currency instance.
    pub fn code(&self) -> &str {
        &self.code
    }

    ///  retruns a Currency instance with the given code.
    ///  if code is not listed for this currency.
    pub fn code_currency(&self, code: &str) -> Self {
        if code == self.code {
            return self.clone();
        }

        let currency = Self::instance(code);
        if currency == *self {
            return currency;
        }

        if !self.attributes.codes.contains(code) {
            panic!("Code not listed for this currency: {}", code);
        }

        Currency {
            code: code.to_string(),
            attributes: self.attributes.clone(),
        }
    }

    ///  get ISO 4217 Currency, if it exists, otherwise return self.
    pub fn iso_4217_currency(&self) -> Self {
        match &self.attributes.iso_code {
            Some(iso_code) => self.code_currency(iso_code),
            None => self.clone(),
        }
    }

    /// get Currency instance with the common code for this currency.
    pub fn commonly_used_currency(&self) -> Self {
        self.code_currency(&self.attributes.common_code)
    }

    /// get all codes associated with this currency.
    pub fn currency_codes(&self) -> BTreeSet<String> {
        self.attributes.codes.clone()
    }

    /// get currency symbol (Unicode)
    pub fn symbol(&self) -> &str {
        &self.attributes.unicode
    }

    /// show name of the currency
    pub fn display_name(&self) -> &str {
        &self.attributes.name
    }
}
