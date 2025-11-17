use crate::instrument::InstrumentDTO;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticker {
    pub instrument: InstrumentDTO, // 或者根据需求替换为 Instrument 类型
    pub open: Decimal,
    pub last: Decimal,
    pub bid: Decimal,
    pub ask: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub vwap: Decimal,
    pub volume: Option<Decimal>,
    pub quote_volume: Option<Decimal>,
    pub timestamp: Option<DateTime<Utc>>,
    pub bid_size: Decimal,
    pub ask_size: Decimal,
    pub percentage_change: Option<Decimal>,
}

impl Ticker {
    pub fn new(
        instrument: InstrumentDTO,
        open: Decimal,
        last: Decimal,
        bid: Decimal,
        ask: Decimal,
        high: Decimal,
        low: Decimal,
        vwap: Decimal,
        volume: Option<Decimal>,
        quote_volume: Option<Decimal>,
        timestamp: Option<DateTime<Utc>>,
        bid_size: Decimal,
        ask_size: Decimal,
        percentage_change: Option<Decimal>,
    ) -> Self {
        Ticker {
            instrument,
            open,
            last,
            bid,
            ask,
            high,
            low,
            vwap,
            volume,
            quote_volume,
            timestamp,
            bid_size,
            ask_size,
            percentage_change,
        }
    }

    // Getters for the fields
    pub fn get_instrument(&self) -> &InstrumentDTO {
        &self.instrument
    }

    // Deprecate get_currency_pair logic
    #[deprecated]
    pub fn get_currency_pair(&self) -> Option<InstrumentDTO> {
        self.instrument.get_currency_pair()
    }

    pub fn get_open(&self) -> Decimal {
        self.open
    }

    pub fn get_last(&self) -> Decimal {
        self.last
    }

    pub fn get_bid(&self) -> Decimal {
        self.bid
    }

    pub fn get_ask(&self) -> Decimal {
        self.ask
    }

    pub fn get_high(&self) -> Decimal {
        self.high
    }

    pub fn get_low(&self) -> Decimal {
        self.low
    }

    pub fn get_vwap(&self) -> Decimal {
        self.vwap
    }

    // get_volume logic, assuming volume is Optional
    pub fn get_volume(&self) -> Decimal {
        match self.volume {
            Some(vol) => vol,
            None => {
                if let (Some(quote_vol), Some(last)) = (self.quote_volume, self.last.to_f64()) {
                    // Convert `last` to Decimal and divide
                    let last_decimal =
                        Decimal::from_f64(last.to_f64().unwrap_or(0.0)).unwrap_or(Decimal::ZERO);
                    Decimal::from_f64(
                        quote_vol.to_f64().unwrap_or(0.0) / last_decimal.to_f64().unwrap_or(0.0),
                    )
                    .unwrap_or(Decimal::ZERO)
                } else {
                    Decimal::ZERO
                }
            }
        }
    }

    // get_quote_volume logic, assuming quote_volume is Optional
    pub fn get_quote_volume(&self) -> Decimal {
        match self.quote_volume {
            Some(quote_vol) => quote_vol,
            None => {
                if let Some(vol) = self.volume {
                    return vol * self.last;
                }
                Decimal::ZERO
            }
        }
    }

    pub fn get_timestamp(&self) -> Option<DateTime<Utc>> {
        self.timestamp
    }

    pub fn get_bid_size(&self) -> Decimal {
        self.bid_size
    }

    pub fn get_ask_size(&self) -> Decimal {
        self.ask_size
    }

    pub fn get_percentage_change(&self) -> Option<Decimal> {
        self.percentage_change
    }
}

impl fmt::Display for Ticker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Ticker [instrument={:?}, open={}, last={}, bid={}, ask={}, high={}, low={}, avg={}, volume={}, quoteVolume={}, timestamp={}, bidSize={}, askSize={}, percentageChange={}]",
            self.instrument,
            self.open,
            self.last,
            self.bid,
            self.ask,
            self.high,
            self.low,
            self.vwap,
            self.volume
                .as_ref()
                .map_or("None".to_string(), |v| v.to_string()),
            self.quote_volume
                .as_ref()
                .map_or("None".to_string(), |v| v.to_string()),
            self.timestamp
                .as_ref()
                .map_or("None".to_string(), |t| t.to_string()),
            self.bid_size,
            self.ask_size,
            self.percentage_change
                .as_ref()
                .map_or("None".to_string(), |p| p.to_string())
        )
    }
}

#[derive(Default)]
pub struct TickerBuilder {
    instrument: Option<InstrumentDTO>,
    open: Option<Decimal>,
    last: Option<Decimal>,
    bid: Option<Decimal>,
    ask: Option<Decimal>,
    high: Option<Decimal>,
    low: Option<Decimal>,
    vwap: Option<Decimal>,
    volume: Option<Decimal>,
    quote_volume: Option<Decimal>,
    timestamp: Option<DateTime<Utc>>,
    bid_size: Option<Decimal>,
    ask_size: Option<Decimal>,
    percentage_change: Option<Decimal>,
    is_built: bool,
}

