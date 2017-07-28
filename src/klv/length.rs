
use byteorder::{ByteOrder, BigEndian, WriteBytesExt};
use serializer::encoder::*;

use std::io::Read;

#[derive(Debug)]
pub struct Length {
  pub value: usize
}

impl Encoder for Length {
  fn serialise(&self) -> Vec<u8> {
    if self.value <= 0x7f {
      return vec![self.value as u8]
    }
    if self.value <= 0xff {
      let mut result = vec![0x81];
      result.write_u8(self.value as u8).unwrap();
      return result
    }
    if self.value <= 0xffff {
      let mut result = vec![0x82];
      result.write_u16::<BigEndian>(self.value as u16).unwrap();
      return result
    }
    if self.value <= 0xffffff {
      let mut result = vec![0x83];
      result.write_u32::<BigEndian>(self.value as u32).unwrap();
      result.remove(1);
      return result
    }
    if self.value <= 0xffffffff {
      let mut result = vec![0x84];
      result.write_u32::<BigEndian>(self.value as u32).unwrap();
      return result
    }
    panic!("wrong length !!!")
  }
}

pub fn parse<R: Read>(stream: &mut R) -> Result<Option<Length>, String> {
  let mut length = vec![0; 1];
  try!(stream.read_exact(&mut length).map_err(|e| e.to_string()));

  if length[0] <= 0x7f {
    Ok(Some(Length{value: length[0] as usize}))
  } else {
    let size = length[0] - 0x80;
    let mut data = vec![0; size as usize];
    try!(stream.read_exact(&mut data).map_err(|e| e.to_string()));

    let long_length = 
      match size {
        1 => data[0] as usize,
        2 => BigEndian::read_u16(&data) as usize,
        3 => {
          data.insert(0, 0);
          BigEndian::read_u32(&data) as usize
        },
        4 => BigEndian::read_u32(&data) as usize,
        5 => {
          data.insert(0, 0);
          data.insert(0, 0);
          data.insert(0, 0);
          BigEndian::read_u64(&data) as usize
        },
        6 => {
          data.insert(0, 0);
          data.insert(0, 0);
          BigEndian::read_u64(&data) as usize
        },
        7 => {
          data.insert(0, 0);
          BigEndian::read_u64(&data) as usize
        },
        8 => BigEndian::read_u64(&data) as usize,
        _ => return Err("wrong length".to_string())
      };

    Ok(Some(Length{value: long_length as usize}))
  }
}
