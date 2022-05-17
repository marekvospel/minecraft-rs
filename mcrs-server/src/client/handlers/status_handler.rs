use mcrs_protocol::packets::status::ping::PingData;
use mcrs_protocol::packets::status::status::StatusResponse;
use serde_json::json;
use std::io::Write;
use std::net::Shutdown::Both;
use std::net::TcpStream;

use crate::Result;
use crate::{ClientData, Packet};

pub(crate) fn handle_status(
  packet: &mut Packet,
  stream: &mut TcpStream,
  client_data: &mut ClientData,
) -> Result<()> {
  match packet.id {
    0 => {
      println!("[0x00] Received Status Request");

      let response = StatusResponse::new(&json!({
        "version": {
          "name": "1.18.2",
          "protocol": 758
        },
        "players": {
          "max": 420,
          "online": 69
        },
        "description": {
          "text": "Hello"
        }
      }));

      let response = response.to_bytes()?;

      let packet = Packet::new(0, response, client_data.compression_threshold);
      let packet = packet.into_bytes()?;

      println!("[0x00] Sending Status Response");
      stream.write(&packet)?;

      Ok(())
    }
    1 => {
      println!("[0x01] Received Ping");

      let ping = PingData::try_from(packet)?;

      let packet = Packet::new(1, ping.to_bytes()?, client_data.compression_threshold);
      println!("[0x01] Sending Pong");
      stream.write(&packet.into_bytes()?)?;

      println!("Closing connection");
      stream.shutdown(Both)?;

      Ok(())
    }
    _ => Ok(()),
  }
}
