use crate::error::{Error, Result};
use crate::packets::packet::Packet;
use crate::var_int::{VarIntRead, WriteVarInt};
use std::io::{BufReader, BufWriter, Cursor, Read, Write};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ChatData {
  message: serde_json::Value,
  position: i8,
  uuid: u128,
}

impl ChatData {
  #[inline]
  pub fn new<R>(message: R, position: i8, uuid: u128) -> Self
  where
    R: Into<serde_json::Value>,
  {
    ChatData {
      message: message.into(),
      position,
      uuid
    }
  }

  #[inline]
  pub fn message(&self) -> &serde_json::Value {
    &self.message
  }

  #[inline]
  pub fn into_message(self) -> serde_json::Value {
    self.message
  }

  #[inline]
  pub fn position(&self) -> i8 {
    self.position
  }

  #[inline]
  pub fn uuid(&self) -> u128 {
    self.uuid
  }
}

impl ChatData {
  pub fn read<R>(reader: &mut R) -> Result<Self>
  where
    R: Read,
  {
    let len = reader.read_var_i32()?;
    let mut buf = vec![0u8; len as usize];
    reader.read_exact(&mut buf)?;
    let json = serde_json::from_str(&String::from_utf8_lossy(&buf).to_string())?;

    let mut buf = [0u8; 1];
    reader.read_exact(&mut buf)?;
    let position = i8::from_be_bytes(buf);

    let mut buf = [0u8; 16];
    reader.read_exact(&mut buf)?;
    let uuid = u128::from_be_bytes(buf);

    Ok(ChatData {
      message: json,
      position,
      uuid
    })
  }
}

impl TryFrom<&Packet> for ChatData {
  type Error = Error;

  fn try_from(value: &Packet) -> std::result::Result<Self, Self::Error> {
    let mut writer = BufReader::new(Cursor::new(value.data()));
    ChatData::read(&mut writer)
  }
}

impl TryFrom<&Vec<u8>> for ChatData {
  type Error = Error;

  fn try_from(value: &Vec<u8>) -> std::result::Result<Self, Self::Error> {
    let mut writer = BufReader::new(Cursor::new(value));
    ChatData::read(&mut writer)
  }
}

impl ChatData {
  pub fn bytes(self) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();

    {
      let mut writer = BufWriter::new(&mut bytes);

      let reason = self.message.to_string().into_bytes();

      writer.write_var_i32(reason.len() as i32)?;
      writer.write(&reason)?;

      let position = self.position.to_be_bytes();
      writer.write(&position)?;

      let uuid = self.uuid.to_be_bytes();
      writer.write(&uuid)?;
    }

    Ok(bytes)
  }
}
