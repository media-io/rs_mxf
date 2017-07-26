
use byteorder::{BigEndian, ReadBytesExt};
use klv::klv_reader::*;
use klv::value::value::*;

use std::io::{Read, Seek};

pub fn parse_random_index_metadata<R: Read + Seek>(reader: &mut KlvReader<R>, size: usize) -> Result<Vec<Element>, String> {
  let count = (size - 4) / 12;
  let mut rip = vec![];

  for _index in 0..count {
      let body_sid = reader.stream.read_u32::<BigEndian>().unwrap();
      let byte_offset = reader.stream.read_u64::<BigEndian>().unwrap();

      rip.push(RandomIndexEntry {
          body_sid: body_sid,
          byte_offset: byte_offset
      });
  }

  let _length = reader.stream.read_u32::<BigEndian>().unwrap();

  let value = Some(ValueData::RandomIndexEntries{
    entries: rip
  });

  Ok(vec![
    Element{
      identifier: ElementIdentifier::RandomIndexMetadata,
      value: value
    },
  ])
}
