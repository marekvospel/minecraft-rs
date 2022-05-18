use mcrs_protocol::error::Error;

pub mod client;

pub(crate) type Result<T> = std::result::Result<T, Error>;
