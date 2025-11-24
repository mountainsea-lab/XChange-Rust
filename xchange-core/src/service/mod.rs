pub mod account;
pub mod marketdata;
pub mod trade;

pub trait BaseService: Send + Sync {}
