use crate::lib::var_int::WriteVarInt;
use crate::{VarIntRead, VarIntSize};
use std::io::{BufReader, BufWriter, Cursor, Error, Read, Write};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Packet {
  pub length: i32,
  pub id: i32,
  pub data: Vec<u8>,
  pub compressed: bool,
}

fn read_packet_uncompressed<R>(reader: &mut R) -> Result<Packet>
where
  R: Read,
{
  let length = reader.read_var_i32()?;
  let id = reader.read_var_i32()?;
  let mut data = vec![0u8; (length as usize) - id.var_int_size()];
  reader.read_exact(&mut data)?;

  Ok(Packet {
    length,
    id,
    data,
    compressed: false,
  })
}

impl Packet {
  pub fn new(id: i32, data: Vec<u8>, compressed: bool) -> Self {
    Packet {
      length: (data.len() + id.var_int_size()) as i32,
      id,
      data,
      compressed,
    }
  }

  pub fn read<R>(reader: &mut R, compressed: bool) -> Result<Self>
  where
    R: Read,
  {
    if !compressed {
      return read_packet_uncompressed(reader);
    }

    todo!()
  }

  pub fn to_bytes(self) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();

    if !self.compressed {
      let mut writer = BufWriter::new(&mut bytes);

      writer.write_var_i32(self.length)?;
      writer.write_var_i32(self.id)?;
      writer.write(&self.data)?;
    }

    Ok(bytes)
  }
}

#[derive(Debug, Eq, PartialEq)]
pub enum GameState {
  Handshaking = 0,
  Status = 1,
  Login = 2,
  Play = 3,
}

impl From<i32> for GameState {
  fn from(value: i32) -> Self {
    use GameState::*;
    match value {
      0 => Handshaking,
      1 => Status,
      2 => Login,
      3 => Play,
      _ => Handshaking,
    }
  }
}

pub struct HandshakeData {
  pub protocol_version: i32,
  pub server_address: String,
  pub server_port: u16,
  pub state: GameState,
}

impl TryFrom<&mut Packet> for HandshakeData {
  type Error = std::io::Error;

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

pub struct StatusResponse {
  response: serde_json::Value,
}

impl StatusResponse {
  pub fn new(response: &serde_json::Value) -> Self {
    StatusResponse {
      response: response.clone(),
    }
  }

  pub fn to_bytes(self) -> Result<Vec<u8>> {
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
