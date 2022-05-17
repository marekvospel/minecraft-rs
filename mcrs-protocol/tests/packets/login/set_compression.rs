use crate::Result;
use mcrs_protocol::packets::login::set_compression::SetCompressionData;

#[test]
fn set_compression_deserialize() -> Result<()> {
  let data = vec![128u8, 1];

  let compression = SetCompressionData::try_from(&data)?;

  assert_eq!(compression, SetCompressionData::new(128));

  Ok(())
}

#[test]
fn set_compression_serialize() -> Result<()> {
  let compression = SetCompressionData::new(128);

  assert_eq!(compression.bytes()?, vec![128u8, 1]);

  Ok(())
}
