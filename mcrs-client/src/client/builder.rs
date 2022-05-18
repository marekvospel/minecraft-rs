use crate::client::client::Client;
use crate::Result;
use std::net::TcpStream;

pub struct ClientBuilder {
  address: String,
}

impl ClientBuilder {
  #[inline]
  pub fn new<S>(address: S) -> Self
  where
    S: Into<String>,
  {
    ClientBuilder {
      address: address.into(),
    }
  }

  pub fn connect(self) -> Result<Client> {
    let stream = TcpStream::connect(self.address)?;

    Ok(Client::new(stream))
  }
}
