use crate::dto::marketdata::KlineInterval;
use rust_decimal::Decimal;
use xchange_core::instrument::{Instrument, InstrumentKind};

#[derive(Debug, Clone)]
pub struct BinanceKline {
    pub instrument: InstrumentKind,
    pub interval: KlineInterval,

    pub open_time: i64,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal,

    pub close_time: i64,
    pub quote_asset_volume: Decimal,
    pub number_of_trades: i64,
    pub taker_buy_base_asset_volume: Decimal,
    pub taker_buy_quote_asset_volume: Decimal,
    pub closed: bool,
}

impl BinanceKline {
    /// 平均价格 (low + high) / 2
    pub fn average_price(&self) -> Decimal {
        (self.low + self.high) / Decimal::from(2)
    }

    pub fn to_string(&self) -> String {
        format!(
            "[{}] {} {} O:{} A:{} C:{}",
            self.instrument.symbol(),
            self.open_time, // 你后续可以格式化
            self.interval.code(),
            self.open,
            self.average_price(),
            self.close
        )
    }
}
