mod futures_contract;

use crate::currency::currency_pair::CurrencyPair;

pub trait Derivative {
    fn currency_pair(&self) -> &CurrencyPair;
}
