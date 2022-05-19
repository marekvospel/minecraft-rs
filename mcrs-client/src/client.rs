use crate::callback::Callback;
use crate::events::{ClientEvent, PacketId};
use crate::state::ClientState;
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
  events: Arc<RwLock<HashMap<ClientEvent, Vec<Callback>>>>,
  state: Arc<RwLock<ClientState>>,
}

impl Client {
  #[inline]
  pub fn new(stream: TcpStream, events: HashMap<ClientEvent, Vec<Callback>>) -> Self {
    Client {
      stream: Arc::new(RwLock::new(stream)),
      events: Arc::new(RwLock::new(events)),
      state: Arc::new(RwLock::new(ClientState::new())),
    }
  }

  #[inline]
  pub fn game_state(&self) -> GameState {
    // TODO: error handling
    self.state.read().unwrap().game_state
  }

  pub fn set_game_state<S>(&self, state: S) -> Result<()>
  where
    S: Into<GameState>,
  {
    // TODO: error handling
    let mut lock = self.state.write().unwrap();
    let client_state = &mut lock;

    client_state.game_state = state.into();

    Ok(())
  }
}

impl Client {
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

  pub fn connected(&self) -> Result<bool> {
    let mut stream = self.stream.write().unwrap();
    let lock = stream.deref_mut();

    let mut buf = [0u8];
    Ok(lock.peek(&mut buf)? > 0)
  }

  ///
  /// Internal method used to call callbacks for events
  ///
  pub fn callback(&self, packet: Packet) -> Result<()> {
    let event = ClientEvent::from(self.game_state()).set_packet_id(PacketId::Id(packet.id()));
    Self::call_callback(self, event, packet.clone())?;
    let event = ClientEvent::from(self.game_state());
    Self::call_callback(self, event, packet.clone())?;
    let event = ClientEvent::Any;
    Self::call_callback(self, event, packet.clone())?;

    Ok(())
  }

  fn call_callback(&self, event: ClientEvent, packet: Packet) -> Result<()> {
    // TODO: error handling
    let mut events = self.events.write().unwrap();
    let lock = events.deref_mut();

    let listeners = lock.get_mut(&event);
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
