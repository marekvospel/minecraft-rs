use std::{io, str, thread};
use std::io::{BufReader, BufWriter, Cursor, Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::net::Shutdown::Both;
use serde_json::json;
use crate::lib::packet::{GameState, HandshakeData, Packet, PingData, StatusResponse};
use crate::lib::var_int::{VarIntRead, VarIntSize};

pub mod lib;

fn main() {

  let mut listener = TcpListener::bind("127.0.0.1:3000").expect("Yikes, could not bind 3000");

  for connection in listener.incoming() {

    let stream = connection.expect("There was an error when creating connection");

    thread::spawn(move || {
      handle_connection(stream).expect("There was an error when handling connection!");
    });

  }

}

struct ClientData {
  state: GameState
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Error> {
  let mut client_data = ClientData {
    state: GameState::Handshaking
  };
  loop {
    if stream.peek(&mut vec![0u8; 1])? <= 0 {
      println!("Connection closed, exiting handle_connection");
      break
    }

    let mut packet = Packet::read(&mut stream, false)?;

    println!("id: {}", packet.id);
    println!("length: {}", packet.length);

    handle_packet(&mut packet, &mut stream, &mut client_data)?;

  }

  Ok(())
}

fn handle_packet(packet: &mut Packet, stream: &mut TcpStream, client_data: &mut ClientData) -> Result<(), Error>{
  let data = BufReader::new(Cursor::new(&mut packet.data));

  if packet.id == 0 {

    if client_data.state == GameState::Handshaking {
      println!("[0x00] Received Handshake");

      let handshake = HandshakeData::try_from(packet)?;

      println!("Setting client state to {:?}", handshake.state);
      client_data.state = handshake.state;

      if client_data.state != GameState::Status {
        stream.shutdown(Both);
      }

    } else if client_data.state == GameState::Status {
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

      let response = response.to_bytes();

      let packet = Packet::new( 0, response, false);
      let packet = packet.to_bytes();

      println!("[0x00] Sending Status Response");
      stream.write(&packet)?;

    }

  } else if packet.id == 1 {

    if client_data.state == GameState::Status {
      println!("[0x01] Received Ping");

      let ping = PingData::try_from(packet)?;

      let packet = Packet::new(1, ping.to_bytes(), false);
      println!("[0x01] Sending Pong");
      stream.write(&packet.to_bytes())?;

      println!("Closing connection");
      stream.shutdown(Both);
    }

  }

  Ok(())
}
