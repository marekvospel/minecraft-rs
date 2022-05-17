use crate::error::Error;
use crate::packets::packet::Packet;
use crate::var_int::{VarIntRead, WriteVarInt};
use crate::Result;
use std::io::{BufReader, BufWriter, Cursor, Read, Write};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LoginStartData {
  username: String,
}

impl LoginStartData {
  #[inline]
  pub fn new<S>(username: S) -> Self
  where
    S: Into<String>,
  {
    LoginStartData {
      username: username.into(),
    }
  }

  #[inline]
  pub fn username(&self) -> &String {
    &self.username
  }
}

impl LoginStartData {
  pub fn read<R>(reader: &mut R) -> Result<Self>
  where
    R: Read,
  {
    let len = reader.read_var_i32()?;
    let mut buf = vec![0u8; len as usize];
    reader.read_exact(&mut buf)?;
    let username = String::from_utf8_lossy(&buf).to_string();

    Ok(LoginStartData { username })
  }
}

impl TryFrom<&Packet> for LoginStartData {
  type Error = Error;

  fn try_from(value: &Packet) -> std::result::Result<Self, Self::Error> {
    let mut reader = BufReader::new(Cursor::new(value.data()));
    Self::read(&mut reader)
  }
}

impl TryFrom<&Vec<u8>> for LoginStartData {
  type Error = Error;

  fn try_from(value: &Vec<u8>) -> std::result::Result<Self, Self::Error> {
    let mut reader = BufReader::new(Cursor::new(value));
    Self::read(&mut reader)
  }
}

impl LoginStartData {
  pub fn bytes(&self) -> Result<Vec<u8>> {
    let mut data = vec![];

    {
      let mut writer = BufWriter::new(&mut data);

      let bytes = self.username().to_string().into_bytes();
      writer.write_var_i32(bytes.len() as i32)?;
      writer.write(&bytes)?;
    }

    Ok(data)
  }
}
