use crate::lib::error::Result;
use crate::lib::packets::login::login_start::LoginStartData;
use crate::lib::packets::login::login_success::LoginSuccessData;
use crate::lib::packets::play::disconnect::DisconnectData;
use crate::{ClientData, GameState, Packet};
use serde_json::json;
use std::io::Write;
use std::net::TcpStream;

pub fn handle_login(
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

      let data = LoginSuccessData::new(0, login.username);
      let packet = Packet::new(2, data.to_bytes()?, false);
      stream.write(&packet.to_bytes()?)?;

      let data = DisconnectData::new(json!({
        "text": "I use Arch btw."
      }));

      let packet = Packet::new(0x1a, data.to_bytes()?, false);
      stream.write(&packet.to_bytes()?)?;

      Ok(())
    }
    _ => Ok(()),
  }
}
