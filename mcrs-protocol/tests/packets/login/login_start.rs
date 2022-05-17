use crate::Result;
use mcrs_protocol::packets::login::login_start::LoginStartData;

#[test]
fn login_start_gets_serialized() -> Result<()> {
  let data = vec![11u8, 109, 97, 114, 101, 107, 118, 111, 115, 112, 101, 108];

  let login = LoginStartData::try_from(&data)?;

  assert_eq!(login, LoginStartData::new("marekvospel"));

  Ok(())
}

#[test]
fn login_start_gets_deserialized() -> Result<()> {
  let login = LoginStartData::new("marekvospel");

  assert_eq!(
    login.bytes()?,
    vec![11u8, 109, 97, 114, 101, 107, 118, 111, 115, 112, 101, 108]
  );

  Ok(())
}
