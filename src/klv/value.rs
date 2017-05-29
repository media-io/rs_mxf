
use byteorder::{BigEndian, WriteBytesExt};
use serializer::encoder::*;

fn get_smpte_identifier() -> Vec<u8> {
  vec![0x06, 0x0e, 0x2b, 0x34]
}

#[derive(Debug, Clone, PartialEq)]
pub enum ElementIdentifier {
  PartitionMajor,
  PartitionMinor,
  PartitionKagSize {size: u32},
  PartitionThisPartition {offset: u64},
  PartitionPreviousPartition {offset: u64},
  PartitionFooterPartition {offset: u64},
  PartitionHeaderByteCount {size: u64},
  PartitionIndexByteCount {index: u64},
  PartitionIndexSid,
  PartitionByteOffset,
  PartitionBodySid,
  PartitionOperationalPattern{item_complexity: u8, package_complexity: char},
  PartitionEssenceContainers,
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
        ElementIdentifier::PartitionMajor => {
          result.write_u16::<BigEndian>(1).unwrap();
        },
        ElementIdentifier::PartitionMinor => {
          result.write_u16::<BigEndian>(3).unwrap();
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
        ElementIdentifier::PartitionIndexByteCount{index} => {
          result.write_u64::<BigEndian>(index).unwrap();
        },
        ElementIdentifier::PartitionIndexSid => {
          result.write_u32::<BigEndian>(0).unwrap();
        },
        ElementIdentifier::PartitionByteOffset => {
          result.write_u64::<BigEndian>(0).unwrap();
        },
        ElementIdentifier::PartitionBodySid => {
          result.write_u32::<BigEndian>(0).unwrap();
        },
        ElementIdentifier::PartitionOperationalPattern{item_complexity, package_complexity} => {
          let mut op = serialise_operational_pattern(item_complexity, package_complexity);
          result.append(&mut op);
        },
        ElementIdentifier::PartitionEssenceContainers => {
          result.write_u64::<BigEndian>(0).unwrap();
          result.write_u64::<BigEndian>(0).unwrap();
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

fn serialise_operational_pattern(item_complexity: u8, package_complexity: char) -> Vec<u8> {
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

  result.push(0x00);
  result.push(0x00);
  result
}

pub struct Tag {
  pub id: [u8; 2],
  pub data: Vec<u8>
}

impl Encoder for Tag {
  fn serialise(&self) -> Vec<u8> {
    let mut result = vec![];
    let mut t = self.data.clone();
    result.push(self.id[0]);
    result.push(self.id[1]);

    result.write_u16::<BigEndian>(t.len() as u16).unwrap();
    result.append(&mut t);
    result
  }
}
