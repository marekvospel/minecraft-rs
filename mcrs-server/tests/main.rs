use mcrs_client::builder::ClientBuilder;
use mcrs_protocol::error::Error;
use mcrs_server::builder::ServerBuilder;
use mcrs_server::events::ServerEvent;
use std::sync::mpsc::sync_channel;

#[test]
fn server_gets_built() -> Result<(), Error> {
  let _ = ServerBuilder::new("0.0.0.0:25565").start()?;

  Ok(())
}

#[test]
fn server_calls_on_connection() -> Result<(), Error> {
  let (tx, rx) = sync_channel(0);

  let _ = ServerBuilder::new("0.0.0.0:25566")
    .on(ServerEvent::Connect, move |_, _| {
      tx.send(true).unwrap();
    })
    .start()?;

  let _ = ClientBuilder::new("127.0.0.1:25566").connect()?;

  assert!(rx.recv().unwrap());

  Ok(())
}
