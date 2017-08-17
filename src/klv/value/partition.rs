
use byteorder::{BigEndian, ReadBytesExt};
use klv::ul::Ul;
use klv::ul::ul::*;
use klv::value::value_data::*;
use klv::value::element::*;

use std::io::Read;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PartitionStatus {
  OpenAndIncomplete,
  ClosedAndIncomplete,
  OpenAndComplete,
  ClosedAndComplete,
}

pub fn parse_status(s: u8) -> PartitionStatus {
  match s {
    0x01 => PartitionStatus::OpenAndIncomplete,
    0x02 => PartitionStatus::ClosedAndIncomplete,
    0x03 => PartitionStatus::OpenAndComplete,
    0x04 => PartitionStatus::ClosedAndComplete,
       _ => panic!("Unknown partition status"),
  }
}

pub fn parse_partition<R: Read>(stream: &mut R) -> Result<Vec<Element>, String> {
  let partition_major    = stream.read_u16::<BigEndian>().unwrap();
  let partition_minor    = stream.read_u16::<BigEndian>().unwrap();
  let kag_size           = stream.read_u32::<BigEndian>().unwrap();
  let this_partition     = stream.read_u64::<BigEndian>().unwrap();
  let previous_partition = stream.read_u64::<BigEndian>().unwrap();
  let footer_partition   = stream.read_u64::<BigEndian>().unwrap();
  let header_byte_count  = stream.read_u64::<BigEndian>().unwrap();
  let index_byte_count   = stream.read_u64::<BigEndian>().unwrap();
  let index_sid          = stream.read_u32::<BigEndian>().unwrap();
  let byte_offset        = stream.read_u64::<BigEndian>().unwrap();
  let body_sid           = stream.read_u32::<BigEndian>().unwrap();

  let mut op_ul = vec![0; 16];
  stream.read_exact(&mut op_ul).unwrap();

  let count_ul_essences = stream.read_u32::<BigEndian>().unwrap();
  let _length_ul_essences = stream.read_u32::<BigEndian>().unwrap();

  let mut essences_kind = vec![];
  for _index in 0..count_ul_essences {
    let mut essence_ul = vec![0; 16];
    stream.read_exact(&mut essence_ul).unwrap();
    match match_ul(essence_ul) {
      Some(essence_kind) => {
        essences_kind.push(essence_kind);
      },
      None => {}
    }
  }

  Ok(vec![
    build_element!(Ul::PartitionMajor, uint16 => partition_major),
    build_element!(Ul::PartitionMinor, uint16 => partition_minor),
    build_element!(Ul::PartitionKagSize, uint32 => kag_size),
    build_element!(Ul::PartitionThisPartition, uint64 => this_partition),
    build_element!(Ul::PartitionPreviousPartition, uint64 => previous_partition),
    build_element!(Ul::PartitionFooterPartition, uint64 => footer_partition),
    build_element!(Ul::PartitionHeaderByteCount, uint64 => header_byte_count),
    build_element!(Ul::PartitionIndexByteCount, uint64 => index_byte_count),
    build_element!(Ul::PartitionIndexSid, uint32 => index_sid),
    build_element!(Ul::PartitionBodyOffset, uint64 => byte_offset),
    build_element!(Ul::PartitionBodySid, uint32 => body_sid),
    Element {
      identifier: Ul::PartitionOperationalPattern,
      value: Some(ValueData::Ul{
        data: match_ul(op_ul).unwrap()
      })
    },
    Element {
      identifier: Ul::PartitionEssenceContainers,
      value: Some(ValueData::ArrayUl{
        data: essences_kind
      })
    }
  ])
}
