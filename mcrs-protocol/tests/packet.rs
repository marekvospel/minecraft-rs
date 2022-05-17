use mcrs_protocol::error::Error;
use mcrs_protocol::packets::packet::Packet;
use std::io::{BufReader, Cursor};

#[test]
fn packet_gets_constructed() {
  let packet = Packet::new(0, vec![], -1);

  assert_eq!(
    packet,
    Packet {
      length: 1,
      id: 0,
      data: vec![],
      compression_threshold: -1
    }
  );
}

#[test]
fn packet_gets_parsed() -> Result<(), Error> {
  let mut stream = BufReader::new(Cursor::new([1u8, 0u8]));

  let packet = Packet::read(&mut stream, -1)?;

  assert_eq!(
    packet,
    Packet {
      length: 1,
      id: 0,
      data: vec![],
      compression_threshold: -1
    }
  );

  Ok(())
}

#[test]
fn compressed_packet_gets_parsed() -> Result<(), Error> {
  let data: [u8; 14] = [13, 4, 120, 156, 99, 96, 96, 96, 0, 0, 0, 4, 0, 1];
  let mut stream = BufReader::new(Cursor::new(data));

  let packet = Packet::read(&mut stream, 1)?;

  assert_eq!(
    packet,
    Packet {
      length: 4,
      id: 0,
      data: vec![0, 0, 0],
      compression_threshold: 1
    }
  );

  Ok(())
}

#[test]
fn packet_gets_serialized() -> Result<(), Error> {
  let packet = Packet::new(0, vec![], -1).into_bytes()?;

  assert_eq!(packet, vec![1u8, 0u8]);

  Ok(())
}

#[test]
fn compressed_packet_gets_serialized() -> Result<(), Error> {
  let packet = Packet::new(0, vec![0, 0, 0], 1).into_bytes()?;

  assert_eq!(packet, [13, 4, 120, 156, 99, 96, 96, 96, 0, 0, 0, 4, 0, 1]);

  Ok(())
}
