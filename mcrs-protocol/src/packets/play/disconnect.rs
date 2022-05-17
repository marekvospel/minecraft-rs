use crate::error::{Error, Result};
use crate::packets::packet::Packet;
use crate::var_int::{VarIntRead, WriteVarInt};
use std::io::{BufReader, BufWriter, Cursor, Read, Write};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DisconnectData {
  reason: serde_json::Value,
}

impl DisconnectData {
  #[inline]
  pub fn new<R>(reason: R) -> Self
  where
    R: Into<serde_json::Value>,
  {
    DisconnectData {
      reason: reason.into(),
    }
  }

  #[inline]
  pub fn reason(&self) -> &serde_json::Value {
    &self.reason
  }

  #[inline]
  pub fn into_reason(self) -> serde_json::Value {
    self.reason
  }
}

impl DisconnectData {
  pub fn read<R>(reader: &mut R) -> Result<Self>
  where
    R: Read,
  {
    let len = reader.read_var_i32()?;
    let mut buf = vec![0u8; len as usize];
    reader.read_exact(&mut buf)?;
    let json = serde_json::from_str(&String::from_utf8_lossy(&buf).to_string())?;

    Ok(DisconnectData { reason: json })
  }
}

impl TryFrom<&Packet> for DisconnectData {
  type Error = Error;

  fn try_from(value: &Packet) -> std::result::Result<Self, Self::Error> {
    let mut writer = BufReader::new(Cursor::new(value.data()));
    DisconnectData::read(&mut writer)
  }
}

impl TryFrom<&Vec<u8>> for DisconnectData {
  type Error = Error;

  fn try_from(value: &Vec<u8>) -> std::result::Result<Self, Self::Error> {
    let mut writer = BufReader::new(Cursor::new(value));
    DisconnectData::read(&mut writer)
  }
}

impl DisconnectData {
  pub fn bytes(self) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();

    {
      let mut writer = BufWriter::new(&mut bytes);

      let reason = self.reason.to_string().into_bytes();

      writer.write_var_i32(reason.len() as i32)?;
      writer.write(&reason)?;
    }

    Ok(bytes)
  }
}
