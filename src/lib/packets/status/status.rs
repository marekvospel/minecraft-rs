use crate::lib::var_int::WriteVarInt;
use std::io::{BufWriter, Error, Write};

type Result<T> = std::result::Result<T, Error>;

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
