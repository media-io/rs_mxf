
use byteorder::{BigEndian, WriteBytesExt};
use serializer::encoder::*;
use klv::length::Length;

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
