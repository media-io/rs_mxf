
use serializer::encoder::*;

use klv::ul::ul::*;
use klv::klv_reader::*;
use klv::length::*;
use klv::value::partition::*;
use klv::value::primer_pack::*;
use klv::value::random_index_metadata::*;
use klv::value::set::*;
use klv::value::value::*;
use klv::value::element::Element;
use klv::value::value_data::*;

use std::io::{Read, Seek, SeekFrom};

#[derive(Debug, PartialEq)]
pub struct Klv {
  pub key: Ul,
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

pub fn next_klv<R: Read + Seek>(mut reader: &mut KlvReader<R>) -> Result<Option<Klv>, String>
{
  let mut identifier_data = vec![0; 16];
  match reader.stream.read_exact(&mut identifier_data) {
    Ok(_some) => {},
    Err(_msg) => {
      return Ok(None)
    },
  };
  
  let detected_key = match_ul(identifier_data);
  let length = parse(&mut reader.stream).unwrap().unwrap();
  let address = reader.stream.seek(SeekFrom::Current(0)).unwrap();

  let key =
    match detected_key {
        Some(ul) => {ul},
        None => {
          let _address = reader.stream.seek(SeekFrom::Current(length.value as i64)).unwrap();
          let klv = Klv{
            key: Ul::Unknown,
            value: Value{
              elements: vec![]
            }
          };
          return Ok(Some(klv));
        },
    };

  let elements =
    match key {
      Ul::HeaderPartition{..} |
      Ul::BodyPartition{..} |
      Ul::FooterPartition{..} => {
        parse_partition(&mut reader.stream).unwrap()
      },
      Ul::PrimerPack => {
        parse_primer_pack(&mut reader).unwrap()
      },
      Ul::RandomIndexMetadata => {
        parse_random_index_metadata(&mut reader, length.value).unwrap()
      },
      Ul::AS10CoreFramework => {
        let mut data = vec![0; length.value];
        let _new_address = reader.stream.read(&mut data).unwrap();
        println!("AS10CoreFramework {:?}", data);
        vec![
          Element{
            identifier: Ul::AS10CoreFramework,
            value: Some(ValueData::ContentData{
              address: address,
              size: length.value,
            })
          }
        ]
      },
      Ul::IndexTableSegment |
      Ul::PrefaceSet |
      Ul::ContentStorageSet |
      Ul::EssenceContainerDataSet |
      Ul::MaterialPackageSet |
      Ul::StaticTrackSet |
      Ul::TrackSet |
      Ul::SequenceSet |
      Ul::SourceClipSet |
      Ul::TimecodeComponentSet |
      Ul::FilePackageSet |
      Ul::DmSegmentDescriptorSet |
      Ul::MultipleDescriptorSet |
      Ul::MpegVideoDescriptorSet |
      Ul::WaveAudioDescriptorSet |
      Ul::Aes3AudioDescriptorSet |
      Ul::Jpeg2000SubDescriptorSet |
      Ul::SoundfieldGroupLabelSubDescriptorSet |
      Ul::AudioChannelLabelSubDescriptorSet |
      Ul::IdentificationSet |
      Ul::RgbaVideoDescriptor |
      Ul::CdciVideoDescriptor => {
        parse_set(&mut reader, length.value).unwrap()
      },
      Ul::FillItemAvid => {
        let _new_address = reader.stream.seek(SeekFrom::Current(length.value as i64)).unwrap();
        vec![]
      },
      _ => {
        let _new_address = reader.stream.seek(SeekFrom::Current(length.value as i64)).unwrap();
        vec![
          Element{
            identifier: Ul::Unknown,
            value: Some(ValueData::ContentData{
              address: address,
              size: length.value,
            })
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
