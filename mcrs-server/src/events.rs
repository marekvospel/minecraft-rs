#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum ServerEvent {
  Connect,
  Disconnect,
}
