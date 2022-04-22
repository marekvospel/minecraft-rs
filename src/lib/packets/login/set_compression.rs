use crate::lib::error::Result;
use crate::lib::var_int::WriteVarInt;
use std::io::BufWriter;

pub struct SetCompressionData {
  compression_threshold: i32,
}

impl SetCompressionData {
  pub fn new(compression_threshold: i32) -> Self {
    SetCompressionData {
      compression_threshold,
    }
  }

  pub fn to_bytes(self) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();

    {
      let mut writer = BufWriter::new(&mut bytes);

      writer.write_var_i32(self.compression_threshold)?;
    }

    Ok(bytes)
  }
}
