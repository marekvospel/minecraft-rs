use mcrs_protocol::legacy::legacy_ping::{LegacyPing, LegacyPingData, LegacyPong};
use std::io::{Error, Write};
use std::net::Shutdown::Both;
use std::net::TcpStream;

pub fn handle_legacy_ping(stream: &mut TcpStream) -> Result<(), Error> {
  println!("[0xFE] Received 0xFE in handshaking state");

  let ping = LegacyPing::read(stream)?;
  println!("[0xFE] Received Legacy Ping");

  let _data: Option<LegacyPingData>;

  if let Some(_) = ping.data {
    // data = Some(PingData::try_from(&ping)?);
    _data = None
  } else {
    _data = None;
  }

  /*
  TODO: move into LegacyPingData impl
  let mut reader = BufReader::new(Cursor::new(&data));

  let mut buf = [0u8];
  reader.read_exact(&mut buf)?;
  let protocol = buf[0];

  let mut buf = [0u8; 2];
  reader.read_exact(&mut buf)?;
  let len = i16::from_be_bytes(buf);

  let mut buf = vec![0u8; (len * 2) as usize];
  reader.read_exact(&mut buf)?;
  let mut buf = buf.to_u16()?;
  let hostname = String::from_utf16(&buf).unwrap();
  let mut buf = [0u8; 4];
  reader.read_exact(&mut buf)?;
  let port = i32::from_be_bytes(buf);
  */

  let packet = LegacyPong::new(
    758,
    "minecraft-rs".to_string(),
    "Hello".to_string(),
    69,
    420,
  )
  .to_bytes()?;

  println!("[0xFE] Sending Legacy Ping Response");
  // TODO: gather more info about modern clients reading the legacy ping response
  stream.write(&packet)?;

  stream.shutdown(Both)?;

  Ok(())
}
