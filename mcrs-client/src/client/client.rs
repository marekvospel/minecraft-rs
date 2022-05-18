use crate::client::callback::Callback;
use crate::Result;
use mcrs_protocol::game_state::GameState;
use mcrs_protocol::packets::packet::Packet;
use std::collections::HashMap;
use std::fmt::Debug;
use std::io::Write;
use std::net::TcpStream;
use std::ops::DerefMut;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct Client {
  stream: Arc<RwLock<TcpStream>>,
  events: Arc<RwLock<HashMap<String, Vec<Callback>>>>,
}

impl Client {
  pub(crate) fn new(stream: TcpStream, events: HashMap<String, Vec<Callback>>) -> Self {
    let client = Client {
      stream: Arc::new(RwLock::new(stream)),
      events: Arc::new(RwLock::new(events)),
    };

    client
  }

  pub fn poll(&self) -> Result<Packet> {
    // TODO: error handling
    let mut stream = self.stream.write().unwrap();
    let lock = stream.deref_mut();

    let packet = Packet::read(lock, -1)?;

    Ok(packet)
  }

  pub fn send(&mut self, packet: Packet) -> Result<()> {
    // TODO: error handling
    let mut stream = self.stream.write().unwrap();

    stream.write(&packet.bytes()?)?;

    Ok(())
  }

  pub(crate) fn callback(&self, state: GameState, packet: Packet) -> Result<()> {
    // TODO: error handling
    let mut events = self.events.write().unwrap();
    let lock = events.deref_mut();

    let listeners = lock.get_mut(&format!("{}:{}", state.to_string(), packet.id()));
    if listeners.is_some() {
      let listeners = listeners.unwrap();

      for listener in listeners {
        listener(packet.clone(), self.clone())
      }
    }

    drop(lock);

    Ok(())
  }
}
