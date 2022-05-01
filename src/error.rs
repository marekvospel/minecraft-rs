use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("Got an IO Error: {0}")]
  IoError(#[from] io::Error),
  #[error("VarInt is too big")]
  VarIntTooBig(),
}

pub(crate) type Result<T> = std::result::Result<T, Error>;
