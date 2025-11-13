//！ A Currency represents a monetary unit, such as the dollar, euro, or yen.
//！ It has a unique three-letter code, a name, and a number of decimal places.
//! It also has a set of alternative codes that can be used to represent the same currency.

pub mod currency;
mod currency_pair;

use crate::currency::currency::Currency;
use once_cell::sync::Lazy;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

/// global currency manager
static CURRENCIES: Lazy<Mutex<BTreeMap<String, Arc<Currency>>>> =
    Lazy::new(|| Mutex::new(BTreeMap::new()));
