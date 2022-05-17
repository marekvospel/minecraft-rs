use crate::ClientData;
use crate::Result;
use minecraft_rs::game_state::GameState;
use minecraft_rs::packets::login::login_start::LoginStartData;
use minecraft_rs::packets::login::login_success::LoginSuccessData;
use minecraft_rs::packets::login::set_compression::SetCompressionData;
use minecraft_rs::packets::packet::Packet;
use minecraft_rs::packets::play::disconnect::DisconnectData;
use minecraft_rs::var_int::WriteVarInt;
use nbt::Map;
use serde_json::{json, Value};
use std::array::IntoIter;
use std::collections::HashMap;
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

    // Worlds array (only 1)
    writer.write_var_i32(2)?;
    let str = "minecraft:overworld".to_string().into_bytes();
    writer.write_var_i32(str.len() as i32)?;
    writer.write(&str)?;
    let str = "minecraft:the_nether".to_string().into_bytes();
    writer.write_var_i32(str.len() as i32)?;
    writer.write(&str)?;

    let dimension = nbt::Value::Compound(HashMap::from_iter([
      ("piglin_safe".to_string(), nbt::Value::Byte(1)),
      ("natural".to_string(), nbt::Value::Byte(1)),
      ("ambient_light".to_string(), nbt::Value::Float(1.0)),
      (
        "infiniburn".to_string(),
        nbt::Value::String("#minecraft:infiniburn_overworld".to_string()),
      ),
      ("respawn_anchor_works".to_string(), nbt::Value::Byte(1)),
      ("has_skylight".to_string(), nbt::Value::Byte(1)),
      ("bed_works".to_string(), nbt::Value::Byte(1)),
      (
        "effects".to_string(),
        nbt::Value::String("minecraft:overworld".to_string()),
      ),
      ("has_raids".to_string(), nbt::Value::Byte(0)),
      ("min_y".to_string(), nbt::Value::Int(0)),
      ("height".to_string(), nbt::Value::Int(256)),
      ("logical_height".to_string(), nbt::Value::Int(256)),
      ("coordinate_scale".to_string(), nbt::Value::Double(1.0)),
      ("ultrawarm".to_string(), nbt::Value::Byte(0)),
      ("has_ceiling".to_string(), nbt::Value::Byte(0)),
    ]));

    let nbt = nbt::Value::Compound(HashMap::from_iter([
      (
        "minecraft:dimension_type".to_string(),
        nbt::Value::Compound(HashMap::from_iter([
          ("type".to_string(), nbt::Value::String("type".to_string())),
          (
            "value".to_string(),
            nbt::Value::List(vec![nbt::Value::Compound(HashMap::from_iter([
              (
                "name".to_string(),
                nbt::Value::String("minecraft:overworld".to_string()),
              ),
              ("id".to_string(), nbt::Value::Int(0)),
              ("element".to_string(), dimension.clone()),
            ]))]),
          ),
        ])),
      ),
      (
        "minecraft:worldgen/biome".to_string(),
        nbt::Value::Compound(HashMap::from_iter([])),
      ),
    ]));

    if let Err(e) = nbt.to_writer(&mut writer) {
      println!("Hi")
    }

    if let Err(e) = dimension.to_writer(&mut writer) {
      println!("Hi")
    }

    let str = "minecraft:overworld".to_string().into_bytes();

    writer.write_var_i32(str.len() as i32)?;
    writer.write(&str)?;

    writer.write(&0i64.to_be_bytes())?;

    writer.write_var_i32(420)?;
    writer.write_var_i32(2)?;
    writer.write_var_i32(2)?;

    writer.write(&0u8.to_be_bytes())?;
    writer.write(&0u8.to_be_bytes())?;
    writer.write(&0u8.to_be_bytes())?;
    writer.write(&0u8.to_be_bytes())?;

    // https://wiki.vg/Protocol#Join_Game
  }

  // Send Join Game
  let packet = Packet::new(0x26, data, client_data.compression_threshold).into_bytes()?;
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
