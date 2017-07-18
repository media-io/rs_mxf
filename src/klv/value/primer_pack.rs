
use byteorder::{BigEndian, ReadBytesExt};
use klv::value::value::*;
use klv::ul::match_ul;
use klv::value::tag::DynamicTagList;

use std::io::Read;

pub fn parse_primer_pack<R: Read>(stream: &mut R) -> Result<Vec<Element>, String> {
  let number_of_items = stream.read_u32::<BigEndian>().unwrap();
  let item_size = stream.read_u32::<BigEndian>().unwrap();

  let mut mapping = vec![];

  match item_size {
    18 => {
      for _index in 0..number_of_items {
        let tag = stream.read_u16::<BigEndian>().unwrap();
        let mut ul_data = vec![0; 16];
        try!(stream.read_exact(&mut ul_data).map_err(|e| e.to_string()));

        match match_ul(ul_data) {
          Some(ul) => {
            mapping.push(DynamicTagList{
              tag: tag,
              identifier: ul
            });
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
      identifier: ElementIdentifier::PrimerPack{
        data: mapping
      },
      value: None
    },
  ])
}
