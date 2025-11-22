extern crate core;

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
