use crate::currency::CurrencyAttributes;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Mutex;

static CURRENCIES: Lazy<Mutex<BTreeMap<String, Currency>>> =
    Lazy::new(|| Mutex::new(BTreeMap::new()));

///  A Currency class roughly modeled after {@link java.util.Currency}. Each object retains the code
///  it was acquired with -- so {@link #getInstance}("BTC").{@link #getCurrencyCode}() will always be
///  "BTC", even though the proposed ISO 4217 code is "XBT"
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Currency {
    /// Currency code (for example, "BTC", "USD", "ETH")
    code: String,

    /// Currency attributes
    attributes: CurrencyAttributes,
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

        // 1. 构建 CurrencyAttributes
        let mut codes = BTreeSet::new();
        codes.insert(common_code.clone());
        for code in alternative_codes {
            codes.insert(code.to_uppercase());
        }

        let attributes =
            CurrencyAttributes::new(common_code.clone(), name, unicode, alternative_codes);

        // 2. 创建 Currency 对象
        let common_currency = Currency {
            code: common_code.clone(),
            attributes: attributes.clone(),
        };

        // 3. 插入全局 Map
        let mut map = CURRENCIES.lock().unwrap();
        for code in &codes {
            let common_code = common_code.clone();
            if *code == common_code {
                // common code 总是存入 Map
                map.insert(code.clone(), common_currency.clone());
            } else if !map.contains_key(code) {
                // alternative code 只在 Map 中不存在时存入
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
}
