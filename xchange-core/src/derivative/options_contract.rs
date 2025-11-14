use crate::currency::currency::Currency;
use crate::currency::currency_pair::CurrencyPair;
use crate::derivative::{Derivative, OptionType};
use crate::instrument::Instrument;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use std::cmp::Ordering;
use std::fmt;
use std::hash::Hasher;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct OptionsContract {
    pub currency_pair: CurrencyPair,
    pub expire_date: NaiveDate,
    pub strike: Decimal,
    pub option_type: OptionType,
}

impl OptionsContract {
    pub fn new(
        currency_pair: CurrencyPair,
        expire_date: NaiveDate,
        strike: Decimal,
        option_type: OptionType,
    ) -> Self {
        Self {
            currency_pair,
            expire_date,
            strike,
            option_type,
        }
    }

    /// Parse from symbol string: "BASE/QUOTE/YYMMDD/STRIKE/C"
    pub fn from_symbol(symbol: &str) -> Result<Self, String> {
        let parts: Vec<&str> = symbol.split('/').collect();

        if parts.len() != 5 {
            return Err(format!(
                "Could not parse options contract from '{}'",
                symbol
            ));
        }

        let base = parts[0];
        let counter = parts[1];
        let expire_date_str = parts[2];
        let strike_str = parts[3];
        let option_type_str = parts[4];

        // Currency pair
        let currency_pair = CurrencyPair::from_symbols(base, counter);

        // Expire date (YYMMDD)
        let expire_date = NaiveDate::parse_from_str(expire_date_str, "%y%m%d")
            .map_err(|_| format!("Could not parse expire date from '{}'", symbol))?;

        // Strike
        let strike = Decimal::from_str_exact(strike_str)
            .map_err(|_| format!("Could not parse strike from '{}'", symbol))?;

        // Option type
        let option_type = OptionType::from_str(option_type_str)
            .map_err(|_| format!("Could not parse option type from '{}'", symbol))?;

        Ok(Self {
            currency_pair,
            expire_date,
            strike,
            option_type,
        })
    }

    pub fn expire_date(&self) -> NaiveDate {
        self.expire_date
    }

    pub fn strike(&self) -> &Decimal {
        &self.strike
    }

    pub fn option_type(&self) -> OptionType {
        self.option_type
    }

    fn symbol(&self) -> String {
        format!(
            "{}/{}/{}/{}/{}",
            self.currency_pair.base,
            self.currency_pair.counter,
            self.expire_date.format("%y%m%d"),
            self.strike,
            self.option_type.to_string()
        )
    }
}

impl Instrument for OptionsContract {
    fn base(&self) -> Arc<Currency> {
        self.currency_pair.base()
    }

    fn counter(&self) -> Arc<Currency> {
        self.currency_pair.counter()
    }

    fn symbol(&self) -> String {
        self.symbol()
    }
}

impl Derivative for OptionsContract {
    fn currency_pair(&self) -> &CurrencyPair {
        &self.currency_pair
    }
}

impl PartialEq for OptionsContract {
    fn eq(&self, other: &Self) -> bool {
        self.currency_pair == other.currency_pair
            && self.expire_date == other.expire_date
            && self.strike == other.strike
            && self.option_type == other.option_type
    }
}

impl Eq for OptionsContract {}

impl std::hash::Hash for OptionsContract {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.currency_pair.hash(state);
        self.expire_date.hash(state);
        self.strike.hash(state);
        self.option_type.hash(state);
    }
}

impl PartialOrd for OptionsContract {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OptionsContract {
    fn cmp(&self, other: &Self) -> Ordering {
        // COMPARATOR: currencyPair -> expireDate -> strike -> type
        match self.currency_pair.cmp(&other.currency_pair) {
            Ordering::Equal => match self.expire_date.cmp(&other.expire_date) {
                Ordering::Equal => match self.strike.cmp(&other.strike) {
                    Ordering::Equal => self.option_type.cmp(&other.option_type),
                    other => other,
                },
                other => other,
            },
            other => other,
        }
    }
}

impl fmt::Display for OptionsContract {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

/// OptionsContract Builder helper
pub struct OptionsContractBuilder {
    currency_pair: Option<CurrencyPair>,
    expire_date: Option<NaiveDate>,
    strike: Option<Decimal>,
    option_type: Option<OptionType>,
}

impl OptionsContractBuilder {
    pub fn new() -> Self {
        Self {
            currency_pair: None,
            expire_date: None,
            strike: None,
            option_type: None,
        }
    }

    pub fn currency_pair(mut self, val: CurrencyPair) -> Self {
        self.currency_pair = Some(val);
        self
    }

    pub fn expire_date(mut self, val: NaiveDate) -> Self {
        self.expire_date = Some(val);
        self
    }

    pub fn strike(mut self, val: Decimal) -> Self {
        self.strike = Some(val);
        self
    }

    pub fn option_type(mut self, val: OptionType) -> Self {
        self.option_type = Some(val);
        self
    }

    /// Constructor OptionsContract
    pub fn build(self) -> OptionsContract {
        OptionsContract {
            currency_pair: self.currency_pair.expect("currency_pair is required"),
            expire_date: self.expire_date.expect("expire_date is required"),
            strike: self.strike.expect("strike is required"),
            option_type: self.option_type.expect("option_type is required"),
        }
    }
}
