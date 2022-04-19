use crate::lib::error::Result;
use crate::Packet;
use std::io::{BufReader, BufWriter, Cursor, Read, Write};

pub struct PingData {
  pub payload: i64,
}

impl TryFrom<&mut Packet> for PingData {
  type Error = std::io::Error;

  fn try_from(packet: &mut Packet) -> std::result::Result<Self, Self::Error> {
    let mut reader = BufReader::new(Cursor::new(&packet.data));

    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf)?;
    let payload = i64::from_be_bytes(buf);

    Ok(PingData { payload })
  }
}

impl PingData {
  pub fn to_bytes(self) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();

    let payload = self.payload.to_be_bytes();

    {
      let mut writer = BufWriter::new(&mut bytes);

      writer.write(&payload)?;
    }

    Ok(bytes)
  }
}
