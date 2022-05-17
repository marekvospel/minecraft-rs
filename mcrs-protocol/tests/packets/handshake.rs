use crate::Result;
use mcrs_protocol::game_state::GameState;
use mcrs_protocol::packets::handshake::HandshakeData;

#[test]
fn handshake_gets_deserialized() -> Result<()> {
  let data = vec![
    246u8, 5, 9, 108, 111, 99, 97, 108, 104, 111, 115, 116, 99, 221, 1,
  ];

  let handshake = HandshakeData::try_from(&data)?;

  assert_eq!(
    handshake,
    HandshakeData::new(758, "localhost", 25565, GameState::Status)
  );

  Ok(())
}

#[test]
fn handshake_gets_serialized() -> Result<()> {
  let handshake = HandshakeData::new(758, "localhost", 25565, GameState::Status).bytes()?;

  assert_eq!(
    handshake,
    vec![246u8, 5, 9, 108, 111, 99, 97, 108, 104, 111, 115, 116, 99, 221, 1]
  );

  Ok(())
}
