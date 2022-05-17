use crate::error::Error;
use crate::packets::packet::Packet;
use crate::var_int::{VarIntRead, WriteVarInt};
use crate::Result;
use std::io::{BufReader, BufWriter, Cursor, Read, Write};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct StatusResponse {
  response: serde_json::Value,
}

impl StatusResponse {
  #[inline]
  pub fn new<R>(response: R) -> Self
  where
    R: Into<serde_json::Value>,
  {
    StatusResponse {
      response: response.into(),
    }
  }

  #[inline]
  pub fn response(&self) -> &serde_json::Value {
    &self.response
  }

  #[inline]
  pub fn into_response(self) -> serde_json::Value {
    self.response
  }
}

impl StatusResponse {
  pub fn read<R>(reader: &mut R) -> Result<Self>
  where
    R: Read,
  {
    let len = reader.read_var_i32()?;
    let mut buf = vec![0u8; len as usize];
    reader.read_exact(&mut buf)?;

    let json = serde_json::from_str(&String::from_utf8_lossy(&buf).to_string())?;

    Ok(StatusResponse { response: json })
  }
}

impl TryFrom<&Packet> for StatusResponse {
  type Error = Error;

  fn try_from(value: &Packet) -> std::result::Result<Self, Self::Error> {
    let mut reader = BufReader::new(Cursor::new(value.data()));
    StatusResponse::read(&mut reader)
  }
}

impl TryFrom<&Vec<u8>> for StatusResponse {
  type Error = Error;

  fn try_from(value: &Vec<u8>) -> std::result::Result<Self, Self::Error> {
    let mut reader = BufReader::new(Cursor::new(value));
    StatusResponse::read(&mut reader)
  }
}

impl StatusResponse {
  pub fn bytes(&self) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();

    let response = self.response.to_string();
    let response_bytes = response.as_bytes();

    {
      let mut writer = BufWriter::new(&mut bytes);

      writer.write_var_i32(response_bytes.len() as i32)?;
      writer.write(&response_bytes)?;
    }

    Ok(bytes)
  }
}
