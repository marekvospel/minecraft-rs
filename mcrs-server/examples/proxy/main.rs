use mcrs_client::builder::ClientBuilder;
use mcrs_client::client::Client;
use mcrs_client::events::ClientEvent::Any;
use mcrs_client::events::{ClientEvent, PacketId};
use mcrs_protocol::error::Error;
use mcrs_protocol::game_state::GameState;
use mcrs_protocol::packets::handshake::HandshakeData;
use mcrs_protocol::packets::login::login_success::LoginSuccessData;
use mcrs_protocol::packets::packet::Packet;
use mcrs_protocol::packets::status::status::StatusResponse;
use mcrs_server::builder::ServerBuilder;
use mcrs_server::events::ServerEvent::Connect;
use serde_json::json;
use std::sync::mpsc::sync_channel;

fn main() -> Result<(), Error> {
  ServerBuilder::new("0.0.0.0:25566")
    .on(Connect, |client, _server| {
      println!("CONNECTED");
      client
        .on(Any, move |packet, client| {
          println!("{:?}, {}", client.game_state(), packet.id());
        })
        .unwrap();
      client
        .on(ClientEvent::Handshaking(0.into()), move |packet, client| {
          let handshake = HandshakeData::try_from(&packet);

          if handshake.is_ok() {
            let handshake = handshake.unwrap();

            match handshake.state() {
              GameState::Status => {
                client.set_game_state(GameState::Status).unwrap();
              }
              GameState::Login => {
                client.set_game_state(GameState::Login).unwrap();
              }
              _ => {}
            }
          }

          return;
        })
        .unwrap();

      status_handlers(client.clone()).unwrap();

      login_handlers(client.clone()).unwrap();
    })
    .start()?;

  loop {}
}

fn status_handlers(client: Client) -> Result<(), Error> {
  client.on(ClientEvent::Status(0.into()), |_, mut client| {
    let server_status = StatusResponse::new(json!({
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
    let packet = Packet::new(0, server_status.bytes().unwrap(), -1);
    client.send(packet).unwrap();
  })?;

  client.on(ClientEvent::Status(1.into()), |packet, mut client| {
    let packet = Packet::new(1, packet.data().clone(), -1);
    client.send(packet).unwrap();
  })?;

  Ok(())
}

fn login_handlers(client: Client) -> Result<(), Error> {
  client.on(ClientEvent::Login(0.into()), |_, mut client| {
    let (upstream_client, uuid) = create_upstream_client().unwrap();
    println!("{}", uuid);

    let packet = Packet::new(
      2,
      LoginSuccessData::new(uuid, "marekvospel").bytes().unwrap(),
      -1,
    );
    client.send(packet).unwrap();
  })?;

  Ok(())
}

fn create_upstream_client() -> Result<(Client, u128), Error> {
  let (tx, rx) = sync_channel(0);

  println!("Connecting...");

  let mut client = ClientBuilder::new("127.0.0.1:25565")
    .on(Any, |packet, client| {
      println!("u {:?}, {}", client.game_state(), packet.id())
    })
    .on(
      ClientEvent::Login(PacketId::Id(2.into())),
      move |packet, _| {
        let data = LoginSuccessData::try_from(&packet).unwrap();
        tx.send(data.uuid()).unwrap();
      },
    )
    .connect()?;

  let packet = HandshakeData::new(758, "localhost", 25565, GameState::Login);
  client.send(Packet::new(0, packet.bytes()?, -1))?;
  client.set_game_state(GameState::Login)?;

  Ok((client, rx.recv().unwrap()))
}
