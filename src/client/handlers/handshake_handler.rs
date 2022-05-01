use crate::Result;
use crate::{ClientData, Packet};
use minecraft_rs::packets::handshake::HandshakeData;

pub(crate) fn handle_handshake(packet: &mut Packet, client_data: &mut ClientData) -> Result<()> {
  match packet.id {
    0 => {
      println!("[0x00] Received Handshake");

      let handshake = HandshakeData::try_from(packet)?;

      println!("Setting client state to {:?}", handshake.state);
      client_data.state = handshake.state;

      Ok(())
    }
    _ => Ok(()),
  }
}
