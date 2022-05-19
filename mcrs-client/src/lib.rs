use mcrs_protocol::error::Error;

pub mod builder;
pub mod callback;
pub mod client;
pub mod events;
pub(crate) mod state;

pub(crate) type Result<T> = std::result::Result<T, Error>;
