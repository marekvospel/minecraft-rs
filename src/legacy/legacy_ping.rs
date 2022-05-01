use std::io::{BufReader, BufWriter, Cursor, Error, ErrorKind, Read, Write};
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

pub struct LegacyPong {
  pub protocol: i32,
  pub version_name: String,
  pub motd: String,
  pub current_players: i128,
  pub max_players: i128,
}

impl LegacyPong {
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

  pub fn to_bytes(self) -> Result<Vec<u8>, Error> {
    let mut bytes = Vec::new();

    {
      let mut writer = BufWriter::new(Cursor::new(&mut bytes));

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

#[derive(Debug)]
pub struct LegacyPingData {
  pub protocol: i8,
  pub hostname: String,
  pub port: i32,
}

impl TryFrom<&LegacyPing> for LegacyPingData {
  type Error = Error;

  fn try_from(_ping: &LegacyPing) -> Result<Self, Self::Error> {
    // TODO: parse LegacyPing.data to get payload, hostname and port
    Ok(LegacyPingData {
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

trait ToU8 {
  fn to_u8(self) -> Result<Vec<u8>, Error>;
}

impl ToU8 for Vec<u16> {
  fn to_u8(self) -> Result<Vec<u8>, Error> {
    let mut output = Vec::new();

    for i in self {
      let buf = i.to_be_bytes();
      output.push(buf[0]);
      output.push(buf[1]);
    }

    Ok(output)
  }
}
