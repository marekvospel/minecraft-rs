use crate::game_state::GameState;
use crate::packets::packet::Packet;
use crate::var_int::{VarIntRead, WriteVarInt};
use crate::Result;
use std::io::{BufReader, BufWriter, Cursor, Read, Write};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct HandshakeData {
  protocol_version: i32,
  server_address: String,
  server_port: u16,
  state: GameState,
}

impl HandshakeData {
  #[inline]
  pub fn new<S>(
    protocol_version: i32,
    server_address: S,
    server_port: u16,
    state: GameState,
  ) -> Self
  where
    S: Into<String>,
  {
    HandshakeData {
      protocol_version,
      server_address: server_address.into(),
      server_port,
      state,
    }
  }

  #[inline]
  pub fn protocol_version(&self) -> i32 {
    self.protocol_version
  }

  #[inline]
  pub fn server_address(&self) -> &String {
    &self.server_address
  }

  #[inline]
  pub fn server_port(&self) -> u16 {
    self.server_port
  }

  #[inline]
  pub fn state(&self) -> GameState {
    self.state
  }
}

impl HandshakeData {
  pub fn read<R>(reader: &mut R) -> Result<Self>
  where
    R: Read,
  {
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

impl TryFrom<&Packet> for HandshakeData {
  type Error = crate::error::Error;

  fn try_from(packet: &Packet) -> std::result::Result<Self, Self::Error> {
    let mut reader = BufReader::new(Cursor::new(packet.data()));

    HandshakeData::read(&mut reader)
  }
}

impl TryFrom<&Vec<u8>> for HandshakeData {
  type Error = crate::error::Error;

  fn try_from(packet: &Vec<u8>) -> std::result::Result<Self, Self::Error> {
    let mut reader = BufReader::new(Cursor::new(packet));

    HandshakeData::read(&mut reader)
  }
}

impl HandshakeData {
  pub fn bytes(&self) -> Result<Vec<u8>> {
    let mut bytes = vec![];

    {
      let mut writer = BufWriter::new(&mut bytes);

      writer.write_var_i32(self.protocol_version)?;
      let address = self.server_address.as_bytes();
      writer.write_var_i32(address.len() as i32)?;
      writer.write(address)?;
      writer.write(&self.server_port.to_be_bytes())?;
      writer.write_var_i32(self.state.as_i32())?;
    }

    Ok(bytes)
  }
}
