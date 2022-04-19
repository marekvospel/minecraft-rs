use crate::lib::var_int::WriteVarInt;
use crate::{ClientData, GameState, Packet, VarIntRead};
use std::io::{BufReader, BufWriter, Cursor, Error, Read, Write};
use std::net::TcpStream;
use std::str::FromStr;

pub fn handle_login(
  packet: &mut Packet,
  stream: &mut TcpStream,
  client_data: &mut ClientData,
) -> Result<(), Error> {
  if packet.id == 0 {
    println!("[0x00] Received Player Login");
    let mut reader = BufReader::new(Cursor::new(&packet.data));

    let len = reader.read_var_i32()?;
    let mut buf = vec![0u8; len as usize];
    reader.read_exact(&mut buf)?;
    let username = String::from_utf8_lossy(&buf).to_string();

    println!(
      "{} is logging in from {}!",
      username,
      stream.peer_addr()?.ip()
    );

    client_data.state = GameState::Play;

    let mut data = Vec::new();

    {
      let mut writer = BufWriter::new(&mut data);

      // TODO: generate uuid
      let uuid = 0u128;
      writer.write(&uuid.to_be_bytes());

      let username = username.as_bytes();
      writer.write_var_i32(username.len() as i32);
      writer.write(username);
    }

    let packet = Packet::new(2, data, false);
    let packet = packet.to_bytes()?;

    stream.write(&packet);
  }

  Ok(())
}
