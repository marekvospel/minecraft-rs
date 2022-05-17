use crate::client::handlers::handle_legacy_ping::handle_legacy_ping;
use crate::client::handlers::packet_handler::handle_packet;
use mcrs_protocol::error::Error;
use mcrs_protocol::game_state::GameState;
use mcrs_protocol::packets::packet::Packet;
use std::net::{TcpListener, TcpStream};
use std::thread;

pub mod client;

fn main() {
  let listener = TcpListener::bind("127.0.0.1:3000").expect("Yikes, could not bind 3000");

  for connection in listener.incoming() {
    let stream = connection.expect("There was an error when creating connection");

    thread::spawn(move || {
      handle_connection(stream).expect("There was an error when handling connection!");
    });
  }
}

pub(crate) struct ClientData {
  state: GameState,
  compression_threshold: i32,
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
  let mut client_data = ClientData {
    state: GameState::Handshaking,
    compression_threshold: -1,
  };
  loop {
    let mut buf = [0u8];
    if stream.peek(&mut buf)? <= 0 {
      println!("Connection closed, exiting handle_connection");
      break;
    }

    if buf[0] == 0xfe && client_data.state == GameState::Handshaking {
      handle_legacy_ping(&mut stream)?;
      continue;
    }

    let mut packet = Packet::read(&mut stream, client_data.compression_threshold)?;

    // println!("id: {}", packet.id);
    // println!("length: {}", packet.length);

    handle_packet(&mut packet, &mut stream, &mut client_data)?;
  }

  Ok(())
}

pub(crate) type Result<T> = std::result::Result<T, Error>;
