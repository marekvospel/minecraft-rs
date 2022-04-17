use crate::lib::legacy::legacy_ping::{LegacyPing, PingData};
use std::io::{BufWriter, Cursor, Error, Write};
use std::net::Shutdown::Both;
use std::net::TcpStream;

pub fn handle_legacy_ping(stream: &mut TcpStream) -> Result<(), Error> {
  println!("[0xFE] Received 0xFE in handshaking state");

  let ping = LegacyPing::read(stream)?;
  println!("[0xFE] Received Legacy Ping");

  let data: Option<PingData>;

  if let Some(_) = ping.data {
    data = Some(PingData::try_from(&ping)?);
  } else {
    data = None;
  }

  println!(
    "{} {:#?} {:#?} {:#?} {:?}",
    ping.payload, ping.packet_identifier, ping.host, ping.data, data
  );

  /*
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

  let mut packet = Vec::new();

  {
    let mut writer = BufWriter::new(Cursor::new(&mut packet));

    let response = format!(
      "§1\0{}\0{}\0{}\0{}\0{}",
      127, "minecraft-rs", "Hello", 69, 420
    );
    let response_bytes = response.encode_utf16().collect::<Vec<u16>>().to_u8()?;
    let len = response.encode_utf16().collect::<Vec<u16>>().len() as i16;
    writer.write(&[0xffu8])?;
    writer.write(&len.to_be_bytes())?;
    writer.write(&response_bytes)?;
  }

  println!("[0xFE] Sending Legacy Ping Response");
  // TODO: gather more info about modern clients reading the legacy ping response
  stream.write(&packet)?;

  stream.shutdown(Both)?;

  Ok(())
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
