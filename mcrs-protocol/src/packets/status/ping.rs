use crate::error::{Error, Result};
use crate::packets::packet::Packet;
use std::io::{BufReader, BufWriter, Cursor, Read, Write};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct PingData {
  payload: i64,
}

impl PingData {
  #[inline]
  pub fn new(payload: i64) -> Self {
    PingData { payload }
  }

  #[inline]
  pub fn payload(&self) -> i64 {
    self.payload
  }
}

impl PingData {
  pub fn read<R>(reader: &mut R) -> Result<Self>
  where
    R: Read,
  {
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf)?;
    let payload = i64::from_be_bytes(buf);

    Ok(PingData { payload })
  }
}

impl TryFrom<&Packet> for PingData {
  type Error = Error;

  fn try_from(packet: &Packet) -> std::result::Result<Self, Self::Error> {
    let mut reader = BufReader::new(Cursor::new(packet.data()));

    PingData::read(&mut reader)
  }
}

impl TryFrom<&Vec<u8>> for PingData {
  type Error = Error;

  fn try_from(data: &Vec<u8>) -> std::result::Result<Self, Self::Error> {
    let mut reader = BufReader::new(Cursor::new(data));

    PingData::read(&mut reader)
  }
}

impl PingData {
  pub fn bytes(&self) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();

    let payload = self.payload.to_be_bytes();

    {
      let mut writer = BufWriter::new(&mut bytes);

      writer.write(&payload)?;
    }

    Ok(bytes)
  }
}
