extern crate core;

use crate::handlers::handle_legacy_ping::handle_legacy_ping;
use crate::lib::packet::{GameState, HandshakeData, Packet, PingData, StatusResponse};
use crate::lib::var_int::{VarIntRead, VarIntSize};
use std::io::Error;
use std::net::{TcpListener, TcpStream};
use std::thread;

use crate::handlers::packet_handler::handle_packet;

pub mod handlers;
pub mod lib;

fn main() {
  let listener = TcpListener::bind("127.0.0.1:3000").expect("Yikes, could not bind 3000");

  for connection in listener.incoming() {
    let stream = connection.expect("There was an error when creating connection");

    thread::spawn(move || {
      handle_connection(stream).expect("There was an error when handling connection!");
    });
  }
}

pub struct ClientData {
  state: GameState,
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Error> {
  let mut client_data = ClientData {
    state: GameState::Handshaking,
  };
  loop {
    let mut buf = [0u8];
    if stream.peek(&mut buf)? <= 0 {
      println!("Connection closed, exiting handle_connection");
      break;
    }

    if buf[0] == 0xfe && client_data.state == GameState::Handshaking {
      handle_legacy_ping(&mut stream, &mut client_data);
      continue;
    }

    let mut packet = Packet::read(&mut stream, false)?;

    // println!("id: {}", packet.id);
    // println!("length: {}", packet.length);

    handle_packet(&mut packet, &mut stream, &mut client_data)?;
  }

  Ok(())
}
