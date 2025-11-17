use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

/// Data object representing a CandleStick
#[derive(Debug, Clone)]
pub struct CandleStick {
    pub open: Decimal,
    pub last: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
    pub quota_volume: Decimal,

    pub vwap: Option<Decimal>,
    pub bid: Option<Decimal>,
    pub bid_size: Option<Decimal>,
    pub ask: Option<Decimal>,
    pub ask_size: Option<Decimal>,

    pub timestamp: DateTime<Utc>, // unix timestamp (ms)
}

impl CandleStick {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        timestamp: DateTime<Utc>,
        open: Decimal,
        last: Decimal,
        high: Decimal,
        low: Decimal,
        close: Decimal,
        volume: Decimal,
        quota_volume: Decimal,
        vwap: Option<Decimal>,
        bid: Option<Decimal>,
        bid_size: Option<Decimal>,
        ask: Option<Decimal>,
        ask_size: Option<Decimal>,
    ) -> Self {
        Self {
            timestamp,
            open,
            last,
            high,
            low,
            close,
            volume,
            quota_volume,
            vwap,
            bid,
            bid_size,
            ask,
            ask_size,
        }
    }

    pub fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    pub fn open(&self) -> &Decimal {
        &self.open
    }

    pub fn last(&self) -> &Decimal {
        &self.last
    }

    pub fn high(&self) -> &Decimal {
        &self.high
    }

    pub fn low(&self) -> &Decimal {
        &self.low
    }

    pub fn close(&self) -> &Decimal {
        &self.close
    }

    pub fn volume(&self) -> &Decimal {
        &self.volume
    }

    pub fn quota_volume(&self) -> &Decimal {
        &self.quota_volume
    }

    pub fn vwap(&self) -> Option<&Decimal> {
        self.vwap.as_ref()
    }

    pub fn bid(&self) -> Option<&Decimal> {
        self.bid.as_ref()
    }

    pub fn bid_size(&self) -> Option<&Decimal> {
        self.bid_size.as_ref()
    }

    pub fn ask(&self) -> Option<&Decimal> {
        self.ask.as_ref()
    }

    pub fn ask_size(&self) -> Option<&Decimal> {
        self.ask_size.as_ref()
    }
}

#[derive(Debug, Default, Clone)]
pub struct Builder {
    timestamp: Option<DateTime<Utc>>,
    open: Option<Decimal>,
    last: Option<Decimal>,
    high: Option<Decimal>,
    low: Option<Decimal>,
    close: Option<Decimal>,
    volume: Option<Decimal>,
    quota_volume: Option<Decimal>,
    vwap: Option<Decimal>,
    bid: Option<Decimal>,
    bid_size: Option<Decimal>,
    ask: Option<Decimal>,
    ask_size: Option<Decimal>,
}

impl Builder {
    pub fn from(candle: &CandleStick) -> Self {
        Self {
            timestamp: Some(candle.timestamp()),
            open: Some(candle.open().clone()),
            last: Some(candle.last().clone()),
            high: Some(candle.high().clone()),
            low: Some(candle.low().clone()),
            close: Some(candle.close().clone()),
            volume: Some(candle.volume().clone()),
            quota_volume: Some(candle.quota_volume().clone()),
            vwap: candle.vwap().cloned(),
            bid: candle.bid().cloned(),
            bid_size: candle.bid_size().cloned(),
            ask: candle.ask().cloned(),
            ask_size: candle.ask_size().cloned(),
        }
    }

    pub fn timestamp(mut self, timestamp: DateTime<Utc>) -> Self {
        self.timestamp = Some(timestamp);
        self
    }

    pub fn open(mut self, open: Decimal) -> Self {
        self.open = Some(open);
        self
    }

    pub fn last(mut self, last: Decimal) -> Self {
        self.last = Some(last);
        self
    }

    pub fn high(mut self, high: Decimal) -> Self {
        self.high = Some(high);
        self
    }

    pub fn low(mut self, low: Decimal) -> Self {
        self.low = Some(low);
        self
    }

    pub fn close(mut self, close: Decimal) -> Self {
        self.close = Some(close);
        self
    }

    pub fn volume(mut self, volume: Decimal) -> Self {
        self.volume = Some(volume);
        self
    }

    pub fn quota_volume(mut self, quota_volume: Decimal) -> Self {
        self.quota_volume = Some(quota_volume);
        self
    }

    pub fn vwap(mut self, vwap: Decimal) -> Self {
        self.vwap = Some(vwap);
        self
    }

    pub fn bid(mut self, bid: Decimal) -> Self {
        self.bid = Some(bid);
        self
    }

    pub fn bid_size(mut self, bid_size: Decimal) -> Self {
        self.bid_size = Some(bid_size);
        self
    }

    pub fn ask(mut self, ask: Decimal) -> Self {
        self.ask = Some(ask);
        self
    }

    pub fn ask_size(mut self, ask_size: Decimal) -> Self {
        self.ask_size = Some(ask_size);
        self
    }

    pub fn build(self) -> CandleStick {
        CandleStick::new(
            self.timestamp.expect("timestamp missing"),
            self.open.expect("open missing"),
            self.last.expect("last missing"),
            self.high.expect("high missing"),
            self.low.expect("low missing"),
            self.close.expect("close missing"),
            self.volume.expect("volume missing"),
            self.quota_volume.expect("quota_volume missing"),
            self.vwap,
            self.bid,
            self.bid_size,
            self.ask,
            self.ask_size,
        )
    }
}
