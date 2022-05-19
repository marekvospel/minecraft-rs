use mcrs_client::builder::ClientBuilder;
use mcrs_client::events::{ClientEvent, PacketId};
use mcrs_protocol::error::Error;
use mcrs_protocol::game_state::GameState;
use mcrs_protocol::packets::handshake::HandshakeData;
use mcrs_protocol::packets::packet::Packet;
use mcrs_protocol::packets::status::ping::PingData;
use std::net::TcpStream;
use std::sync::mpsc::sync_channel;
use std::thread::sleep;
use std::time::Duration;

fn server_online() -> bool {
  TcpStream::connect("localhost:25577").is_ok()
}

#[test]
fn client_pings_manually() -> Result<(), Error> {
  sleep(Duration::from_millis(100));
  if !server_online() {
    return Ok(());
  }

  let mut client = ClientBuilder::new("localhost:25577").connect_inner()?;

  let handshake = HandshakeData::new(758, "localhost", 25565, GameState::Status);
  let packet = Packet::new(0, handshake.bytes()?, -1);

  client.send(packet)?;

  let packet = Packet::new(0, vec![], -1);
  client.send(packet)?;

  let ping = PingData::new(69);
  let packet = Packet::new(1, ping.bytes()?, -1);
  client.send(packet)?;

  let _ = client.poll()?;

  let received = client.poll()?;
  let pong = PingData::try_from(&received)?;

  assert_eq!(pong.payload(), 69);

  Ok(())
}

#[test]
fn client_pings_auto() -> Result<(), Error> {
  sleep(Duration::from_millis(100));
  if !server_online() {
    return Ok(());
  }

  let (tx, rx) = sync_channel(0);

  let mut client = ClientBuilder::new("localhost:25577")
    .on(
      ClientEvent::Status(PacketId::Id(0)),
      move |_, mut client| {
        // Send ping
        let ping = PingData::new(69);
        let packet = Packet::new(1, ping.bytes().unwrap(), -1);
        // Server isn't able to change
        sleep(Duration::from_millis(10));
        client.send(packet).unwrap();
      },
    )
    .on(ClientEvent::Status(PacketId::Id(1)), move |packet, _| {
      // Receive pong
      let data = PingData::try_from(packet.data()).unwrap();
      tx.send(data.payload()).unwrap();
    })
    .connect()?;

  let handshake = HandshakeData::new(758, "localhost", 25565, GameState::Status);
  let packet = Packet::new(0, handshake.bytes()?, -1);
  client.set_game_state(GameState::Status)?;

  client.send(packet)?;

  let packet = Packet::new(0, vec![], -1);
  client.send(packet)?;

  assert_eq!(rx.recv().unwrap(), 69);

  Ok(())
}
