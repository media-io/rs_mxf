
use byteorder::{BigEndian, WriteBytesExt};
use serializer::encoder::*;

use klv::tag::*;
use klv::essence_identifiers::*;

fn get_smpte_identifier() -> Vec<u8> {
  vec![0x06, 0x0e, 0x2b, 0x34]
}

#[derive(Debug, Clone, PartialEq)]
pub enum ElementIdentifier {
  PartitionMajor{ value: u16 },
  PartitionMinor{ value: u16 },
  PartitionKagSize{ size: u32 },
  PartitionThisPartition{ offset: u64 },
  PartitionPreviousPartition{ offset: u64 },
  PartitionFooterPartition{ offset: u64 },
  PartitionHeaderByteCount{ size: u64 },
  PartitionIndexByteCount{ size: u64 },
  PartitionIndexSid{ value: u32 },
  PartitionByteOffset{ offset: u64 },
  PartitionBodySid{ value: u32 },
  PartitionOperationalPattern{
    item_complexity: u8,
    package_complexity: char,
    internal_essence: bool,
    stream_file: bool,
    uni_track: bool,
  },
  PartitionEssenceContainers{essences: Vec<EssenceIdentifier>},
  InstanceUid {uuid: Vec<u8>},
  // PackageUid {umid: Vec<u8>},
  GenerationUid {uuid: Vec<u8>},

  Data {data: Vec<u8>},
  ContentData {
    address: usize,
    size: usize
  },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Element {
  pub identifier: ElementIdentifier
}

#[derive(Debug, Clone, PartialEq)]
pub struct Value {
  pub elements: Vec<Element>
}

impl Encoder for Value {
  fn serialise(&self) -> Vec<u8> {
    let mut result = vec![];
    for element in self.elements.clone() {
      match element.identifier {
        ElementIdentifier::PartitionMajor{value} => {
          result.write_u16::<BigEndian>(value).unwrap();
        },
        ElementIdentifier::PartitionMinor{value} => {
          result.write_u16::<BigEndian>(value).unwrap();
        },
        ElementIdentifier::PartitionKagSize{size} => {
          result.write_u32::<BigEndian>(size).unwrap();
        },
        ElementIdentifier::PartitionThisPartition{offset} => {
          result.write_u64::<BigEndian>(offset).unwrap();
        },
        ElementIdentifier::PartitionPreviousPartition{offset} => {
          result.write_u64::<BigEndian>(offset).unwrap();
        },
        ElementIdentifier::PartitionFooterPartition{offset} => {
          result.write_u64::<BigEndian>(offset).unwrap();
        },
        ElementIdentifier::PartitionHeaderByteCount{size} => {
          result.write_u64::<BigEndian>(size).unwrap();
        },
        ElementIdentifier::PartitionIndexByteCount{size} => {
          result.write_u64::<BigEndian>(size).unwrap();
        },
        ElementIdentifier::PartitionIndexSid{value} => {
          result.write_u32::<BigEndian>(value).unwrap();
        },
        ElementIdentifier::PartitionByteOffset{offset} => {
          result.write_u64::<BigEndian>(offset).unwrap();
        },
        ElementIdentifier::PartitionBodySid{value} => {
          result.write_u32::<BigEndian>(value).unwrap();
        },
        ElementIdentifier::PartitionOperationalPattern{item_complexity, package_complexity, internal_essence, stream_file, uni_track} => {
          let mut op = serialise_operational_pattern(item_complexity, package_complexity, internal_essence, stream_file, uni_track);
          result.append(&mut op);
        },
        ElementIdentifier::PartitionEssenceContainers{essences} => {
          result.write_u32::<BigEndian>(essences.len() as u32).unwrap();
          result.write_u32::<BigEndian>(16).unwrap();
          for _ul in essences {
            // TODO wrote essence UL
          }
        },
        ElementIdentifier::InstanceUid{uuid} => {
          let tag = Tag{id: [0x3c, 0x0a], data: uuid};
          let mut content = Encoder::serialise(&tag);
          result.append(&mut content);
        },
        ElementIdentifier::GenerationUid{uuid} => {
          let tag = Tag{id: [0x01, 0x02], data: uuid};
          let mut content = Encoder::serialise(&tag);
          result.append(&mut content);
        },
        ElementIdentifier::Data{data} => {
          let mut d = data;
          result.append(&mut d);
        },
        ElementIdentifier::ContentData{..} => {
          panic!("unable to wrap a key based on Content Data kind. Missing data.");
        },
      }
    }
    result
  }
}

fn serialise_operational_pattern(item_complexity: u8, package_complexity: char, internal_essence: bool, stream_file: bool, uni_track: bool) -> Vec<u8> {
  let mut result = get_smpte_identifier();
  let mut op_identifier = vec![0x04, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01];

  result.append(&mut op_identifier);
  result.push(item_complexity);
  match package_complexity {
    'a' => result.push(0x01),
    'b' => result.push(0x02),
    'c' => result.push(0x03),
    _ => panic!("unsupported package complexity")
  };

  let mut flags = 0x01;
  flags += (uni_track as u8) << 4;
  flags += (stream_file as u8) << 3;
  flags += (internal_essence as u8) << 2;

  result.push(flags);
  result.push(0x00);
  result
}

pub fn parse_operational_pattern(data: Vec<u8>) -> Option<ElementIdentifier> {
  match (data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8], data[9], data[10], data[11], data[12], data[13], data[14], data[15]) {
    (0x06, 0x0e, 0x2b, 0x34, 0x04, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, item_complexity, package_complexity_value, flags, _reserved) => {

      let package_complexity =
        match package_complexity_value {
          0x01 => {'a'},
          0x02 => {'b'},
          0x03 => {'c'},
          _ => {' '}
        };

      Some(
        ElementIdentifier::PartitionOperationalPattern{
          item_complexity: item_complexity,
          package_complexity: package_complexity,
          internal_essence: ((flags & 0b00000010) == 0),
          stream_file: ((flags & 0b00000100) == 0),
          uni_track: ((flags & 0b00001000) == 0),
        }
      )
    },
    (_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
      None
    }
  }
}
