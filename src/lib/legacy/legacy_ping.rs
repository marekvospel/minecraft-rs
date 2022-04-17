use std::io::{BufReader, Cursor, Error, ErrorKind, Read};
use std::net::TcpStream;

pub struct LegacyPing {
  /// Should always be 0x01
  pub payload: u8,
  /// 0xfa = plugin message identifier
  pub packet_identifier: Option<u8>,
  /// MC|PingHost (encoded as UTF16 string inside tcpstream)
  pub host: Option<String>,
  /// The rest of the data in the ping
  pub data: Option<Vec<u8>>,
}

impl LegacyPing {
  pub fn read(stream: &mut TcpStream) -> Result<LegacyPing, Error> {
    stream.set_nonblocking(true)?;

    let mut buf = [0u8];
    stream.read_exact(&mut buf)?;

    if buf[0] != 0xfe {
      return Err(Error::new(ErrorKind::Other, "Not a valid LegacyPing"));
    }

    stream.read_exact(&mut buf)?;

    let mut legacy_ping = LegacyPing {
      payload: buf[0],
      packet_identifier: None,
      host: None,
      data: None,
    };

    let mut buf = [0u8];

    if let Err(e) = stream.read_exact(&mut buf) {
      stream.set_nonblocking(false)?;
      return if e.kind() == ErrorKind::WouldBlock {
        Ok(legacy_ping)
      } else {
        Err(e)
      };
    }

    legacy_ping.packet_identifier = Some(buf[0]);

    stream.set_nonblocking(false)?;

    let mut buf = [0u8; 2];
    stream.read_exact(&mut buf)?;
    let len = i16::from_be_bytes(buf);

    let mut buf = vec![0u8; (2 * len) as usize];
    stream.read_exact(&mut buf)?;
    let buf = buf.to_u16()?;
    let host = String::from_utf16(&buf).unwrap();

    legacy_ping.host = Some(host);

    let mut buf = [0u8; 2];
    stream.read_exact(&mut buf)?;
    let len = i16::from_be_bytes(buf);

    let mut data = vec![0u8; len as usize];
    stream.read_exact(&mut data)?;

    legacy_ping.data = Some(data);

    Ok(legacy_ping)
  }
}

#[derive(Debug)]
pub struct PingData {
  pub protocol: i8,
  pub hostname: String,
  pub port: i32,
}

impl TryFrom<&LegacyPing> for PingData {
  type Error = Error;

  fn try_from(_ping: &LegacyPing) -> Result<Self, Self::Error> {
    // TODO: parse LegacyPing.data to get payload, hostname and port
    Ok(PingData {
      protocol: 0,
      hostname: "".to_string(),
      port: 0,
    })
  }
}

trait ToU16 {
  fn to_u16(self) -> Result<Vec<u16>, Error>;
}

impl ToU16 for Vec<u8> {
  fn to_u16(self) -> Result<Vec<u16>, Error> {
    let mut output = Vec::new();

    let mut reader = BufReader::new(Cursor::new(&self));

    loop {
      let mut buf = [0u8; 2];
      if reader.read(&mut buf[0..1])? != 1 {
        break;
      }
      if reader.read(&mut buf[1..2])? != 1 {
        // TODO: return error
        break;
      }

      output.push(u16::from_be_bytes(buf));
    }

    Ok(output)
  }
}
