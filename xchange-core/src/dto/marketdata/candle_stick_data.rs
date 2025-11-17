use crate::dto::marketdata::candle_stick::CandleStick;
use crate::instrument::Instrument;
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize)]
pub struct CandleStickData {
    #[serde(skip_serializing)]
    instrument: Arc<dyn Instrument>,
    candle_sticks: Vec<CandleStick>,
}

impl CandleStickData {
    pub fn new(instrument: Arc<dyn Instrument>, candle_sticks: Vec<CandleStick>) -> Self {
        Self {
            instrument,
            candle_sticks: candle_sticks,
        }
    }

    pub fn instrument(&self) -> Arc<dyn Instrument> {
        Arc::clone(&self.instrument)
    }

    pub fn candle_sticks(&self) -> Vec<CandleStick> {
        self.candle_sticks.clone()
    }
}
