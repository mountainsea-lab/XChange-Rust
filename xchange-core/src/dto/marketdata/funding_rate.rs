use crate::instrument::{Instrument, InstrumentDTO};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundingRate {
    pub instrument: InstrumentDTO,
    pub funding_rate_1h: Decimal,
    pub funding_rate_8h: Decimal,
    pub funding_rate_date: DateTime<Utc>,
    pub funding_rate_effective_in_minutes: i64,
}

impl FundingRate {
    pub fn new(
        instrument: InstrumentDTO,
        funding_rate_1h: Decimal,
        funding_rate_8h: Decimal,
        funding_rate_date: DateTime<Utc>,
        funding_rate_effective_in_minutes: i64,
    ) -> Self {
        let effective = if funding_rate_effective_in_minutes == 0 {
            Self::calculate_funding_rate_effective_in_minutes(funding_rate_date)
        } else {
            funding_rate_effective_in_minutes
        };

        Self {
            instrument,
            funding_rate_1h,
            funding_rate_8h,
            funding_rate_date,
            funding_rate_effective_in_minutes: effective,
        }
    }

    pub fn calculate_funding_rate_effective_in_minutes(date: DateTime<Utc>) -> i64 {
        let now = Utc::now();
        let duration = now.signed_duration_since(date);
        duration.num_minutes()
    }
}

pub struct FundingRateBuilder {
    instrument: Option<InstrumentDTO>,
    funding_rate_1h: Option<Decimal>,
    funding_rate_8h: Option<Decimal>,
    funding_rate_date: Option<DateTime<Utc>>,
    funding_rate_effective_in_minutes: Option<i64>,
}

impl FundingRateBuilder {
    pub fn new() -> Self {
        Self {
            instrument: None,
            funding_rate_1h: None,
            funding_rate_8h: None,
            funding_rate_date: None,
            funding_rate_effective_in_minutes: None,
        }
    }

    pub fn instrument(mut self, instrument: InstrumentDTO) -> Self {
        self.instrument = Some(instrument);
        self
    }

    pub fn funding_rate_1h(mut self, value: Decimal) -> Self {
        self.funding_rate_1h = Some(value);
        self
    }

    pub fn funding_rate_8h(mut self, value: Decimal) -> Self {
        self.funding_rate_8h = Some(value);
        self
    }

    pub fn funding_rate_date(mut self, value: DateTime<Utc>) -> Self {
        self.funding_rate_date = Some(value);
        self
    }

    pub fn funding_rate_effective_in_minutes(mut self, value: i64) -> Self {
        self.funding_rate_effective_in_minutes = Some(value);
        self
    }

    pub fn build(self) -> FundingRate {
        let instrument = self.instrument.expect("instrument is required");
        let fr1h = self.funding_rate_1h.expect("funding_rate_1h is required");
        let fr8h = self.funding_rate_8h.expect("funding_rate_8h is required");
        let date = self
            .funding_rate_date
            .expect("funding_rate_date is required");

        let effective = match self.funding_rate_effective_in_minutes {
            Some(0) | None => FundingRate::calculate_funding_rate_effective_in_minutes(date),
            Some(v) => v,
        };

        FundingRate {
            instrument,
            funding_rate_1h: fr1h,
            funding_rate_8h: fr8h,
            funding_rate_date: date,
            funding_rate_effective_in_minutes: effective,
        }
    }
}
