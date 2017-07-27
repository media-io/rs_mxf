
use byteorder::{BigEndian, WriteBytesExt};
use serializer::encoder::*;

pub struct Tag {
  pub id: [u8; 2],
  pub data: Vec<u8>
}

impl Encoder for Tag {
  fn serialise(&self) -> Vec<u8> {
    let mut result = vec![];
    let mut t = self.data.clone();
    result.push(self.id[0]);
    result.push(self.id[1]);

    result.write_u16::<BigEndian>(t.len() as u16).unwrap();
    result.append(&mut t);
    result
  }
}
