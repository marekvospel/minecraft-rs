use crate::Result;
use mcrs_protocol::packets::status::ping::PingData;

#[test]
fn ping_gets_deserialized() -> Result<()> {
  let data = vec![0u8, 0, 0, 0, 0, 0, 5, 57];

  let ping = PingData::try_from(&data)?;

  assert_eq!(ping, PingData::new(1337));

  Ok(())
}

#[test]
fn ping_gets_serialized() -> Result<()> {
  let status = PingData::new(1337);

  assert_eq!(status.bytes()?, vec![0u8, 0, 0, 0, 0, 0, 5, 57]);

  Ok(())
}
