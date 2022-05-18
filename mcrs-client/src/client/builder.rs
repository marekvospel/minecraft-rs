use crate::client::client::Client;
use crate::Result;

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
    Ok(Client::new(self.address)?)
  }
}
