use crate::lib::error::{Error, Result};
use crate::{Packet, VarIntRead};
use std::io::{BufReader, Cursor, Read};

pub struct LoginStartData {
  pub username: String,
}

impl LoginStartData {
  pub fn new(username: String) -> Self {
    LoginStartData { username }
  }

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

impl TryFrom<&mut Packet> for LoginStartData {
  type Error = Error;

  fn try_from(value: &mut Packet) -> std::result::Result<Self, Self::Error> {
    let mut reader = BufReader::new(Cursor::new(&value.data));
    Self::read(&mut reader)
  }
}
