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
    // 字段索引常量（对应 Binance API Kline 返回顺序）
    const IDX_OPEN_TIME: usize = 0;
    const IDX_OPEN: usize = 1;
    const IDX_HIGH: usize = 2;
    const IDX_LOW: usize = 3;
    const IDX_CLOSE: usize = 4;
    const IDX_VOLUME: usize = 5;
    const IDX_CLOSE_TIME: usize = 6;
    const IDX_QUOTE_ASSET_VOLUME: usize = 7;
    const IDX_NUMBER_OF_TRADES: usize = 8;
    const IDX_TAKER_BUY_BASE: usize = 9;
    const IDX_TAKER_BUY_QUOTE: usize = 10;
    const IDX_CLOSED: usize = 11;

    pub fn new(
        instrument: &InstrumentKind,
        interval: &KlineInterval,
        raw: &[serde_json::Value],
    ) -> Self {
        fn parse_decimal(raw: &[serde_json::Value], idx: usize) -> Decimal {
            raw.get(idx)
                .and_then(|v| v.as_str())
                .and_then(|s| s.parse::<Decimal>().ok())
                .unwrap_or(Decimal::ZERO)
        }

        fn parse_i64(raw: &[serde_json::Value], idx: usize) -> i64 {
            raw.get(idx).and_then(|v| v.as_i64()).unwrap_or(0)
        }

        fn parse_bool(raw: &[serde_json::Value], idx: usize) -> bool {
            raw.get(idx).and_then(|v| v.as_bool()).unwrap_or(true)
        }

        Self {
            instrument: instrument.clone(), // InstrumentKind 必须实现 Clone
            interval: interval.clone(),     // KlineInterval 必须实现 Clone
            open_time: parse_i64(raw, Self::IDX_OPEN_TIME),
            open: parse_decimal(raw, Self::IDX_OPEN),
            high: parse_decimal(raw, Self::IDX_HIGH),
            low: parse_decimal(raw, Self::IDX_LOW),
            close: parse_decimal(raw, Self::IDX_CLOSE),
            volume: parse_decimal(raw, Self::IDX_VOLUME),
            close_time: parse_i64(raw, Self::IDX_CLOSE_TIME),
            quote_asset_volume: parse_decimal(raw, Self::IDX_QUOTE_ASSET_VOLUME),
            number_of_trades: parse_i64(raw, Self::IDX_NUMBER_OF_TRADES),
            taker_buy_base_asset_volume: parse_decimal(raw, Self::IDX_TAKER_BUY_BASE),
            taker_buy_quote_asset_volume: parse_decimal(raw, Self::IDX_TAKER_BUY_QUOTE),
            closed: parse_bool(raw, Self::IDX_CLOSED),
        }
    }

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
