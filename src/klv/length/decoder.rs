
use byteorder::{ByteOrder, BigEndian};
use std::io::{Error, Read, Seek};
use serializer::decoder::*;
use klv::klv_reader::KlvReader;
use klv::length::Length;

impl Decoder for Length {
  fn deserialize<R: Read + Seek>(&mut self, reader: &mut KlvReader<R>) -> Result<bool, Error> {
    let mut length = vec![0; 1];
    match reader.stream.read_exact(&mut length) {
      Ok(()) => {},
      Err(msg) => {
        return Err(msg)
      },
    }

    if length[0] <= 0x7f {
      self.value = length[0] as usize;
      return Ok(true);
    } else {
      let size = length[0] - 0x80;
      let mut data = vec![0; size as usize];
      match reader.stream.read_exact(&mut data) {
        Ok(()) => {},
        Err(msg) => {
          return Err(msg)
        },
      }

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
          _ => unimplemented!()
        };

      self.value = long_length as usize;
      return Ok(true);
    }
  }
}
