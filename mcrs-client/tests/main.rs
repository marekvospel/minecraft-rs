use mcrs_client::client::builder::ClientBuilder;
use mcrs_protocol::error::Error;
use mcrs_protocol::game_state::GameState;
use mcrs_protocol::packets::handshake::HandshakeData;
use mcrs_protocol::packets::packet::Packet;
use mcrs_protocol::packets::status::ping::PingData;
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;

fn server_online() -> bool {
  TcpStream::connect("localhost:25577").is_ok()
}

#[test]
fn client_pings() -> Result<(), Error> {
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
fn client_logs_events() -> Result<(), Error> {
  sleep(Duration::from_millis(100));
  if !server_online() {
    return Ok(());
  }

  let mut client = ClientBuilder::new("localhost:25577")
    .on("Status:0", move |_, _| println!("Status:0"))
    .connect()?;

  let handshake = HandshakeData::new(758, "localhost", 25565, GameState::Status);
  let packet = Packet::new(0, handshake.bytes()?, -1);

  client.send(packet)?;

  let packet = Packet::new(0, vec![], -1);
  client.send(packet)?;

  // let ping = PingData::new(69);
  // let packet = Packet::new(1, ping.bytes()?, -1);
  // client.send(packet)?;

  sleep(Duration::from_secs(1));

  Ok(())
}
