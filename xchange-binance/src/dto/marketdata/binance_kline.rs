// use chrono::{DateTime, Utc};
// use rust_decimal::Decimal;
// use xchange_core::instrument::Instrument;
//
// #[derive(Debug, Clone)]
// pub struct BinanceKline {
//     pub instrument: Instrument,
//     pub interval: KlineInterval,
//
//     pub open_time: i64,
//     pub open: Decimal,
//     pub high: Decimal,
//     pub low: Decimal,
//     pub close: Decimal,
//     pub volume: Decimal,
//
//     pub close_time: i64,
//     pub quote_asset_volume: Decimal,
//     pub number_of_trades: i64,
//     pub taker_buy_base_asset_volume: Decimal,
//     pub taker_buy_quote_asset_volume: Decimal,
//     pub closed: bool,
// }
//
// impl BinanceKline {
//     /// 平均价格 (low + high) / 2
//     pub fn average_price(&self) -> Decimal {
//         (self.low + self.high) / Decimal::from(2)
//     }
//
//     /// Rust 版 to_string
//     pub fn summary(&self) -> String {
//         let ts = DateTime::<Utc>::from_timestamp_millis(self.open_time)
//             .map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string())
//             .unwrap_or_else(|| self.open_time.to_string());
//
//         format!(
//             "[{}] {} {} O:{} A:{} C:{}",
//             self.instrument,
//             ts,
//             self.interval,
//             self.open,
//             self.average_price(),
//             self.close
//         )
//     }
// }
