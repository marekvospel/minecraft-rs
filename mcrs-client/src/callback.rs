// https://github.com/1c3t3a/rust-socketio/blob/main/socketio/src/client/callback.rs

use crate::client::Client;
use mcrs_protocol::packets::packet::Packet;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};

pub(crate) struct Callback {
  inner: Box<dyn for<'a> Fn(Packet, Client) + 'static + Send + Sync>,
}

impl Callback {
  #[inline]
  pub(crate) fn new<C>(callback: C) -> Self
  where
    C: for<'a> Fn(Packet, Client) + 'static + Send + Sync,
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
  type Target = dyn for<'a> FnMut(Packet, Client) + 'static + Sync + Send;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

impl DerefMut for Callback {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.inner
  }
}
