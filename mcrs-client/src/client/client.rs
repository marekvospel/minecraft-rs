use crate::Result;
use mcrs_protocol::packets::packet::Packet;
use std::io::Write;
use std::net::TcpStream;
use std::ops::DerefMut;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct Client {
  stream: Arc<RwLock<TcpStream>>,
}

impl Client {
  pub(crate) fn new(stream: TcpStream) -> Self {
    let client = Client {
      stream: Arc::new(RwLock::new(stream)),
    };

    client
  }

  pub fn poll(&self) -> Result<Packet> {
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
}
