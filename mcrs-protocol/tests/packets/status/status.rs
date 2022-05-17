use crate::Result;
use mcrs_protocol::packets::status::status::StatusResponse;
use serde_json::json;

#[test]
fn status_gets_deserialized() -> Result<()> {
  let data = vec![
    28u8, 123, 34, 118, 101, 114, 115, 105, 111, 110, 34, 58, 123, 34, 112, 114, 111, 116, 111, 99,
    111, 108, 34, 58, 55, 53, 56, 125, 125,
  ];

  let status = StatusResponse::try_from(&data)?;

  assert_eq!(
    status,
    StatusResponse::new(json!({
      "version": {
            "protocol": 758
          },
    }))
  );

  Ok(())
}

#[test]
fn status_gets_serialized() -> Result<()> {
  let status = StatusResponse::new(json!({
    "version": {
          "protocol": 758
        },
  }));

  assert_eq!(
    status.bytes()?,
    vec![
      28u8, 123, 34, 118, 101, 114, 115, 105, 111, 110, 34, 58, 123, 34, 112, 114, 111, 116, 111,
      99, 111, 108, 34, 58, 55, 53, 56, 125, 125,
    ]
  );

  Ok(())
}
