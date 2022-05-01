use crate::ClientData;
use crate::Result;
use minecraft_rs::game_state::GameState;
use minecraft_rs::packets::login::login_start::LoginStartData;
use minecraft_rs::packets::login::login_success::LoginSuccessData;
use minecraft_rs::packets::login::set_compression::SetCompressionData;
use minecraft_rs::packets::packet::Packet;
use std::io::Write;
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
      let data = SetCompressionData::new(1).to_bytes()?;
      let packet = Packet::new(0x03, data, client_data.compression_threshold);
      client_data.compression_threshold = 1;
      stream.write(&packet.into_bytes()?)?;

      // Send success
      let data = LoginSuccessData::new(0, login.username);
      let packet = Packet::new(2, data.to_bytes()?, client_data.compression_threshold);
      stream.write(&packet.into_bytes()?)?;

      // let data = DisconnectData::new(json!({
      //   "text": "I use Arch btw."
      // }));

      // let packet = Packet::new(0x1a, data.to_bytes()?, false);
      // stream.write(&packet.to_bytes()?)?;

      Ok(())
    }
    _ => Ok(()),
  }
}
