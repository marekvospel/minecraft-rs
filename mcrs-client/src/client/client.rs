use crate::Result;
use mcrs_protocol::packets::packet::Packet;
use std::io::Write;
use std::net::TcpStream;
use std::ops::DerefMut;
use std::sync::{Arc, RwLock};
use std::thread::spawn;

#[derive(Debug, Clone)]
pub struct Client {
  stream: Arc<RwLock<TcpStream>>,
}

impl Client {
  pub(crate) fn new<U>(url: U) -> Result<Self>
  where
    U: Into<String>,
  {
    let client = Client {
      stream: Arc::new(RwLock::new(TcpStream::connect(url.into())?)),
    };

    Ok(client)
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
