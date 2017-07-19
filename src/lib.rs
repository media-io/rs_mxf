
extern crate byteorder;

pub mod klv;
pub mod serializer;

#[cfg(test)]
mod test {
  // use serializer::decoder::*;
  use serializer::encoder::*;
  use klv::klv::*;
  use klv::key::partition::*;
  use klv::key::dict::*;
  use klv::key::key::Key;
  use klv::value::value::*;
  use klv::value::essence_identifiers::*;
  use klv::value::value::Element;

  #[test]
  fn read_empty_file() {
    use std::io::Cursor;

    let mut stream = Cursor::new(vec![0; 0]);

    let result = next_klv(&mut stream);

    assert!(result.is_ok());
    assert!(result.unwrap() == None);
  }

  #[test]
  fn read_first_key() {
    use std::io::Cursor;

    let data = vec![
      0x06, 0x0e, 0x2b, 0x34, 0x02, 0x05, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, 0x01, 0x02, 0x04, 0x00,
      0x83, 0x00, 0x00, 0x68,
      0x00, 0x01, 0x00, 0x02, // version major and minor
      0x00, 0x00, 0x01, 0x00, // kag size
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // this partition
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // previous partition
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // footer partition
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // header byte count
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // index byte count
      0x00, 0x00, 0x00, 0x00, // index sid
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // body byte count
      0x00, 0x00, 0x00, 0x00, // body sid
      0x06, 0x0e, 0x2b, 0x34, 0x04, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, 0x01, 0x01, 0x09, 0x00, // operational pattern
      0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x10, // UL batch
      0x06, 0x0e, 0x2b, 0x34, 0x04, 0x01, 0x01, 0x02, 0x0d, 0x01, 0x03, 0x01, 0x02, 0x04, 0x97, 0x01, // Mpeg ES
    ];

    let mut stream = Cursor::new(data);

    let valid_klv = next_klv(&mut stream).unwrap().unwrap();
    
    assert_eq!(valid_klv, Klv {
      key: Key {
        identifier: KeyIdentifier::HeaderPartition{status: PartitionStatus::ClosedAndComplete}
      },
      value: Value {
        elements: vec![
          Element {
            identifier: ElementIdentifier::PartitionMajor {value: 1},
            value: None
          },
          Element {
            identifier: ElementIdentifier::PartitionMinor {value: 2},
            value: None
          },
          Element {
            identifier: ElementIdentifier::PartitionKagSize {size: 256},
            value: None
          },
          Element {
            identifier: ElementIdentifier::PartitionThisPartition {offset: 0},
            value: None
          },
          Element {
            identifier: ElementIdentifier::PartitionPreviousPartition {offset: 0},
            value: None
          },
          Element {
            identifier: ElementIdentifier::PartitionFooterPartition {offset: 0},
            value: None
          },
          Element {
            identifier: ElementIdentifier::PartitionHeaderByteCount {size: 0},
            value: None
          },
          Element {
            identifier: ElementIdentifier::PartitionIndexByteCount {size: 0},
            value: None
          },
          Element {
            identifier: ElementIdentifier::PartitionIndexSid {value: 0},
            value: None
          },
          Element {
            identifier: ElementIdentifier::PartitionByteOffset {offset: 0},
            value: None
          },
          Element {
            identifier: ElementIdentifier::PartitionBodySid {value: 0},
            value: None
          },
          Element {
            identifier: ElementIdentifier::PartitionOperationalPattern {
              item_complexity: 1,
              package_complexity: 'a',
              internal_essence: true,
              stream_file: true,
              uni_track: false
            },
            value: None
          },
          Element {
            identifier: ElementIdentifier::PartitionEssenceContainers {
              essences: vec![EssenceIdentifier::MpegEsWithStreamIdFrameWrapped]
            },
            value: None
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
      identifier: KeyIdentifier::HeaderPartition{status: PartitionStatus::ClosedAndComplete}
    };

    let header_value = Value {
      elements: vec![
        Element{
          identifier: ElementIdentifier::PartitionMajor{value: 1},
          value: None
        },
        Element{
          identifier: ElementIdentifier::PartitionMinor{value: 2},
          value: None
        },
        Element{
          identifier: ElementIdentifier::PartitionKagSize{size: 512},
          value: None
        },
        Element{
          identifier: ElementIdentifier::PartitionThisPartition{offset: 0},
          value: None
        },
        Element{
          identifier: ElementIdentifier::PartitionPreviousPartition{offset: 0},
          value: None
        },
        Element{
          identifier: ElementIdentifier::PartitionFooterPartition{offset: 0},
          value: None
        },
        Element{
          identifier: ElementIdentifier::PartitionHeaderByteCount{size: 0},
          value: None
        },
        Element{
          identifier: ElementIdentifier::PartitionIndexByteCount{size: 0},
          value: None
        },
        Element{
          identifier: ElementIdentifier::PartitionIndexSid{value: 0},
          value: None
        },
        Element{
          identifier: ElementIdentifier::PartitionByteOffset{offset: 0},
          value: None
        },
        Element{
          identifier: ElementIdentifier::PartitionBodySid{value: 0},
          value: None
        },
        Element{
          identifier: ElementIdentifier::PartitionOperationalPattern {
            item_complexity: 1,
            package_complexity: 'a',
            internal_essence: true,
            stream_file: true,
            uni_track: false
          },
          value: None
        },
        Element{
          identifier: ElementIdentifier::PartitionEssenceContainers { essences: vec![] },
          value: None
        },
      ]
    };

    let header_klv = Klv {
      key: header_key,
      value: header_value
    };

    stream.write(Encoder::serialise(&header_klv).as_ref()).unwrap();

    // let static_track_key = Key {
    //   identifier: KeyIdentifier::StaticTrackSet
    // };

    // let static_track_value = Value {
    //   elements: vec![
    //     Element{
    //       identifier: ElementIdentifier::InstanceUid,
    //       value: None
    //     },
    //     Element{
    //       identifier: ElementIdentifier::GenerationIdentifier,
    //       value: None
    //     },
    //   ]
    // };

    // let static_track_klv = Klv {
    //   key: static_track_key,
    //   value: static_track_value
    // };

    // stream.write(Encoder::serialise(&static_track_klv).as_ref()).unwrap();

    let frame_key = Key {
      identifier: KeyIdentifier::PictureItemMpegFrameWrappedPictureElement
    };

    let frame_value = Value {
      elements: vec![
        Element{
          identifier: ElementIdentifier::Data{data: vec![0x00; 2000]},
          value: None
        }
      ]
    };

    let frame_klv = Klv {
      key: frame_key,
      value: frame_value
    };

    stream.write(Encoder::serialise(&frame_klv).as_ref()).unwrap();
    assert_eq!(stream.get_ref().len(), 2124);
  }
}
