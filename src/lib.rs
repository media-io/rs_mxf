
extern crate byteorder;

pub mod klv;
pub mod serializer;

#[cfg(test)]
mod test {
  // use serializer::decoder::*;
  use serializer::encoder::*;
  use klv::klv::*;
  use klv::key::*;
  use klv::value::*;

  #[test]
  fn read_empty_file() {
    use std::io::Cursor;

    let mut stream = Cursor::new(vec![0; 0]);

    let result = next_klv(&mut stream);

    assert!(result.is_err());
    assert!(result.err() == Some("failed to fill whole buffer".to_string()));
  }

  #[test]
  fn read_first_key() {
    use std::io::Cursor;

    let data = vec![
      0x06, 0x0e, 0x2b, 0x34, 0x02, 0x05, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, 0x01, 0x02, 0x00, 0x00,
      0x83, 0x00, 0x00, 0x05,
      0x01, 0x02, 0x03, 0x04, 0x05,
    ];

    let mut stream = Cursor::new(data);

    let valid_klv = next_klv(&mut stream).unwrap().unwrap();
    
    assert_eq!(valid_klv, Klv {
      key: Key { identifier: KeyIdentifier::HeaderPartition },
      value: Value {
        elements: vec![
          Element {
            identifier: ElementIdentifier::ContentData {
              address: 20, size: 5
            }
          }
        ]
      }
    });
  }

  #[test]
  fn generate_file() {
    use std::io::prelude::*;
    use std::io::Cursor;

    let mut stream = Cursor::new(vec![0; 0]);

    let header_key = Key {
      identifier: KeyIdentifier::HeaderPartition
    };

    let header_value = Value {
      elements: vec![
        Element{
          identifier: ElementIdentifier::PartitionMajor
        },
        Element{
          identifier: ElementIdentifier::PartitionMinor
        },
        Element{
          identifier: ElementIdentifier::PartitionKagSize{size: 512}
        },
        Element{
          identifier: ElementIdentifier::PartitionThisPartition{offset: 0}
        },
        Element{
          identifier: ElementIdentifier::PartitionPreviousPartition{offset: 0}
        },
        Element{
          identifier: ElementIdentifier::PartitionFooterPartition{offset: 0}
        },
        Element{
          identifier: ElementIdentifier::PartitionHeaderByteCount{size: 0}
        },
        Element{
          identifier: ElementIdentifier::PartitionIndexByteCount{index: 0}
        },
        Element{
          identifier: ElementIdentifier::PartitionIndexSid
        },
        Element{
          identifier: ElementIdentifier::PartitionByteOffset
        },
        Element{
          identifier: ElementIdentifier::PartitionBodySid
        },
        Element{
          identifier: ElementIdentifier::PartitionOperationalPattern{item_complexity: 1, package_complexity: 'a'}
        },
        Element{
          identifier: ElementIdentifier::PartitionEssenceContainers
        },
      ]
    };

    let header_klv = Klv {
      key: header_key,
      value: header_value
    };

    stream.write(Encoder::serialise(&header_klv).as_ref()).unwrap();

    let static_track_key = Key {
      identifier: KeyIdentifier::StaticTrack
    };

    let static_track_value = Value {
      elements: vec![
        Element{
          identifier: ElementIdentifier::InstanceUid{uuid: vec![0x66, 0x66, 0x66, 0x66]}
        },
        Element{
          identifier: ElementIdentifier::GenerationUid{uuid: vec![0x66, 0x66, 0x66, 0x66]}
        },
      ]
    };

    let static_track_klv = Klv {
      key: static_track_key,
      value: static_track_value
    };

    stream.write(Encoder::serialise(&static_track_klv).as_ref()).unwrap();

    let frame_key = Key {
      identifier: KeyIdentifier::PictureItemMpegFrameWrappedPictureElement
    };

    let frame_value = Value {
      elements: vec![
        Element{
          identifier: ElementIdentifier::Data{data: vec![0x00; 2000]}
        }
      ]
    };

    let frame_klv = Klv {
      key: frame_key,
      value: frame_value
    };

    stream.write(Encoder::serialise(&frame_klv).as_ref()).unwrap();
    assert_eq!(stream.get_ref().len(), 2165);
  }
}
