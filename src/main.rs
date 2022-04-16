extern crate core;

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
    if stream.peek(&mut vec![0u8; 1])? <= 0 {
      println!("Connection closed, exiting handle_connection");
      break;
    }

    let mut packet = Packet::read(&mut stream, false)?;

    // println!("id: {}", packet.id);
    // println!("length: {}", packet.length);

    handle_packet(&mut packet, &mut stream, &mut client_data)?;
  }

  Ok(())
}
