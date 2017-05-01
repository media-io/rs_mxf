
use serializer::encoder::*;

use klv::key::*;
use klv::value::*;
use klv::length::*;

use std::io::{Read, BufReader, Seek, SeekFrom};

#[derive(Debug)]
pub struct Klv {
  pub key: Key,
  pub value: Value
}

impl Encoder for Klv {
  fn serialise(&self) -> Vec<u8> {
    let mut key_data = Encoder::serialise(&self.key);
    let mut value_data = Encoder::serialise(&self.value);

    let length = Length{value: value_data.len()};

    let mut length_data = Encoder::serialise(&length);

    key_data.append(&mut length_data);
    key_data.append(&mut value_data);
    key_data
  }
}

pub fn next_klv<R: Read + Seek>(mut stream: &mut BufReader<R>) -> Result<Option<Klv>, String>
{
  let mut identifier_data = vec![0; 16];
  try!(stream.read_exact(&mut identifier_data).map_err(|e| e.to_string()));
  let key = parse_key(identifier_data);
  let length = parse(&mut stream).unwrap().unwrap();

  let address = stream.seek(SeekFrom::Current(0)).unwrap();

  let mut data = vec![0; length.value];
  try!(stream.read_exact(&mut data).map_err(|e| e.to_string()));

  let identifier = ElementIdentifier::ContentData {
    address: address as usize,
    size: length.value
  };

  let value = Value{
    elements: vec![
      Element{
        identifier: identifier
      }]
  };

  let klv = Klv{
    key: key,
    value: value
  };

  Ok(Some(klv))
}
