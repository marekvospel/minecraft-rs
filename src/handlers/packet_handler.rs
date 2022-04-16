use crate::handlers::handshake_handler::handle_handshake;
use crate::handlers::status_handler::handle_status;
use crate::{ClientData, GameState, Packet};
use std::io::Error;
use std::net::TcpStream;

pub fn handle_packet(
  packet: &mut Packet,
  stream: &mut TcpStream,
  client_data: &mut ClientData,
) -> Result<(), Error> {
  use GameState::*;
  match client_data.state {
    Handshaking => handle_handshake(packet, client_data),
    Status => handle_status(packet, stream, client_data),
    _ => Ok(()),
  }
}
