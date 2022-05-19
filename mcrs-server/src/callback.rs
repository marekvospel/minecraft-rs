// https://github.com/1c3t3a/rust-socketio/blob/main/socketio/src/client/callback.rs

use crate::server::Server;
use mcrs_client::client::Client;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};

pub struct Callback {
  inner: Box<dyn for<'a> FnMut(Client, Server) + 'static + Send + Sync>,
}

impl Callback {
  #[inline]
  pub fn new<C>(callback: C) -> Self
  where
    C: for<'a> FnMut(Client, Server) + 'static + Send + Sync,
  {
    Callback {
      inner: Box::new(callback),
    }
  }
}

impl Debug for Callback {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Callback")
  }
}

impl Deref for Callback {
  type Target = dyn for<'a> FnMut(Client, Server) + 'static + Sync + Send;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

impl DerefMut for Callback {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.inner
  }
}
