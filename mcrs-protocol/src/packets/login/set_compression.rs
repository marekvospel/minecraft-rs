use crate::error::{Error, Result};
use crate::packets::packet::Packet;
use crate::var_int::{VarIntRead, WriteVarInt};
use std::io::{BufReader, BufWriter, Cursor, Read};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct SetCompressionData {
  compression_threshold: i32,
}

impl SetCompressionData {
  #[inline]
  pub fn new(compression_threshold: i32) -> Self {
    SetCompressionData {
      compression_threshold,
    }
  }

  #[inline]
  pub fn compression_threshold(&self) -> i32 {
    self.compression_threshold
  }
}

impl SetCompressionData {
  pub fn read<R>(reader: &mut R) -> Result<Self>
  where
    R: Read,
  {
    let threshold = reader.read_var_i32()?;

    Ok(SetCompressionData {
      compression_threshold: threshold,
    })
  }
}

impl TryFrom<&Packet> for SetCompressionData {
  type Error = Error;

  fn try_from(value: &Packet) -> std::result::Result<Self, Self::Error> {
    let mut reader = BufReader::new(Cursor::new(value.data()));
    SetCompressionData::read(&mut reader)
  }
}

impl TryFrom<&Vec<u8>> for SetCompressionData {
  type Error = Error;

  fn try_from(value: &Vec<u8>) -> std::result::Result<Self, Self::Error> {
    let mut reader = BufReader::new(Cursor::new(value));
    SetCompressionData::read(&mut reader)
  }
}

impl SetCompressionData {
  pub fn bytes(self) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();

    {
      let mut writer = BufWriter::new(&mut bytes);

      writer.write_var_i32(self.compression_threshold)?;
    }

    Ok(bytes)
  }
}
