use crate::{GameState, Packet, VarIntRead};
use std::io::{BufReader, Cursor, Read};

pub struct HandshakeData {
  pub protocol_version: i32,
  pub server_address: String,
  pub server_port: u16,
  pub state: GameState,
}

impl TryFrom<&mut Packet> for HandshakeData {
  type Error = crate::lib::error::Error;

  fn try_from(packet: &mut Packet) -> std::result::Result<Self, Self::Error> {
    let mut reader = BufReader::new(Cursor::new(&packet.data));

    let protocol_version = reader.read_var_i32()?;
    let buf = reader.read_var_i32()?;
    let mut buf = vec![0u8; buf as usize];
    reader.read_exact(&mut buf)?;
    let server_address = String::from(String::from_utf8_lossy(&buf));
    let mut buf = [0u8, 0u8];
    reader.read_exact(&mut buf)?;
    let server_port = u16::from_be_bytes(buf);
    let next = reader.read_var_i32()?;

    Ok(HandshakeData {
      protocol_version,
      server_address,
      server_port,
      state: GameState::from(next),
    })
  }
}
