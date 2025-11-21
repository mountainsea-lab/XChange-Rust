use crate::currency::currency_pair::CurrencyPair;
use crate::instrument::InstrumentDTO;

pub trait Params {}

/// Trait representing parameters that carry one or more instruments.
pub trait InstrumentsParams: Params {
    /// Returns the instruments associated with this parameter object.
    fn instruments(&self) -> &[InstrumentDTO];
}

/// Trait representing parameters that carry one or more currency pairs.
pub trait CurrencyPairsParam: Params {
    /// Returns the currency pairs associated with this parameter object.
    fn currency_pairs(&self) -> &[CurrencyPair];
}
