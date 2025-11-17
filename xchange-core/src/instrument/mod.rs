use crate::currency::currency::Currency;
use crate::currency::currency_pair::CurrencyPair;
use crate::derivative::OptionType;
use crate::derivative::futures_contract::FuturesContract;
use crate::derivative::options_contract::OptionsContract;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::Arc;

/// `Instrument` trait:
///  Base object for financial instruments supported in xchange such as CurrencyPair, Future or Option
pub trait Instrument: fmt::Debug + Send + Sync {
    /// （base currency）
    fn base(&self) -> Arc<Currency>;

    /// （counter currency）
    fn counter(&self) -> Arc<Currency>;

    /// default symbol pair，example "BTC/USDT"
    fn symbol(&self) -> String {
        let base = self.base();
        let counter = self.counter();
        format!("{}/{}", base.code, counter.code)
    }
}

#[derive(Debug, Clone)]
pub enum InstrumentKind {
    CurrencyPair(CurrencyPair),
    FuturesContract(FuturesContract),
    OptionsContract(OptionsContract),
}

impl Instrument for InstrumentKind {
    fn base(&self) -> Arc<Currency> {
        match self {
            InstrumentKind::CurrencyPair(cp) => cp.base(),
            InstrumentKind::FuturesContract(fc) => fc.base(),
            InstrumentKind::OptionsContract(oc) => oc.base(),
        }
    }

    fn counter(&self) -> Arc<Currency> {
        match self {
            InstrumentKind::CurrencyPair(cp) => cp.counter(),
            InstrumentKind::FuturesContract(fc) => fc.counter(),
            InstrumentKind::OptionsContract(oc) => oc.counter(),
        }
    }
    fn symbol(&self) -> String {
        match self {
            InstrumentKind::CurrencyPair(cp) => cp.symbol(),
            InstrumentKind::FuturesContract(fc) => fc.symbol(),
            InstrumentKind::OptionsContract(oc) => oc.symbol(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash)]
pub enum InstrumentDTO {
    Spot {
        base: String,
        counter: String,
    },
    Futures {
        base: String,
        counter: String,
        prompt: Option<String>,
    },
    Options {
        base: String,
        counter: String,
        strike: Decimal,
        expire_date: NaiveDate,
        option_type: OptionType,
    },
}

impl From<InstrumentDTO> for InstrumentKind {
    fn from(dto: InstrumentDTO) -> Self {
        match dto {
            InstrumentDTO::Spot { base, counter } => {
                InstrumentKind::CurrencyPair(CurrencyPair::from_symbols(&base, &counter))
            }
            InstrumentDTO::Futures {
                base,
                counter,
                prompt,
            } => InstrumentKind::FuturesContract(FuturesContract::new(
                Arc::new(CurrencyPair::from_symbols(&base, &counter)),
                prompt,
            )),
            InstrumentDTO::Options {
                base,
                counter,
                strike,
                expire_date,
                option_type,
            } => InstrumentKind::OptionsContract(OptionsContract::new(
                CurrencyPair::from_symbols(&base, &counter),
                expire_date,
                strike,
                option_type,
            )),
        }
    }
}
impl From<InstrumentKind> for InstrumentDTO {
    fn from(kind: InstrumentKind) -> Self {
        match kind {
            InstrumentKind::CurrencyPair(cp) => InstrumentDTO::Spot {
                base: cp.base().code.clone(),
                counter: cp.counter().code.clone(),
            },

            InstrumentKind::FuturesContract(fc) => InstrumentDTO::Futures {
                base: fc.base().code.clone(),
                counter: fc.counter().code.clone(),
                prompt: fc.prompt.clone(),
            },

            InstrumentKind::OptionsContract(oc) => InstrumentDTO::Options {
                base: oc.base().code.clone(),
                counter: oc.counter().code.clone(),
                strike: oc.strike,
                expire_date: oc.expire_date,
                option_type: oc.option_type,
            },
        }
    }
}

impl InstrumentDTO {
    // Deprecating the `get_currency_pair` method
    #[deprecated]
    pub fn get_currency_pair(&self) -> Option<InstrumentDTO> {
        match self {
            InstrumentDTO::Spot { base, counter } => Some(InstrumentDTO::Spot {
                base: base.clone(),
                counter: counter.clone(),
            }),
            InstrumentDTO::Futures { base, counter, .. } => Some(InstrumentDTO::Futures {
                base: base.clone(),
                counter: counter.clone(),
                prompt: None,
            }),
            InstrumentDTO::Options { base, counter, .. } => Some(InstrumentDTO::Options {
                base: base.clone(),
                counter: counter.clone(),
                strike: Decimal::ZERO,
                expire_date: NaiveDate::from_ymd_opt(2023, 1, 1)?,
                option_type: OptionType::Call,
            }),
            // Other cases
            _ => None,
        }
    }
}
