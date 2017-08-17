
use byteorder::{BigEndian, ReadBytesExt};
use klv::ul::Ul;
use klv::klv_reader::*;
use klv::value::value_data::*;
use klv::value::element::Element;
use serializer::decoder::Decoder;
use std::io::{Read, Seek};
use timecode;

pub fn parse_system_item_system<R: Read + Seek>(mut reader: &mut KlvReader<R>, _size: usize) -> Result<Vec<Element>, String> {
  
  let _smb = reader.stream.read_u8().unwrap();
  let _cpr = reader.stream.read_u8().unwrap();
  let _cpt = reader.stream.read_u8().unwrap();
  let channel_handle = reader.stream.read_u16::<BigEndian>().unwrap();
  let continuity_count = reader.stream.read_u16::<BigEndian>().unwrap();


  let mut smpte_ul = Ul::Unknown;
  match smpte_ul.deserialize(&mut reader) {
    Ok(_) => {},
    Err(msg) => return Err(msg.to_string())
  }

  let mut creation_date_timestamp = vec![0; 17];
  let _res = reader.stream.read(&mut creation_date_timestamp).unwrap();

  let mut user_date_timestamp = vec![0; 17];
  let _res = reader.stream.read(&mut user_date_timestamp).unwrap();

  let creation_tc = timecode::parser::smpte_331m(&creation_date_timestamp);
  let user_tc = timecode::parser::smpte_331m(&user_date_timestamp);

  Ok(vec![
    Element{
      identifier: Ul::SystemItemSystemMetadataPack,
      value: Some(ValueData::SystemItemSystem {
        channel_handle: channel_handle,
        continuity_count: continuity_count,
        essence_ul: smpte_ul,
        creation_timestamp: creation_tc,
        user_timestamp: user_tc,
      })
    },
  ])
}

pub fn parse_system_item_package<R: Read + Seek>(_reader: &mut KlvReader<R>, _size: usize) -> Result<Vec<Element>, String> {
  Ok(vec![])
}
