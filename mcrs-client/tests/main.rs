use mcrs_client::client::builder::ClientBuilder;
use mcrs_protocol::error::Error;
use mcrs_protocol::game_state::GameState;
use mcrs_protocol::packets::handshake::HandshakeData;
use mcrs_protocol::packets::packet::Packet;
use mcrs_protocol::packets::status::ping::PingData;
use mcrs_protocol::packets::status::status::StatusResponse;
use std::thread::sleep;
use std::time::Duration;

#[test]
fn client_pings() -> Result<(), Error> {
  let mut client = ClientBuilder::new("localhost:25577").connect()?;

  let handshake = HandshakeData::new(758, "localhost", 25565, GameState::Status);
  let packet = Packet::new(0, handshake.bytes()?, -1);

  client.send(packet)?;

  let packet = Packet::new(0, vec![], -1);
  client.send(packet)?;

  let ping = PingData::new(69);
  let packet = Packet::new(1, ping.bytes()?, -1);
  client.send(packet)?;

  let received = client.poll()?;
  let status = StatusResponse::try_from(&received)?;

  println!("{:?}, {:?}", received, status);

  let received = client.poll()?;
  let pong = PingData::try_from(&received)?;

  println!("{:?}, {:?}", received, pong);

  Ok(())
}