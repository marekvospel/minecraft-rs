use crate::lib::error::Result;
use crate::lib::var_int::WriteVarInt;
use crate::{VarIntRead, VarIntSize};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::{BufReader, BufWriter, Cursor, Read, Write};

#[derive(Debug)]
pub struct Packet {
  pub length: i32,
  pub id: i32,
  pub data: Vec<u8>,
  pub compression_threshold: i32,
}

impl Packet {
  pub fn new(id: i32, data: Vec<u8>, compression_threshold: i32) -> Self {
    Packet {
      length: (data.len() + id.var_int_size()) as i32,
      id,
      data,
      compression_threshold,
    }
  }

  pub fn read<R>(reader: &mut R, compression_threshold: i32) -> Result<Self>
  where
    R: Read,
  {
    let length = reader.read_var_i32()?;

    return if compression_threshold < 0 {
      Self::read_packet_uncompressed(reader, length, compression_threshold)
    } else {
      Self::read_packet_compressed(reader, length, compression_threshold)
    };
  }

  fn read_packet_uncompressed<R>(
    reader: &mut R,
    length: i32,
    compression_threshold: i32,
  ) -> Result<Self>
  where
    R: Read,
  {
    let id = reader.read_var_i32()?;
    let mut data = vec![0u8; (length as usize) - id.var_int_size()];
    reader.read_exact(&mut data)?;

    Ok(Packet {
      length,
      id,
      data,
      compression_threshold,
    })
  }

  fn read_packet_compressed<R>(
    reader: &mut R,
    length: i32,
    compression_threshold: i32,
  ) -> Result<Self>
  where
    R: Read,
  {
    let mut data_length = reader.read_var_i32()?;

    let mut data = vec![0u8; (length as usize) - data_length.var_int_size()];
    reader.read_exact(&mut data)?;

    let mut uncompressed = Vec::new();

    if data_length != 0 {
      let mut decoder = ZlibDecoder::new(Cursor::new(data));
      decoder.read_to_end(&mut uncompressed)?;
    } else {
      uncompressed = data;
      data_length = length - data_length.var_int_size() as i32;
    }

    let mut reader = BufReader::new(Cursor::new(uncompressed));

    let id = reader.read_var_i32()?;
    let mut data = vec![0u8; data_length as usize - id.var_int_size()];
    reader.read_exact(&mut data)?;

    Ok(Packet {
      length: data_length,
      id,
      data,
      compression_threshold,
    })
  }

  pub fn into_bytes(self) -> Result<Vec<u8>> {
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
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write(&uncompressed)?;
        compressed = encoder.finish()?;
      } else {
        compressed = uncompressed
      }

      {
        let mut writer = BufWriter::new(&mut bytes);

        let len = compressed.len() as i32;

        writer.write_var_i32(len.var_int_size() as i32 + len)?;
        if self.compression_threshold < self.length && self.compression_threshold != 0 {
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
