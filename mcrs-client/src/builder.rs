use crate::callback::Callback;
use crate::client::Client;
use crate::events::ClientEvent;
use crate::Result;
use mcrs_protocol::packets::packet::Packet;
use std::collections::HashMap;
use std::net::TcpStream;
use std::thread::spawn;

#[derive(Debug)]
pub struct ClientBuilder {
  address: String,
  events: HashMap<ClientEvent, Vec<Callback>>,
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

  pub fn on<E, C>(mut self, event: E, callback: C) -> Self
  where
    E: Into<ClientEvent>,
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
      if !clone.connected().unwrap() {
        break;
      }

      let packet = clone.poll().unwrap();
      clone.callback(packet).unwrap();
    });

    Ok(client)
  }

  pub fn connect_inner(self) -> Result<Client> {
    let stream = TcpStream::connect(self.address)?;

    Ok(Client::new(stream, self.events))
  }
}
