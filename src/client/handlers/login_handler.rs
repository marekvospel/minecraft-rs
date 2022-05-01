use crate::ClientData;
use crate::Result;
use minecraft_rs::game_state::GameState;
use minecraft_rs::packets::login::login_start::LoginStartData;
use minecraft_rs::packets::login::login_success::LoginSuccessData;
use minecraft_rs::packets::login::set_compression::SetCompressionData;
use minecraft_rs::packets::packet::Packet;
use minecraft_rs::packets::play::disconnect::DisconnectData;
use minecraft_rs::var_int::WriteVarInt;
use serde_json::json;
use std::io::{BufWriter, Cursor, Write};
use std::net::TcpStream;

pub(crate) fn handle_login(
  packet: &mut Packet,
  stream: &mut TcpStream,
  client_data: &mut ClientData,
) -> Result<()> {
  match packet.id {
    0 => {
      println!("[0x00] Received Player Login");

      let login = LoginStartData::try_from(packet)?;

      println!(
        "{} is logging in from {}!",
        login.username,
        stream.peer_addr()?.ip()
      );

      // TODO: start keep alive loop
      client_data.state = GameState::Play;

      // Set compression
      let data = SetCompressionData::new(-1).to_bytes()?;
      let new_packet = Packet::new(0x03, data, client_data.compression_threshold);
      client_data.compression_threshold = -1;
      stream.write(&new_packet.into_bytes()?)?;

      // Send success
      let data = LoginSuccessData::new(0, login.username);
      let new_packet = Packet::new(2, data.to_bytes()?, client_data.compression_threshold);
      stream.write(&new_packet.into_bytes()?)?;

      // let data = DisconnectData::new(json!({
      // "text": "I use Arch btw."
      // }));

      // let packet = Packet::new(0x1a, data.to_bytes()?, client_data.compression_threshold);
      // stream.write(&packet.into_bytes()?)?;

      send_login_data(stream, client_data)?;

      Ok(())
    }
    _ => Ok(()),
  }
}

fn send_login_data(stream: &mut TcpStream, client_data: &ClientData) -> Result<()> {
  let mut data = vec![];

  {
    let mut writer = BufWriter::new(Cursor::new(&mut data));

    writer.write(&0i32.to_be_bytes())?;
    writer.write(&[0u8])?;
    writer.write(&0u8.to_be_bytes())?;
    writer.write(&0u8.to_be_bytes())?;
    writer.write_var_i32(1);
    let str = "minecraft:world".into_string().as_bytes();
    writer.write_var_i32(str.len() as i32);
    writer.write(&str);
    // TODO: add rest of Join Game fields
    // https://wiki.vg/Protocol#Join_Game
  }

  // Send Join Game
  let packet = Packet::new(0x4B, data, client_data.compression_threshold).into_bytes()?;
  stream.write(&packet)?;

  let mut data = vec![];

  {
    let mut writer = BufWriter::new(Cursor::new(&mut data));

    let position =
      (((0i64 & 0x3FFFFFF) << 38) | ((0i64 & 0x3FFFFFF) << 12) | (0i64 & 0xFFF)) as u64;

    writer.write(&position.to_be_bytes())?;
    writer.write(&0f32.to_be_bytes())?;
  }

  // Send Spawn Position
  let packet = Packet::new(0x4B, data, client_data.compression_threshold).into_bytes()?;
  stream.write(&packet)?;

  let mut data = vec![];

  {
    let mut writer = BufWriter::new(Cursor::new(&mut data));

    writer.write_var_i32(0)?;
    writer.write(&0i64.to_be_bytes())?;
    writer.write(&0i64.to_be_bytes())?;
    writer.write(&0i64.to_be_bytes())?;
    writer.write(&[0u8])?;
  }

  // Send Player Position And Look
  let packet = Packet::new(0x37, data, client_data.compression_threshold).into_bytes()?;
  stream.write(&packet)?;

  Ok(())
}
