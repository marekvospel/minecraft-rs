use std::io::Error;

use crate::lib::packets::handshake::HandshakeData;
use crate::{ClientData, Packet};

pub fn handle_handshake(packet: &mut Packet, client_data: &mut ClientData) -> Result<(), Error> {
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
