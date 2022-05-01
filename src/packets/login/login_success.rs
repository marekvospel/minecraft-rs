use crate::var_int::WriteVarInt;
use crate::Result;
use std::io::{BufWriter, Write};

// TODO: replace u128 with Uuid
pub struct LoginSuccessData {
  uuid: u128,
  username: String,
}

impl LoginSuccessData {
  pub fn new(uuid: u128, username: String) -> Self {
    LoginSuccessData { uuid, username }
  }

  pub fn to_bytes(self) -> Result<Vec<u8>> {
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
