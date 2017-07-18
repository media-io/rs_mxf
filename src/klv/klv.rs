
use serializer::encoder::*;

use klv::key::key::*;
use klv::key::dict::*;
use klv::key::reader::*;
use klv::length::*;
use klv::value::partition::*;
use klv::value::primer_pack::*;
use klv::value::set::*;
use klv::value::value::*;

use std::io::{Read, Seek, SeekFrom};

#[derive(Debug, PartialEq)]
pub struct Klv {
  pub key: Key,
  pub value: Value
}

impl Encoder for Klv {
  fn serialise(&self) -> Vec<u8> {
    let mut key_data = Encoder::serialise(&self.key);
    let mut value_data = Encoder::serialise(&self.value);

    let length = Length{value: value_data.len()};

    let mut length_data = Encoder::serialise(&length);

    key_data.append(&mut length_data);
    key_data.append(&mut value_data);
    key_data
  }
}

pub fn next_klv<R: Read + Seek>(mut stream: &mut R) -> Result<Option<Klv>, String>
{
  let mut identifier_data = vec![0; 16];
  match stream.read_exact(&mut identifier_data) {
    Ok(_some) => {},
    Err(_msg) => {
      // println!("{:?}", msg);
      return Ok(None)
    },
  };
  // try!(stream.read_exact(&mut identifier_data).map_err(|e| e.to_string()));
  let key = parse_key(identifier_data);
  let length = parse(&mut stream).unwrap().unwrap();
  let address = stream.seek(SeekFrom::Current(0)).unwrap();

  let identifier = ElementIdentifier::ContentData {
    address: address as usize,
    size: length.value
  };

  let elements =
    match key.identifier {
      KeyIdentifier::HeaderPartition{status: _} |
      KeyIdentifier::BodyPartition{status: _} |
      KeyIdentifier::FooterPartition{status: _} => {
        parse_partition(&mut stream).unwrap()
      },
      KeyIdentifier::PrimerPack => {
        parse_primer_pack(&mut stream).unwrap()
      },
      KeyIdentifier::PrefaceSet |
      KeyIdentifier::ContentStorageSet |
      KeyIdentifier::EssenceContainerDataSet |
      KeyIdentifier::MaterialPackageSet |
      KeyIdentifier::StaticTrackSet |
      KeyIdentifier::TrackSet |
      KeyIdentifier::SequenceSet |
      KeyIdentifier::SourceClipSet |
      KeyIdentifier::TimecodeComponentSet |
      KeyIdentifier::FilePackageSet |
      KeyIdentifier::DmSegmentDescriptorSet |
      KeyIdentifier::MultipleDescriptorSet |
      KeyIdentifier::MpegVideoDescriptorSet |
      KeyIdentifier::Aes3AudioDescriptorSet |
      KeyIdentifier::Jpeg2000SubDescriptorSet |
      KeyIdentifier::IdentificationSet |
      KeyIdentifier::CdciVideoDescriptor => {
        parse_set(&mut stream, length.value).unwrap()
      },
      KeyIdentifier::FillItemAvid => {
        let _new_address = stream.seek(SeekFrom::Current(length.value as i64)).unwrap();
        vec![]
      },
      _ => {
        let _new_address = stream.seek(SeekFrom::Current(length.value as i64)).unwrap();
        vec![
          Element{
            identifier: identifier,
            value: None
          }
        ]
      },
    };

  let value = Value{
    elements: elements
  };

  let klv = Klv{
    key: key,
    value: value
  };

  Ok(Some(klv))
}
