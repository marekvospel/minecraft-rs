use crate::error::Result;
use crate::var_int::{VarIntRead, VarIntSize, WriteVarInt};
use std::io::{BufReader, BufWriter, Cursor, Read, Write};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Packet {
  length: i32,
  id: i32,
  data: Vec<u8>,
  #[cfg(feature = "compression")]
  compression_threshold: i32,
}

impl Packet {
  #[cfg(feature = "compression")]
  #[inline]
  pub fn new(id: i32, data: Vec<u8>, compression_threshold: i32) -> Self {
    Packet {
      length: (data.len() + id.var_int_size()) as i32,
      id,
      data,
      compression_threshold,
    }
  }

  #[cfg(not(feature = "compression"))]
  #[inline]
  pub fn new(id: i32, data: Vec<u8>) -> Self {
    Packet {
      length: data.len() as i32,
      id,
      data,
    }
  }

  #[inline]
  pub fn id(&self) -> i32 {
    self.id
  }

  #[inline]
  pub fn length(&self) -> i32 {
    self.length
  }

  #[inline]
  pub fn data(&self) -> &Vec<u8> {
    &self.data
  }

  #[cfg(feature = "compression")]
  #[inline]
  pub fn compression_threshold(&self) -> i32 {
    self.compression_threshold
  }

  // TODO: Setters
}

impl Packet {
  #[cfg(not(feature = "compression"))]
  pub fn read<R>(reader: &mut R) -> Result<Self>
  where
    R: Read,
  {
    let length = reader.read_var_i32()?;
    let id = reader.read_var_i32()?;
    let mut data = vec![0u8; (length as usize) - id.var_int_size()];
    reader.read_exact(&mut data)?;

    Ok(Packet { length, id, data })
  }

  #[cfg(feature = "compression")]
  pub fn read<R>(reader: &mut R, compression_threshold: i32) -> Result<Self>
  where
    R: Read,
  {
    let length = reader.read_var_i32()?;

    if compression_threshold < 0 {
      let id = reader.read_var_i32()?;
      let mut data = vec![0u8; (length as usize) - id.var_int_size()];
      reader.read_exact(&mut data)?;

      Ok(Packet {
        length,
        id,
        data,
        compression_threshold,
      })
    } else {
      let mut data_length = reader.read_var_i32()?;

      let mut data = vec![0u8; (length as usize) - data_length.var_int_size()];
      reader.read_exact(&mut data)?;

      let mut uncompressed = Vec::new();

      if data_length != 0 {
        {
          use flate2::read;
          let mut decoder = read::ZlibDecoder::new(Cursor::new(data));
          decoder.read_to_end(&mut uncompressed)?;
        }
      } else {
        uncompressed = data;
        data_length = length - data_length.var_int_size() as i32;
      }

      let mut reader = BufReader::new(Cursor::new(uncompressed));

      let id = reader.read_var_i32()?;
      let mut data = vec![0u8; data_length as usize - id.var_int_size()];
      reader.read_exact(&mut data)?;

      Ok(Packet {
        id,
        length: data_length,
        data,
        compression_threshold,
      })
    }
  }
}

impl Packet {
  #[cfg(not(feature = "compression"))]
  pub fn bytes(&self) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();

    {
      let mut writer = BufWriter::new(&mut bytes);

      writer.write_var_i32(self.length)?;
      writer.write_var_i32(self.id)?;
      writer.write(&self.data)?;
    }

    Ok(bytes)
  }

  #[cfg(feature = "compression")]
  pub fn bytes(self) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();

    if self.compression_threshold < 0 {
      let mut writer = BufWriter::new(&mut bytes);

      writer.write_var_i32(self.length)?;
      writer.write_var_i32(self.id)?;
      writer.write(&self.data)?;
    } else {
      let mut uncompressed = Vec::new();
      let compressed: Vec<u8>;

      {
        let mut writer = BufWriter::new(&mut uncompressed);

        writer.write_var_i32(self.id)?;
        writer.write(&self.data)?;
      }

      if self.compression_threshold < self.length && self.compression_threshold != 0 {
        use flate2::{write, Compression};
        let mut encoder = write::ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write(&uncompressed)?;
        compressed = encoder.finish()?;
      } else {
        compressed = uncompressed
      }

      {
        let mut writer = BufWriter::new(&mut bytes);

        let len = compressed.len() as i32;

        writer.write_var_i32(len.var_int_size() as i32 + len)?;
        if self.compression_threshold < self.length
          && self.compression_threshold != 0
          && cfg!(feature = "compression")
        {
          writer.write_var_i32(self.length)?;
        } else {
          writer.write_var_i32(0)?;
        }
        writer.write(&compressed)?;
      }
    }

    Ok(bytes)
  }
}
