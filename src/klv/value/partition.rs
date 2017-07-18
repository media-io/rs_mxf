
use byteorder::{BigEndian, ReadBytesExt};
use klv::value::value::*;
use klv::value::essence_identifiers::*;

use std::io::Read;

#[derive(Debug, PartialEq, Clone)]
pub enum PartitionStatus {
  OpenAndIncomplete,
  ClosedAndIncomplete,
  OpenAndComplete,
  ClosedAndComplete,
}

pub fn partition_status_value(status: PartitionStatus) -> u8 {
  match status {
    PartitionStatus::OpenAndIncomplete => 0x01,
    PartitionStatus::ClosedAndIncomplete => 0x02,
    PartitionStatus::OpenAndComplete => 0x03,
    PartitionStatus::ClosedAndComplete => 0x04
  }
}

#[macro_export]
macro_rules! partition_status_value {
  (PartitionStatus::OpenAndIncomplete) => (0x01);
  (PartitionStatus::ClosedAndIncomplete) => (0x02);
  (PartitionStatus::OpenAndComplete) => (0x03);
  (PartitionStatus::ClosedAndComplete) => (0x04);
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
    let essence_kind = parse_essence_ul(essence_ul);
    essences_kind.push(essence_kind);
  }

  Ok(vec![
    Element{
      identifier: ElementIdentifier::PartitionMajor{value: partition_major},
      value: None
    },
    Element{
      identifier: ElementIdentifier::PartitionMinor{value: partition_minor},
      value: None
    },
    Element{
      identifier: ElementIdentifier::PartitionKagSize{size: kag_size},
      value: None
    },
    Element{
      identifier: ElementIdentifier::PartitionThisPartition{offset: this_partition},
      value: None
    },
    Element{
      identifier: ElementIdentifier::PartitionPreviousPartition{offset: previous_partition},
      value: None
    },
    Element{
      identifier: ElementIdentifier::PartitionFooterPartition{offset: footer_partition},
      value: None
    },
    Element{
      identifier: ElementIdentifier::PartitionHeaderByteCount{size: header_byte_count},
      value: None
    },
    Element{
      identifier: ElementIdentifier::PartitionIndexByteCount{size: index_byte_count},
      value: None
    },
    Element{
      identifier: ElementIdentifier::PartitionIndexSid{value: index_sid},
      value: None
    },
    Element{
      identifier: ElementIdentifier::PartitionByteOffset{offset: byte_offset},
      value: None
    },
    Element{
      identifier: ElementIdentifier::PartitionBodySid{value: body_sid},
      value: None
    },
    Element{
      identifier: parse_operational_pattern(op_ul).unwrap(),
      value: None
    },
    Element{
      identifier: ElementIdentifier::PartitionEssenceContainers {
        essences: essences_kind
      },
      value: None
    }
  ])
}
