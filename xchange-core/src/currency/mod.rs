//！ A Currency represents a monetary unit, such as the dollar, euro, or yen.
//！ It has a unique three-letter code, a name, and a number of decimal places.
//! It also has a set of alternative codes that can be used to represent the same currency.

pub mod currency;

use iso_currency::Currency;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

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
                if let Some(iso) = Currency::from_code(code) {
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
            Currency::from_code(iso).map_or(common_code.clone(), |c| c.name().to_string())
        } else {
            common_code.clone()
        };

        // 4. unicode default to common_code
        let final_unicode = if let Some(unicode) = unicode {
            unicode
        } else if let Some(ref iso) = iso_code {
            Currency::from_code(iso).map_or(common_code.clone(), |c| c.symbol().to_string())
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

impl std::hash::Hash for CurrencyAttributes {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
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
