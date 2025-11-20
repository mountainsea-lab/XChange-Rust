use crate::currency::currency::Currency;
use crate::dto::meta::fee_tier::FeeTier;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstrumentMetaData {
    /// Trading fee (fraction)
    #[serde(rename = "trading_fee")]
    pub trading_fee: Option<Decimal>,

    /// Trading fee tiers sorted ascending by quantity
    #[serde(rename = "fee_tiers", default)]
    pub fee_tiers: Vec<FeeTier>,

    /// Minimum trade amount
    #[serde(rename = "min_amount")]
    pub minimum_amount: Option<Decimal>,

    /// Maximum trade amount
    #[serde(rename = "max_amount")]
    pub maximum_amount: Option<Decimal>,

    /// Minimum counter amount
    #[serde(rename = "counter_min_amount")]
    pub counter_minimum_amount: Option<Decimal>,

    /// Maximum counter amount
    #[serde(rename = "counter_max_amount")]
    pub counter_maximum_amount: Option<Decimal>,

    /// Decimal places of price
    #[serde(rename = "price_scale")]
    pub price_scale: Option<u32>,

    /// Decimal places of volume
    #[serde(rename = "volume_scale")]
    pub volume_scale: Option<u32>,

    /// Amount step size
    #[serde(rename = "amount_step_size")]
    pub amount_step_size: Option<Decimal>,

    /// Price step size
    #[serde(rename = "price_step_size")]
    pub price_step_size: Option<Decimal>,

    /// Trading fee currency
    #[serde(rename = "trading_fee_currency")]
    pub trading_fee_currency: Option<Currency>,

    /// Market order enabled
    #[serde(rename = "market_order_enabled", default)]
    pub market_order_enabled: bool,

    /// Contract value
    #[serde(rename = "contract_value")]
    pub contract_value: Option<Decimal>,
}

impl InstrumentMetaData {
    pub fn new(
        trading_fee: Option<Decimal>,
        mut fee_tiers: Vec<FeeTier>,
        minimum_amount: Option<Decimal>,
        maximum_amount: Option<Decimal>,
        counter_minimum_amount: Option<Decimal>,
        counter_maximum_amount: Option<Decimal>,
        price_scale: Option<u32>,
        volume_scale: Option<u32>,
        amount_step_size: Option<Decimal>,
        price_step_size: Option<Decimal>,
        trading_fee_currency: Option<Currency>,
        market_order_enabled: bool,
        contract_value: Option<Decimal>,
    ) -> Self {
        // Java: Arrays.sort(feeTiers)
        fee_tiers.sort();

        Self {
            trading_fee,
            fee_tiers,
            minimum_amount: minimum_amount.map(|d| d.normalize()),
            maximum_amount: maximum_amount.map(|d| d.normalize()),
            counter_minimum_amount: counter_minimum_amount.map(|d| d.normalize()),
            counter_maximum_amount: counter_maximum_amount.map(|d| d.normalize()),
            price_scale,
            volume_scale,
            amount_step_size: amount_step_size.map(|d| d.normalize()),
            price_step_size: price_step_size.map(|d| d.normalize()),
            trading_fee_currency,
            market_order_enabled,
            contract_value,
        }
    }

    pub fn builder() -> InstrumentMetaDataBuilder {
        InstrumentMetaDataBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct InstrumentMetaDataBuilder {
    trading_fee: Option<Decimal>,
    fee_tiers: Option<Vec<FeeTier>>,
    minimum_amount: Option<Decimal>,
    maximum_amount: Option<Decimal>,
    counter_minimum_amount: Option<Decimal>,
    counter_maximum_amount: Option<Decimal>,
    price_scale: Option<u32>,
    volume_scale: Option<u32>,
    amount_step_size: Option<Decimal>,
    price_step_size: Option<Decimal>,
    trading_fee_currency: Option<Currency>,
    market_order_enabled: bool,
    contract_value: Option<Decimal>,
}

impl InstrumentMetaDataBuilder {
    pub fn trading_fee(mut self, fee: Decimal) -> Self {
        self.trading_fee = Some(fee);
        self
    }

    pub fn fee_tiers(mut self, tiers: Vec<FeeTier>) -> Self {
        self.fee_tiers = Some(tiers);
        self
    }

    pub fn minimum_amount(mut self, v: Decimal) -> Self {
        self.minimum_amount = Some(v.normalize());
        self
    }

    pub fn maximum_amount(mut self, v: Decimal) -> Self {
        self.maximum_amount = Some(v.normalize());
        self
    }

    pub fn counter_minimum_amount(mut self, v: Decimal) -> Self {
        self.counter_minimum_amount = Some(v.normalize());
        self
    }

    pub fn counter_maximum_amount(mut self, v: Decimal) -> Self {
        self.counter_maximum_amount = Some(v.normalize());
        self
    }

    pub fn price_scale(mut self, v: u32) -> Self {
        self.price_scale = Some(v);
        self
    }

    pub fn volume_scale(mut self, v: u32) -> Self {
        self.volume_scale = Some(v);
        self
    }

    pub fn amount_step_size(mut self, v: Decimal) -> Self {
        self.amount_step_size = Some(v.normalize());
        self
    }

    pub fn price_step_size(mut self, v: Decimal) -> Self {
        self.price_step_size = Some(v.normalize());
        self
    }

    pub fn trading_fee_currency(mut self, c: Currency) -> Self {
        self.trading_fee_currency = Some(c);
        self
    }

    pub fn market_order_enabled(mut self, enabled: bool) -> Self {
        self.market_order_enabled = enabled;
        self
    }

    pub fn contract_value(mut self, v: Decimal) -> Self {
        self.contract_value = Some(v);
        self
    }

    pub fn build(mut self) -> InstrumentMetaData {
        // Java: Arrays.sort(feeTiers)
        let mut tiers = self.fee_tiers.unwrap_or_default();
        tiers.sort();

        InstrumentMetaData {
            trading_fee: self.trading_fee,
            fee_tiers: tiers,
            minimum_amount: self.minimum_amount,
            maximum_amount: self.maximum_amount,
            counter_minimum_amount: self.counter_minimum_amount,
            counter_maximum_amount: self.counter_maximum_amount,
            price_scale: self.price_scale,
            volume_scale: self.volume_scale,
            amount_step_size: self.amount_step_size,
            price_step_size: self.price_step_size,
            trading_fee_currency: self.trading_fee_currency,
            market_order_enabled: self.market_order_enabled,
            contract_value: self.contract_value,
        }
    }
}
