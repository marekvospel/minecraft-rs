use mcrs_protocol::error::Error;

pub mod builder;
pub mod callback;
pub mod events;
pub mod server;

pub(crate) type Result<T> = std::result::Result<T, Error>;
