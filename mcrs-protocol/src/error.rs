use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("Got an IO Error: {0}")]
  IoError(#[from] io::Error),
  #[error("VarInt is too big")]
  VarIntTooBig(),
  #[error("String is not json serializable: {0}")]
  InvalidJson(#[from] serde_json::Error),
  #[error("Invalid legacy packet: {0}")]
  LegacyError(String),
}

pub(crate) type Result<T> = std::result::Result<T, Error>;
