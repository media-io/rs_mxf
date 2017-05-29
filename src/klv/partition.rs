
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use serializer::encoder::*;
use klv::value::*;
use klv::essence_identifiers::*;

use std::io::Read;

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
      identifier: ElementIdentifier::PartitionMajor{value: partition_major}
    },
    Element{
      identifier: ElementIdentifier::PartitionMinor{value: partition_minor}
    },
    Element{
      identifier: ElementIdentifier::PartitionKagSize{size: kag_size}
    },
    Element{
      identifier: ElementIdentifier::PartitionThisPartition{offset: this_partition}
    },
    Element{
      identifier: ElementIdentifier::PartitionPreviousPartition{offset: previous_partition}
    },
    Element{
      identifier: ElementIdentifier::PartitionFooterPartition{offset: footer_partition}
    },
    Element{
      identifier: ElementIdentifier::PartitionHeaderByteCount{size: header_byte_count}
    },
    Element{
      identifier: ElementIdentifier::PartitionIndexByteCount{size: index_byte_count}
    },
    Element{
      identifier: ElementIdentifier::PartitionIndexSid{value: index_sid}
    },
    Element{
      identifier: ElementIdentifier::PartitionByteOffset{offset: byte_offset}
    },
    Element{
      identifier: ElementIdentifier::PartitionBodySid{value: body_sid}
    },
    Element{
      identifier: parse_operational_pattern(op_ul).unwrap()
    },
    Element{
      identifier: ElementIdentifier::PartitionEssenceContainers {
        essences: essences_kind
      }
    }
  ])
}
