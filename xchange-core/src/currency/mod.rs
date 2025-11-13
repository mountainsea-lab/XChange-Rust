//！ A Currency represents a monetary unit, such as the dollar, euro, or yen.
//！ It has a unique three-letter code, a name, and a number of decimal places.
//! It also has a set of alternative codes that can be used to represent the same currency.

pub mod currency;

use crate::currency::currency::Currency;
use once_cell::sync::Lazy;
use std::collections::BTreeMap;
use std::sync::Mutex;

/// global currency init
static CURRENCIES: Lazy<Mutex<BTreeMap<String, Currency>>> = Lazy::new(|| {
    let mut m = BTreeMap::new();
    macro_rules! create_currency {
        ($code:expr, $name:expr $(, $alt:expr)*) => {{
            let currency = Currency::create_currency(&$code, $name, None, &[]);
            m.insert($code, currency);
        }};
    }

    // create create_currency
    create_currency!(
        "AED".to_string(),
        Some("United Arab Emirates Dirham".to_string()),
        None,
        &[]
    );
    create_currency!(
        "AFN".to_string(),
        Some("Afghan Afghani".to_string()),
        None,
        &[]
    );
    create_currency!(
        "ALL".to_string(),
        Some("Albanian Lek".to_string()),
        None,
        &[]
    );
    create_currency!(
        "AMD".to_string(),
        Some("Armenian Dram".to_string()),
        None,
        &[]
    );
    create_currency!("ANC".to_string(), Some("Anoncoin".to_string()), None, &[]);
    create_currency!(
        "ANG".to_string(),
        Some("Netherlands Antillean Guilder".to_string()),
        None,
        &[]
    );
    create_currency!(
        "AOA".to_string(),
        Some("Angolan Kwanza".to_string()),
        None,
        &[]
    );
    create_currency!("ARN".to_string(), Some("Aeron".to_string()), None, &[]);

    // last return Mutex  map
    Mutex::new(m)
});
