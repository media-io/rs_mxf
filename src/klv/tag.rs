
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use serializer::encoder::*;
use klv::value::*;
use klv::essence_identifiers::*;

use std::io::Read;

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


// pub fn parse_tag<R: Read>(stream: &mut R) -> Result<Option<Tag>, String> {
//   let mut data = vec![0; 2];
//   try!(stream.read_exact(&mut data).map_err(|e| e.to_string()));

//   match (data[0], data[1]) {
//     (0x00, 0x00) => {
//       println!("TAG");
//     },
//     _ => {
//       println!("Unknown TAG");
//     }
//   }
// }
