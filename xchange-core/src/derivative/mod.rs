pub mod futures_contract;
pub mod options_contract;

use crate::currency::currency_pair::CurrencyPair;
use std::str::FromStr;
use std::fmt;

pub trait Derivative {
    fn currency_pair(&self) -> &CurrencyPair;
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptionType {
    Call,
    Put,
}

impl FromStr for OptionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "C" => Ok(OptionType::Call),
            "P" => Ok(OptionType::Put),
            _ => Err(format!("Unknown option type: {}", s)),
        }
    }
}

impl fmt::Display for OptionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let p = match self {
            OptionType::Call => "C",
            OptionType::Put => "P",
        };
        write!(f, "{}", p)
    }
}
