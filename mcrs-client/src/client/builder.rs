use crate::client::callback::Callback;
use crate::client::client::Client;
use crate::Result;
use mcrs_protocol::game_state::GameState;
use mcrs_protocol::packets::packet::Packet;
use std::collections::HashMap;
use std::net::TcpStream;
use std::thread::spawn;

#[derive(Debug)]
pub struct ClientBuilder {
  address: String,
  events: HashMap<String, Vec<Callback>>,
}

impl ClientBuilder {
  #[inline]
  pub fn new<S>(address: S) -> Self
  where
    S: Into<String>,
  {
    ClientBuilder {
      address: address.into(),
      events: HashMap::new(),
    }
  }

  pub fn on<S, C>(mut self, event: S, callback: C) -> Self
  where
    S: Into<String>,
    C: for<'a> Fn(Packet, Client) + 'static + Send + Sync,
  {
    let event = event.into();
    if !self.events.contains_key(&event) {
      self.events.insert(event.clone(), Vec::new());
    }

    let list = self.events.get_mut(&event).unwrap();

    list.push(Callback::new(callback));

    self
  }

  pub fn connect(self) -> Result<Client> {
    let client = Self::connect_inner(self)?;
    let clone = client.clone();

    spawn(move || loop {
      let packet = clone.poll().unwrap();
      clone.callback(GameState::Status, packet).unwrap();
    });

    Ok(client)
  }

  pub fn connect_inner(self) -> Result<Client> {
    let stream = TcpStream::connect(self.address)?;

    Ok(Client::new(stream, self.events))
  }
}
