use mcrs_protocol::game_state::GameState;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum ClientEvent {
  Any,
  Handshaking(PacketId),
  Status(PacketId),
  Login(PacketId),
  Play(PacketId),
}

impl ClientEvent {
  pub fn set_packet_id<I>(self, id: I) -> Self
  where
    I: Into<PacketId>,
  {
    let id = id.into();

    match self {
      ClientEvent::Any => ClientEvent::Any,
      ClientEvent::Handshaking(_) => ClientEvent::Handshaking(id),
      ClientEvent::Status(_) => ClientEvent::Status(id),
      ClientEvent::Login(_) => ClientEvent::Login(id),
      ClientEvent::Play(_) => ClientEvent::Play(id),
    }
  }
}

impl From<GameState> for ClientEvent {
  #[inline]
  fn from(state: GameState) -> Self {
    match state {
      GameState::Handshaking => ClientEvent::Handshaking(PacketId::Any),
      GameState::Status => ClientEvent::Status(PacketId::Any),
      GameState::Login => ClientEvent::Login(PacketId::Any),
      GameState::Play => ClientEvent::Play(PacketId::Any),
    }
  }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum PacketId {
  Any,
  Id(i32),
}

impl From<i32> for PacketId {
  fn from(id: i32) -> Self {
    PacketId::Id(id)
  }
}
