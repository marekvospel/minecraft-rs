use std::io::{Error, Read, Write};

/*
 Inspired by
 https://github.com/luojia65/mc-varint
*/

// SEGMENT_BITS = 0x7F; 0b01111111
// CONTINUE_BIT = 0x80; 0b10000000
const SEGMENTS_BITS: u8 = 0b01111111;
const CONTINUE_BIT: u8 = 0b10000000;

type Result<T> = std::result::Result<T, Error>;

/*
 VarInt Size
*/

pub trait VarIntSize {
  fn var_int_size(self) -> usize;
}

impl VarIntSize for i32 {
  fn var_int_size(self) -> usize {
    if self == 0 {
      return 1;
    }

    let mut temp = self;
    let mut size = 0;

    for i in 0..5 {
      if temp == 0 {
        break;
      }

      let bits = temp.to_le_bytes()[0];

      if (bits & SEGMENTS_BITS) != 0 {
        size = i + 1
      }

      temp >>= 7
    }

    size
  }
}

impl VarIntSize for i64 {
  fn var_int_size(self) -> usize {
    if self == 0 {
      return 1;
    }

    let mut temp = self;
    let mut size = 0;

    for i in 0..10 {
      if temp == 0 {
        break;
      }

      let bits = temp.to_le_bytes()[0];

      if (bits & SEGMENTS_BITS) != 0 {
        size = i + 1
      }

      temp >>= 7
    }

    size
  }
}

/*
 Reading
*/
pub trait VarIntRead {
  fn read_var_i32(&mut self) -> Result<i32>;
  fn read_var_i64(&mut self) -> Result<i64>;
}

impl<R> VarIntRead for R
where
  R: Read,
{
  fn read_var_i32(&mut self) -> Result<i32> {
    let mut buf = [0u8];
    let mut value = 0;

    for i in 0..5 {
      self.read_exact(&mut buf)?;

      value |= ((buf[0] & SEGMENTS_BITS) as i32) << 7 * i;
      if (buf[0] & CONTINUE_BIT) == 0 {
        break;
      } else if i == 5 {
        // TODO: throw error
      }
    }

    Ok(value)
  }

  fn read_var_i64(&mut self) -> Result<i64> {
    let mut buf = [0u8];
    let mut value = 0;

    for i in 0..10 {
      self.read_exact(&mut buf)?;

      value |= ((buf[0] & SEGMENTS_BITS) as i64) << 7 * i;
      if (buf[0] & CONTINUE_BIT) == 0 {
        break;
      } else if i == 10 {
        // TODO: throw error
      }
    }

    Ok(value)
  }
}

pub trait WriteVarInt {
  fn write_var_i32(&mut self, value: i32) -> Result<usize>;
  fn write_var_i64(&mut self, value: i64) -> Result<usize>;
}

impl<W> WriteVarInt for W
where
  W: Write,
{
  fn write_var_i32(&mut self, mut value: i32) -> Result<usize> {
    if value == 0 {
      self.write(&mut [0u8])?;
      return Ok(1);
    }

    let mut buf = [0];
    let mut cnt = 0;
    while value != 0 {
      buf[0] = (value & 0b0111_1111) as u8;
      value = (value >> 7) & (i32::MAX >> 6);
      if value != 0 {
        buf[0] |= 0b1000_0000;
      }
      cnt += self.write(&mut buf)?;
    }

    Ok(cnt)
  }

  fn write_var_i64(&mut self, mut value: i64) -> Result<usize> {
    if value == 0 {
      self.write(&mut [0u8])?;
      return Ok(1);
    }

    let mut buf = [0];
    let mut cnt = 0;
    while value != 0 {
      buf[0] = (value & 0b0111_1111) as u8;
      value = (value >> 7) & (i64::MAX >> 6);
      if value != 0 {
        buf[0] |= 0b1000_0000;
      }
      cnt += self.write(&mut buf)?;
    }

    Ok(cnt)
  }
}
