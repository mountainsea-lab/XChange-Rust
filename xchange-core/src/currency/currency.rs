use crate::currency::CURRENCIES;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::hash::{Hash, Hasher};
use std::string::ToString;
use std::sync::Arc;

/// Currency attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyAttributes {
    /// all codes(including common and alternative codes)
    pub codes: BTreeSet<String>,

    /// ISO currecy code(if exists)
    pub iso_code: Option<String>,

    /// common code(main use code)
    pub common_code: String,

    /// currency name
    pub name: String,

    /// Unicode symbol
    pub unicode: String,
}

impl CurrencyAttributes {
    /// create a new CurrencyAttributes instance
    pub fn new(
        common_code: impl Into<String>,
        name: Option<String>,
        unicode: Option<String>,
        alternative_codes: &[String],
    ) -> Self {
        let common_code = common_code.into();

        // 1. create codes set (unique and sorted)
        let mut codes: BTreeSet<String> = alternative_codes.iter().cloned().collect();
        codes.insert(common_code.clone());

        // 2. try find ISO code from X-codes; first standard ISO, then X-codes
        let mut iso_code: Option<String> = None;
        let mut possible_x_code: Option<String> = None;

        for code in &codes {
            if iso_code.is_none() {
                if let Some(iso) = iso_currency::Currency::from_code(code) {
                    iso_code = Some(iso.code().to_string());
                }
            }
            if code.starts_with('X') {
                possible_x_code = Some(code.clone());
            }
        }

        let iso_code = iso_code.or(possible_x_code);

        // 3.  default name is common_code
        let final_name = if let Some(name) = name {
            name
        } else if let Some(ref iso) = iso_code {
            // try find name from ISO code
            iso_currency::Currency::from_code(iso)
                .map_or(common_code.clone(), |c| c.name().to_string())
        } else {
            common_code.clone()
        };

        // 4. unicode default to common_code
        let final_unicode = if let Some(unicode) = unicode {
            unicode
        } else if let Some(ref iso) = iso_code {
            iso_currency::Currency::from_code(iso)
                .map_or(common_code.clone(), |c| c.symbol().to_string())
        } else {
            common_code.clone()
        };

        Self {
            codes,
            iso_code,
            common_code,
            name: final_name,
            unicode: final_unicode,
        }
    }

    /// is contains the specified code
    pub fn contains_code(&self, code: &str) -> bool {
        self.codes.contains(code)
    }
}

impl Hash for CurrencyAttributes {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.common_code.hash(state);
    }
}

impl Eq for CurrencyAttributes {}

// based on common_code equality
impl PartialEq for CurrencyAttributes {
    fn eq(&self, other: &Self) -> bool {
        self.common_code == other.common_code
    }
}

///  A Currency class roughly modeled after {@link java.util.Currency}. Each object retains the code
///  it was acquired with -- so {@link #getInstance}("BTC").{@link #getCurrencyCode}() will always be
///  "BTC", even though the proposed ISO 4217 code is "XBT"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Currency {
    /// Currency code (for example, "BTC", "USD", "ETH")
    pub code: String,

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

    pub fn new_with_code_attributes(
        alternative_code: &str,
        attributes: CurrencyAttributes,
    ) -> Self {
        Self {
            code: alternative_code.to_uppercase(),
            attributes,
        }
    }

    /// Returns a Currency instance for the given currency code.
    pub fn instance(code: &str) -> Arc<Self> {
        if let Some(currency) = Self::instance_no_create(code) {
            currency
        } else {
            Self::create_currency(code, None, None, &[])
        }
    }

    /// Returns the Currency instance for the given currency code only if one already exists.
    pub fn instance_no_create(code: &str) -> Option<Arc<Currency>> {
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
    ) -> Arc<Self> {
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
                map.insert(code.clone(), Arc::new(common_currency.clone()));
            } else if !map.contains_key(code) {
                // alternative code insert map if only not contains key
                map.insert(
                    code.clone(),
                    Arc::new(Currency {
                        code: code.clone(),
                        attributes: attributes.clone(),
                    }),
                );
            }
        }

        Arc::new(common_currency)
    }

    ///  returns the original code used to create this currency instance.
    pub fn currency_code(&self) -> &str {
        &self.code
    }

    ///  retruns a Currency instance with the given code.
    ///  if code is not listed for this currency.
    pub fn code_currency(self: &Arc<Self>, code: &str) -> Option<Arc<Self>> {
        if code == self.code {
            return Some(Arc::clone(self));
        }

        let currency = Currency::instance(code);
        if Arc::ptr_eq(&currency, self) || currency.code == self.code {
            return Some(currency);
        }

        if !self.attributes.codes.iter().any(|c| c.as_str() == code) {
            return None;
        }

        Some(Arc::new(Currency {
            code: code.to_string(),
            attributes: self.attributes.clone(),
        }))
    }

    ///  get ISO 4217 Currency, if it exists, otherwise return self.
    pub fn iso_4217_currency(self: &Arc<Self>) -> Option<Arc<Self>> {
        match &self.attributes.iso_code {
            Some(iso_code) => self.code_currency(iso_code),
            None => Some(Arc::clone(self)),
        }
    }

    /// get Currency instance with the common code for this currency.
    pub fn commonly_used_currency(self: &Arc<Self>) -> Option<Arc<Self>> {
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

/// 预定义主要币种，按需初始化
pub static AED: Lazy<Arc<Currency>> = Lazy::new(|| {
    Currency::create_currency(
        "AED",
        Some("United Arab Emirates Dirham".to_string()),
        None,
        &[],
    )
});
pub static USD: Lazy<Arc<Currency>> =
    Lazy::new(|| Currency::create_currency("USD", Some("US Dollar".to_string()), None, &[]));
pub static EUR: Lazy<Arc<Currency>> =
    Lazy::new(|| Currency::create_currency("EUR", Some("Euro".to_string()), None, &[]));
