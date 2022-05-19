use crate::callback::Callback;
use crate::events::ServerEvent;
use crate::server::Server;
use crate::Result;
use mcrs_client::client::Client;
use std::collections::HashMap;
use std::net::TcpListener;
use std::thread::spawn;

#[derive(Debug)]
pub struct ServerBuilder {
  events: HashMap<ServerEvent, Vec<Callback>>,
  bind: String,
}

impl ServerBuilder {
  #[inline]
  pub fn new<S>(bind: S) -> Self
  where
    S: Into<String>,
  {
    ServerBuilder {
      events: HashMap::new(),
      bind: bind.into(),
    }
  }

  pub fn on<C, E>(mut self, event: E, callback: C) -> Self
  where
    C: for<'a> FnMut(Client, Server) + 'static + Sync + Send,
    E: Into<ServerEvent>,
  {
    let event = event.into();
    if !self.events.contains_key(&event) {
      self.events.insert(event.clone(), Vec::new());
    }

    let list = self.events.get_mut(&event).unwrap();

    list.push(Callback::new(callback));

    self
  }

  pub fn connect(self) -> Result<Server> {
    let (server, listener) = self.connect_inner()?;
    let clone = server.clone();

    spawn(move || loop {
      for connection in listener.incoming() {
        let connection = connection.unwrap();

        clone
          .callback(
            ServerEvent::Connect,
            Client::new(connection, HashMap::new()),
          )
          .unwrap();
      }
    });

    Ok(server)
  }

  pub(crate) fn connect_inner(self) -> Result<(Server, TcpListener)> {
    let listener = TcpListener::bind(self.bind)?;

    Ok((Server::new(self.events), listener))
  }
}
