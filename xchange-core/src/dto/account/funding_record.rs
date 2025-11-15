use crate::currency::currency::Currency;
use crate::dto::account::{FundingType, Status};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

/// DTO representing funding information
///
///  <p>Funding information contains the detail of deposit/withdrawal transaction for a specific
///  currency
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FundingRecord {
    pub address: Option<String>,
    pub address_tag: Option<String>,
    pub date: Option<DateTime<Utc>>,
    pub currency: Currency,
    pub amount: Option<Decimal>,
    pub internal_id: Option<String>,
    pub blockchain_transaction_hash: Option<String>,
    pub type_: FundingType,
    pub status: Option<Status>,
    pub balance: Option<Decimal>,
    pub fee: Option<Decimal>,
    pub description: Option<String>,
}

impl FundingRecord {
    pub fn amount(&self) -> Option<Decimal> {
        self.amount.map(|a| a.abs())
    }

    pub fn builder(
        currency: Currency,
        amount: Decimal,
        type_: FundingType,
    ) -> FundingRecordBuilder {
        FundingRecordBuilder::new(currency, amount, type_)
    }
}

pub struct FundingRecordBuilder {
    address: Option<String>,
    address_tag: Option<String>,
    date: Option<DateTime<Utc>>,
    currency: Currency,
    amount: Option<Decimal>,
    internal_id: Option<String>,
    blockchain_transaction_hash: Option<String>,
    type_: FundingType,
    status: Option<Status>,
    balance: Option<Decimal>,
    fee: Option<Decimal>,
    description: Option<String>,
}

impl FundingRecordBuilder {
    pub fn new(currency: Currency, amount: Decimal, type_: FundingType) -> Self {
        Self {
            address: None,
            address_tag: None,
            date: None,
            currency,
            amount: Some(amount),
            internal_id: None,
            blockchain_transaction_hash: None,
            type_,
            status: None,
            balance: None,
            fee: None,
            description: None,
        }
    }

    pub fn address(mut self, address: impl Into<String>) -> Self {
        self.address = Some(address.into());
        self
    }

    pub fn address_tag(mut self, address_tag: impl Into<String>) -> Self {
        self.address_tag = Some(address_tag.into());
        self
    }

    pub fn date(mut self, date: DateTime<Utc>) -> Self {
        self.date = Some(date);
        self
    }

    pub fn internal_id(mut self, internal_id: impl Into<String>) -> Self {
        self.internal_id = Some(internal_id.into());
        self
    }

    pub fn blockchain_transaction_hash(mut self, hash: impl Into<String>) -> Self {
        self.blockchain_transaction_hash = Some(hash.into());
        self
    }

    pub fn status(mut self, status: Status) -> Self {
        self.status = Some(status);
        self
    }

    pub fn balance(mut self, balance: Decimal) -> Self {
        self.balance = Some(balance);
        self
    }

    pub fn fee(mut self, fee: Decimal) -> Self {
        self.fee = Some(fee);
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn build(self) -> FundingRecord {
        FundingRecord {
            address: self.address,
            address_tag: self.address_tag,
            date: self.date,
            currency: self.currency,
            amount: self.amount,
            internal_id: self.internal_id,
            blockchain_transaction_hash: self.blockchain_transaction_hash,
            type_: self.type_,
            status: self.status,
            balance: self.balance,
            fee: self.fee,
            description: self.description,
        }
    }
}
