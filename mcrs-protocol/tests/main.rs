use mcrs_protocol::error::Error;

#[test]
fn test_runs() {
  assert!(true)
}

pub(crate) type Result<T> = std::result::Result<T, Error>;

mod packet;
mod packets;
