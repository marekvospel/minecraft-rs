use crate::callback::Callback;
use crate::events::ServerEvent;
use crate::Result;
use mcrs_client::client::Client;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Server {
  clients: Arc<RwLock<Vec<Client>>>,
  events: Arc<RwLock<HashMap<ServerEvent, Vec<Callback>>>>,
}

impl Server {
  #[inline]
  pub fn new(events: HashMap<ServerEvent, Vec<Callback>>) -> Self {
    Server {
      clients: Arc::new(RwLock::new(Vec::new())),
      events: Arc::new(RwLock::new(events)),
    }
  }
}

impl Server {
  pub fn callback(&self, event: ServerEvent, client: Client) -> Result<()> {
    let mut events = self.events.write().unwrap();

    let listeners = events.get_mut(&event);

    if listeners.is_some() {
      let listeners = listeners.unwrap();

      for listener in listeners {
        listener(client.clone(), self.clone())
      }
    }

    Ok(())
  }
}
