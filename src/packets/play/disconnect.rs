use crate::error::Result;
use crate::var_int::WriteVarInt;
use std::io::{BufWriter, Write};

pub struct DisconnectData {
  pub reason: serde_json::Value,
}

impl DisconnectData {
  pub fn new(reason: serde_json::Value) -> Self {
    DisconnectData { reason }
  }

  pub fn to_bytes(self) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();

    {
      let mut writer = BufWriter::new(&mut bytes);

      let reason = self.reason.to_string().into_bytes();

      writer.write_var_i32(reason.len() as i32)?;
      writer.write(&reason)?;
    }

    Ok(bytes)
  }
}