impl TickerBuilder {
    // Builder method to set the instrument
    pub fn instrument(mut self, instrument: InstrumentDTO) -> Self {
        self.instrument = Some(instrument);
        self
    }

    // Builder method to set the open value
    pub fn open(mut self, open: Decimal) -> Self {
        self.open = Some(open);
        self
    }

    // Builder method to set the last value
    pub fn last(mut self, last: Decimal) -> Self {
        self.last = Some(last);
        self
    }

    // Builder method to set the bid value
    pub fn bid(mut self, bid: Decimal) -> Self {
        self.bid = Some(bid);
        self
    }

    // Builder method to set the ask value
    pub fn ask(mut self, ask: Decimal) -> Self {
        self.ask = Some(ask);
        self
    }

    // Builder method to set the high value
    pub fn high(mut self, high: Decimal) -> Self {
        self.high = Some(high);
        self
    }

    // Builder method to set the low value
    pub fn low(mut self, low: Decimal) -> Self {
        self.low = Some(low);
        self
    }

    // Builder method to set the vwap value
    pub fn vwap(mut self, vwap: Decimal) -> Self {
        self.vwap = Some(vwap);
        self
    }

    // Builder method to set the volume value
    pub fn volume(mut self, volume: Decimal) -> Self {
        self.volume = Some(volume);
        self
    }

    // Builder method to set the quote volume value
    pub fn quote_volume(mut self, quote_volume: Decimal) -> Self {
        self.quote_volume = Some(quote_volume);
        self
    }

    // Builder method to set the timestamp value
    pub fn timestamp(mut self, timestamp: DateTime<Utc>) -> Self {
        self.timestamp = Some(timestamp);
        self
    }

    // Builder method to set the bid size
    pub fn bid_size(mut self, bid_size: Decimal) -> Self {
        self.bid_size = Some(bid_size);
        self
    }

    // Builder method to set the ask size
    pub fn ask_size(mut self, ask_size: Decimal) -> Self {
        self.ask_size = Some(ask_size);
        self
    }

    // Builder method to set the percentage change
    pub fn percentage_change(mut self, percentage_change: Decimal) -> Self {
        self.percentage_change = Some(percentage_change);
        self
    }

    // Final method to build the Ticker instance
    pub fn build(mut self) -> Result<Ticker, String> {
        if self.is_built {
            return Err("The entity has already been built".to_string());
        }

        // Ensure all required fields are set
        let instrument = self.instrument.ok_or("Instrument is required")?;
        let open = self.open.ok_or("Open value is required")?;
        let last = self.last.ok_or("Last value is required")?;
        let bid = self.bid.ok_or("Bid value is required")?;
        let ask = self.ask.ok_or("Ask value is required")?;
        let high = self.high.ok_or("High value is required")?;
        let low = self.low.ok_or("Low value is required")?;
        let vwap = self.vwap.ok_or("VWAP value is required")?;
        let volume = self.volume;
        let quote_volume = self.quote_volume;
        let timestamp = self.timestamp;
        let bid_size = self.bid_size.ok_or("Bid size is required")?;
        let ask_size = self.ask_size.ok_or("Ask size is required")?;
        let percentage_change = self.percentage_change;

        // After validation, return the Ticker instance
        let ticker = Ticker {
            instrument,
            open,
            last,
            bid,
            ask,
            high,
            low,
            vwap,
            volume,
            quote_volume,
            timestamp,
            bid_size,
            ask_size,
            percentage_change,
        };

        // Mark the builder as built
        self.is_built = true;

        Ok(ticker)
    }
}

impl PartialEq for Ticker {
    fn eq(&self, other: &Self) -> bool {
        self.instrument == other.instrument
            && self.open == other.open
            && self.last == other.last
            && self.bid == other.bid
            && self.ask == other.ask
            && self.high == other.high
            && self.low == other.low
            && self.vwap == other.vwap
            && self.volume == other.volume
            && self.quote_volume == other.quote_volume
            && self.timestamp == other.timestamp
            && self.bid_size == other.bid_size
            && self.ask_size == other.ask_size
            && self.percentage_change == other.percentage_change
    }
}

impl Eq for Ticker {}

impl Hash for Ticker {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.instrument.hash(state);
        self.open.hash(state);
        self.last.hash(state);
        self.bid.hash(state);
        self.ask.hash(state);
        self.high.hash(state);
        self.low.hash(state);
        self.vwap.hash(state);
        self.volume.hash(state);
        self.quote_volume.hash(state);
        self.timestamp.hash(state);
        self.bid_size.hash(state);
        self.ask_size.hash(state);
        self.percentage_change.hash(state);
    }
}
