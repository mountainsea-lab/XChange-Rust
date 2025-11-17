use crate::instrument::InstrumentDTO;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use std::fmt;

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
