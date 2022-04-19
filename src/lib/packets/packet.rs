use crate::lib::error::Result;
use crate::lib::var_int::WriteVarInt;
use crate::{VarIntRead, VarIntSize};
use std::io::{BufWriter, Read, Write};

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
