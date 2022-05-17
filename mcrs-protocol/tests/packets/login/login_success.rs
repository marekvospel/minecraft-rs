use crate::Result;
use mcrs_protocol::packets::login::login_success::LoginSuccessData;

#[test]
fn login_start_gets_serialized() -> Result<()> {
  let data = vec![
    0u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 109, 97, 114, 101, 107, 118, 111, 115,
    112, 101, 108,
  ];

  let login = LoginSuccessData::try_from(&data)?;

  assert_eq!(login, LoginSuccessData::new(0, "marekvospel"));

  Ok(())
}

#[test]
fn login_start_gets_deserialized() -> Result<()> {
  let login = LoginSuccessData::new(0, "marekvospel");

  assert_eq!(
    login.bytes()?,
    vec![
      0u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 109, 97, 114, 101, 107, 118, 111, 115,
      112, 101, 108
    ]
  );

  Ok(())
}
