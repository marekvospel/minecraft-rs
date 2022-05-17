#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum GameState {
  Handshaking = 0,
  Status = 1,
  Login = 2,
  Play = 3,
}

impl From<i32> for GameState {
  fn from(value: i32) -> Self {
    use GameState::*;
    match value {
      0 => Handshaking,
      1 => Status,
      2 => Login,
      3 => Play,
      _ => Handshaking,
    }
  }
}

impl GameState {
  pub fn as_i32(&self) -> i32 {
    match self {
      GameState::Handshaking => 0,
      GameState::Status => 1,
      GameState::Login => 2,
      GameState::Play => 3,
    }
  }
}
