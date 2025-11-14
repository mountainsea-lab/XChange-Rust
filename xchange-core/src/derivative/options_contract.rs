use crate::currency::currency_pair::CurrencyPair;
use crate::derivative::OptionType;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use std::str::FromStr;

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
}

impl std::fmt::Display for OptionsContract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/{}/{}/{}/{}",
            self.currency_pair.base,
            self.currency_pair.counter,
            self.expire_date.format("%y%m%d"),
            self.strike,
            self.option_type.to_string()
        )
    }
}
