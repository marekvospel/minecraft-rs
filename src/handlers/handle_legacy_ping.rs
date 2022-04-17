use crate::ClientData;
use std::io::{BufReader, BufWriter, Cursor, Error, Read, Write};
use std::net::Shutdown::Both;
use std::net::TcpStream;

pub fn handle_legacy_ping(
  stream: &mut TcpStream,
  client_data: &mut ClientData,
) -> Result<(), Error> {
  println!("[0xFE] Received 0xFE in handshaking state");

  let mut buf = vec![0u8];
  stream.read_exact(&mut buf)?;
  let identifier = buf[0];

  // TODO: pre 1.6 clients only send 0xFE

  stream.read_exact(&mut buf)?;
  let payload = buf[0];
  stream.read_exact(&mut buf)?;
  let plugin_message = buf[0];

  let mut buf = [0u8; 2];
  stream.read_exact(&mut buf)?;
  let host_len = i16::from_be_bytes(buf);
  let mut buf = vec![0u8; (2 * host_len) as usize];
  stream.read_exact(&mut buf)?;
  let mut buf = buf.to_u16()?;
  let host = String::from_utf16(&buf).unwrap();

  if identifier != 0xfe || payload != 0x01 || plugin_message != 0xfa || host != "MC|PingHost" {
    stream.shutdown(Both)?;
    // TODO: return error
    return Ok(());
  }

  println!("[0xFE] Received Legacy Ping");

  let mut buf = [0u8; 2];
  stream.read_exact(&mut buf)?;
  let len = i16::from_be_bytes(buf);

  let mut data = vec![0u8; len as usize];
  stream.read_exact(&mut data)?;

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

pub trait ToU16 {
  fn to_u16(self) -> Result<Vec<u16>, Error>;
}

pub trait ToU8 {
  fn to_u8(self) -> Result<Vec<u8>, Error>;
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
