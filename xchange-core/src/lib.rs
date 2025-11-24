extern crate core;

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

pub mod client;
pub mod currency;
pub mod derivative;
pub mod dto;
pub mod error;
pub mod exchange;
pub mod exchange_specification;
pub mod instrument;
pub mod service;
pub mod utils;
#[derive(Debug)]
pub enum BuildError {
    MissingField(String),
    InvalidData(String),
}

// ---- TimeUnit ----

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TimeUnit {
    Nanoseconds,
    Microseconds,
    Milliseconds,
    Seconds,
    Minutes,
    Hours,
    Days,
}

impl TimeUnit {
    pub fn now(self) -> u64 {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock went backwards");

        match self {
            TimeUnit::Nanoseconds => duration.as_nanos() as u64,
            TimeUnit::Microseconds => duration.as_micros() as u64,
            TimeUnit::Milliseconds => duration.as_millis() as u64,
            TimeUnit::Seconds => duration.as_secs(),
            TimeUnit::Minutes => duration.as_secs() / 60,
            TimeUnit::Hours => duration.as_secs() / 3600,
            TimeUnit::Days => duration.as_secs() / 86400,
        }
    }
}
