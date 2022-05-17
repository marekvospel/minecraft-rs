use crate::Result;
use mcrs_protocol::packets::play::disconnect::DisconnectData;
use serde_json::json;

#[test]
fn disconnect_gets_deserialized() -> Result<()> {
  let data = vec![
    13u8, 123, 34, 116, 101, 120, 116, 34, 58, 34, 72, 105, 34, 125,
  ];

  let disconnect = DisconnectData::try_from(&data)?;

  assert_eq!(
    disconnect,
    DisconnectData::new(json!({
      "text": "Hi"
    }))
  );

  Ok(())
}

#[test]
fn disconnect_gets_serialized() -> Result<()> {
  let disconnect = DisconnectData::new(json!({
    "text": "Hi"
  }));

  assert_eq!(
    disconnect.bytes()?,
    vec![13u8, 123, 34, 116, 101, 120, 116, 34, 58, 34, 72, 105, 34, 125]
  );

  Ok(())
}
