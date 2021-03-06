use mcrs_protocol::error::Error;
use mcrs_protocol::packets::packet::Packet;
use std::io::{BufReader, Cursor};

#[test]
fn packet_gets_deserialized() -> Result<(), Error> {
  let mut stream = BufReader::new(Cursor::new([1u8, 0u8]));

  let packet = Packet::read(&mut stream, -1)?;

  assert_eq!(packet, Packet::new(0, vec![], -1));

  Ok(())
}

#[test]
fn compressed_packet_gets_deserialized() -> Result<(), Error> {
  let data: [u8; 14] = [13, 4, 120, 156, 99, 96, 96, 96, 0, 0, 0, 4, 0, 1];
  let mut stream = BufReader::new(Cursor::new(data));

  let packet = Packet::read(&mut stream, 1)?;

  assert_eq!(packet, Packet::new(0, vec![0, 0, 0], 1));

  Ok(())
}

#[test]
fn packet_gets_serialized() -> Result<(), Error> {
  let packet = Packet::new(0, vec![], -1).bytes()?;

  assert_eq!(packet, vec![1u8, 0u8]);

  Ok(())
}

#[test]
fn compressed_packet_gets_serialized() -> Result<(), Error> {
  let packet = Packet::new(0, vec![0, 0, 0], 1).bytes()?;

  assert_eq!(packet, [13, 4, 120, 156, 99, 96, 96, 96, 0, 0, 0, 4, 0, 1]);

  Ok(())
}
