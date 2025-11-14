pub mod futures_contract;
pub mod options_contract;

use crate::currency::currency_pair::CurrencyPair;
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

pub trait Derivative {
    fn currency_pair(&self) -> &CurrencyPair;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptionType {
    Call,
    Put,
}

impl PartialOrd for OptionType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OptionType {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (OptionType::Call, OptionType::Call) => Ordering::Equal,
            (OptionType::Call, OptionType::Put) => Ordering::Less,
            (OptionType::Put, OptionType::Call) => Ordering::Greater,
            (OptionType::Put, OptionType::Put) => Ordering::Equal,
        }
    }
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
