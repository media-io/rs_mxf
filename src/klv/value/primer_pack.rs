
use byteorder::{BigEndian, ReadBytesExt};
use klv::klv_reader::*;
use klv::value::value::*;
use klv::ul::*;

use std::io::{Read, Seek};

pub fn parse_primer_pack<R: Read + Seek>(reader: &mut KlvReader<R>) -> Result<Vec<Element>, String> {
  let number_of_items = reader.stream.read_u32::<BigEndian>().unwrap();
  let item_size = reader.stream.read_u32::<BigEndian>().unwrap();

  let mut mapping = vec![];

  match item_size {
    18 => {
      for _index in 0..number_of_items {
        let tag = reader.stream.read_u16::<BigEndian>().unwrap();
        let mut ul_data = vec![0; 16];
        try!(reader.stream.read_exact(&mut ul_data).map_err(|e| e.to_string()));

        match match_ul(ul_data) {
          Some(ul) => {
            let dynamic_tag = DynamicTag {
              tag: tag,
              identifier: ul
            };
            mapping.push(dynamic_tag.clone());
            reader.elements.push(dynamic_tag);
          }
          None => {
          }
        }
      }
    },
    _ => unimplemented!(),
  }

  Ok(vec![
    Element{
      identifier: Ul::PrimerPack,
      value: Some(ValueData::DynamicTags {
        entries: mapping
      })
    },
  ])
}
