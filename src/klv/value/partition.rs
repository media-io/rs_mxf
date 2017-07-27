
use byteorder::{BigEndian, ReadBytesExt};
use klv::ul::*;
use klv::value::value::*;

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
    match match_ul(essence_ul) {
      Some(essence_kind) => {
        essences_kind.push(essence_kind);
      },
      None => {}
    }
  }

  Ok(vec![
    Element{
      identifier: Ul::PartitionMajor,
      value: Some(ValueData::Uint16{
        data: partition_major
      })
    },
    Element{
      identifier: Ul::PartitionMinor,
      value: Some(ValueData::Uint16{
        data: partition_minor
      })
    },
    Element{
      identifier: Ul::PartitionKagSize,
      value: Some(ValueData::Uint32{
        data: kag_size
      })
    },
    Element{
      identifier: Ul::PartitionThisPartition,
      value: Some(ValueData::Uint64{
        data: this_partition
      })
    },
    Element{
      identifier: Ul::PartitionPreviousPartition,
      value: Some(ValueData::Uint64{
        data: previous_partition
      })
    },
    Element{
      identifier: Ul::PartitionFooterPartition,
      value: Some(ValueData::Uint64{
        data: footer_partition
      })
    },
    Element{
      identifier: Ul::PartitionHeaderByteCount,
      value: Some(ValueData::Uint64{
        data: header_byte_count
      })
    },
    Element{
      identifier: Ul::PartitionIndexByteCount,
      value: Some(ValueData::Uint64{
        data: index_byte_count
      })
    },
    Element{
      identifier: Ul::PartitionIndexSid,
      value: Some(ValueData::Uint32{
        data: index_sid
      })
    },
    Element{
      identifier: Ul::PartitionByteOffset,
      value: Some(ValueData::Uint64{
        data: byte_offset
      })
    },
    Element{
      identifier: Ul::PartitionBodySid,
      value: Some(ValueData::Uint32{
        data: body_sid
      })
    },
    // Element{
    //   identifier: Ul::PartitionOperationalPattern,
    //   value: Some(ValueData::Ul{
    //     data: match_ul(op_ul).unwrap()
    //   })
    // },
    Element{
      identifier: Ul::PartitionEssenceContainers,
      value: Some(ValueData::ArrayUl{
        data: essences_kind
      })
    }
  ])
}
