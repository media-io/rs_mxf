
use serializer::decoder::*;

use klv::ul::*;
use klv::klv::Klv;
use klv::klv_reader::*;
use klv::length::*;
use klv::value::Value;
use klv::value::partition::*;
use klv::value::primer_pack::*;
use klv::value::random_index_metadata::*;
use klv::value::set::*;
use klv::value::system_item::*;
use klv::value::element::Element;
use klv::value::value_data::*;

use std::io::{Error, ErrorKind, Read, Seek, SeekFrom};

impl Decoder for Klv {
  fn deserialize<R: Read + Seek>(&mut self, mut reader: &mut KlvReader<R>) -> Result<bool, Error> {
    
    let mut ul = Ul::Unknown;
    match ul.deserialize(&mut reader) {
      Ok(true) => {},
      Ok(false) => {
        return Ok(false)
      },
      Err(msg) => return Err(msg)
    }

    let mut length = Length{ ..Default::default() };
    match length.deserialize(&mut reader) {
      Ok(true) => {},
      Ok(false) => {
        return Err(Error::new(ErrorKind::UnexpectedEof, "Unable to read length of the KLV element"))
      },
      Err(msg) => return Err(msg)
    }

    let address = reader.stream.seek(SeekFrom::Current(0)).unwrap();

    let elements =
      match ul {
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
        Ul::SystemItemSystemMetadataPack => {
          parse_system_item_system(&mut reader, length.value).unwrap()
        },
        Ul::SystemItemPackageMetadataSet => {
          parse_system_item_package(&mut reader, length.value).unwrap()
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

    self.key = ul;
    self.value = value;

    Ok(true)
  }
}
