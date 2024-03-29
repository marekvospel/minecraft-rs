use crate::error::Error::{IoError, LegacyError};
use crate::Result;
use std::io::{BufReader, BufWriter, Cursor, Error, ErrorKind, Read, Write};
use std::net::TcpStream;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LegacyPing {
  /// Should always be 0x01
  payload: u8,
  /// 0xfa = plugin message identifier
  packet_identifier: Option<u8>,
  /// MC|PingHost (encoded as UTF16 string inside tcpstream)
  host: Option<String>,
  /// The rest of the data in the ping
  data: Option<Vec<u8>>,
}

impl LegacyPing {
  #[inline]
  pub fn new(
    payload: u8,
    packet_identifier: Option<u8>,
    host: Option<String>,
    data: Option<Vec<u8>>,
  ) -> Self {
    LegacyPing {
      payload,
      packet_identifier,
      host,
      data,
    }
  }

  #[inline]
  pub fn payload(&self) -> u8 {
    self.payload
  }

  #[inline]
  pub fn packet_identifier(&self) -> &Option<u8> {
    &self.packet_identifier
  }

  #[inline]
  pub fn host(&self) -> &Option<String> {
    &self.host
  }

  #[inline]
  pub fn data(&self) -> &Option<Vec<u8>> {
    &self.data
  }
}

impl LegacyPing {
  pub fn read(stream: &mut TcpStream) -> Result<LegacyPing> {
    stream.set_nonblocking(true)?;

    let mut buf = [0u8];
    stream.read_exact(&mut buf)?;

    if buf[0] != 0xfe {
      return Err(LegacyError(
        "Only legacy ping (0xfe) is allowed.".to_string(),
      ));
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
        Err(IoError(e))
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

// TODO: LegacyPing.bytes()

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LegacyPong {
  protocol: i32,
  version_name: String,
  motd: String,
  current_players: i128,
  max_players: i128,
}

impl LegacyPong {
  #[inline]
  pub fn new(
    protocol: i32,
    version_name: String,
    motd: String,
    current_players: i128,
    max_players: i128,
  ) -> Self {
    LegacyPong {
      protocol,
      version_name,
      motd,
      current_players,
      max_players,
    }
  }

  #[inline]
  pub fn protocol(&self) -> i32 {
    self.protocol
  }

  #[inline]
  pub fn version_name(&self) -> &String {
    &self.version_name
  }

  #[inline]
  pub fn motd(&self) -> &String {
    &self.motd
  }

  #[inline]
  pub fn current_players(&self) -> i128 {
    self.current_players
  }

  #[inline]
  pub fn max_players(&self) -> i128 {
    self.max_players
  }
}

impl LegacyPong {
  pub fn bytes(self) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();

    {
      let mut writer = BufWriter::new(&mut bytes);

      let response = format!(
        "§1\0{}\0{}\0{}\0{}\0{}",
        self.protocol, self.version_name, self.motd, self.current_players, self.max_players
      );
      let response_bytes = response.encode_utf16().collect::<Vec<u16>>().to_u8()?;
      let len = response.encode_utf16().collect::<Vec<u16>>().len() as i16;
      writer.write(&[0xffu8])?;
      writer.write(&len.to_be_bytes())?;
      writer.write(&response_bytes)?;
    }

    Ok(bytes)
  }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LegacyPingData {
  protocol: i8,
  hostname: String,
  port: i32,
}

impl LegacyPingData {
  #[inline]
  pub fn new<S>(protocol: i8, hostname: S, port: i32) -> Self
  where
    S: Into<String>,
  {
    LegacyPingData {
      protocol,
      hostname: hostname.into(),
      port,
    }
  }

  #[inline]
  pub fn protocol(&self) -> i8 {
    self.protocol
  }

  #[inline]
  pub fn hostname(&self) -> &String {
    &self.hostname
  }

  #[inline]
  pub fn port(&self) -> i32 {
    self.port
  }
}

impl TryFrom<&LegacyPing> for LegacyPingData {
  type Error = Error;

  fn try_from(_ping: &LegacyPing) -> std::result::Result<Self, Self::Error> {
    // TODO: parse LegacyPing.data to get payload, hostname and port
    Ok(LegacyPingData {
      protocol: 0,
      hostname: "".to_string(),
      port: 0,
    })
  }
}

trait ToU16 {
  fn to_u16(self) -> Result<Vec<u16>>;
}

impl ToU16 for Vec<u8> {
  fn to_u16(self) -> Result<Vec<u16>> {
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

trait ToU8 {
  fn to_u8(self) -> Result<Vec<u8>>;
}

impl ToU8 for Vec<u16> {
  fn to_u8(self) -> Result<Vec<u8>> {
    let mut output = Vec::new();

    for i in self {
      let buf = i.to_be_bytes();
      output.push(buf[0]);
      output.push(buf[1]);
    }

    Ok(output)
  }
}
