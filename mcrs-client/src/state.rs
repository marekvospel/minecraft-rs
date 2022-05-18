use mcrs_protocol::game_state::GameState;

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct ClientState {
  pub(crate) game_state: GameState,
  pub(crate) compression_threshold: i32,
}

impl ClientState {
  pub(crate) fn new() -> Self {
    ClientState {
      game_state: GameState::Handshaking,
      compression_threshold: -1,
    }
  }
}
