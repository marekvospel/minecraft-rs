use crate::error::Error;
use crate::packets::packet::Packet;
use crate::var_int::{VarIntRead, WriteVarInt};
use crate::Result;
use std::io::{BufReader, BufWriter, Cursor, Read, Write};

// TODO: replace u128 with Uuid
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LoginSuccessData {
  uuid: u128,
  username: String,
}

impl LoginSuccessData {
  #[inline]
  pub fn new<S>(uuid: u128, username: S) -> Self
  where
    S: Into<String>,
  {
    LoginSuccessData {
      uuid,
      username: username.into(),
    }
  }

  #[inline]
  pub fn uuid(&self) -> u128 {
    self.uuid
  }

  #[inline]
  pub fn username(&self) -> &String {
    &self.username
  }
}

impl LoginSuccessData {
  pub fn read<R>(reader: &mut R) -> Result<Self>
  where
    R: Read,
  {
    let mut buf = [0u8; 16];
    reader.read_exact(&mut buf)?;
    let uuid = u128::from_be_bytes(buf);

    let len = reader.read_var_i32()?;
    let mut buf = vec![0u8; len as usize];
    reader.read_exact(&mut buf)?;
    let username = String::from_utf8_lossy(&buf).to_string();

    Ok(LoginSuccessData { uuid, username })
  }
}

impl TryFrom<&Packet> for LoginSuccessData {
  type Error = Error;

  fn try_from(value: &Packet) -> std::result::Result<Self, Self::Error> {
    let mut reader = BufReader::new(Cursor::new(value.data()));
    LoginSuccessData::read(&mut reader)
  }
}

impl TryFrom<&Vec<u8>> for LoginSuccessData {
  type Error = Error;

  fn try_from(value: &Vec<u8>) -> std::result::Result<Self, Self::Error> {
    let mut reader = BufReader::new(Cursor::new(value));
    LoginSuccessData::read(&mut reader)
  }
}

impl LoginSuccessData {
  pub fn bytes(self) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();

    {
      let mut writer = BufWriter::new(&mut bytes);

      writer.write(&self.uuid.to_be_bytes())?;

      let username = self.username.as_bytes();
      writer.write_var_i32(username.len() as i32)?;
      writer.write(username)?;
    }

    Ok(bytes)
  }
}
