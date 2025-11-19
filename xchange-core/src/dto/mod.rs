pub mod account;
pub mod loan_order;
pub mod marketdata;
pub mod meta;
pub mod order;
pub mod trade;

#[derive(Debug)]
pub enum BuildError {
    MissingField(String),
    InvalidData(String),
}
