
extern crate byteorder;
extern crate timecode;

#[macro_use] pub mod klv;
pub mod serializer;

#[cfg(test)]
mod test {
  use serializer::decoder::Decoder;
  use serializer::encoder::*;
  use klv::ul::Ul;
  use klv::klv::*;
  use klv::klv_reader::*;
  use klv::value::Value;
  use klv::value::value_data::*;
  use klv::value::partition::*;
  use klv::value::element::Element;

  #[test]
  fn read_empty_file() {
    use std::io::Cursor;

    let stream = Cursor::new(vec![0; 0]);
    let mut reader = KlvReader{
      stream: stream,
      elements: vec![]
    };

    let mut klv = Klv{..Default::default()};
    let result = klv.deserialize(&mut reader);

    assert!(result.is_ok());
    assert!(result.unwrap() == false);
  }

  #[test]
  fn read_first_key() {
    use std::io::Cursor;

    let data = vec![
      0x06, 0x0e, 0x2b, 0x34, 0x02, 0x05, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, 0x01, 0x02, 0x04, 0x00,
      0x83, 0x00, 0x00, 0x68,
      0x00, 0x01, 0x00, 0x02, // version major and minor
      0x00, 0x00, 0x02, 0x00, // kag size
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // this partition
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // previous partition
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // footer partition
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // header byte count
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // index byte count
      0x00, 0x00, 0x00, 0x00, // index sid
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // body byte count
      0x00, 0x00, 0x00, 0x00, // body sid
      0x06, 0x0e, 0x2b, 0x34, 0x04, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, 0x01, 0x01, 0x09, 0x00, // operational pattern
      0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x10, // UL batch
      0x06, 0x0e, 0x2b, 0x34, 0x04, 0x01, 0x01, 0x02, 0x0d, 0x01, 0x03, 0x01, 0x02, 0x04, 0x97, 0x01, // Mpeg ES
      0x06, 0x0e, 0x2b, 0x34, 0x04, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x03, 0x01, 0x02, 0x06, 0x03, 0x00, // Mpeg ES Audio
      0x06, 0x0e, 0x2b, 0x34, 0x04, 0x01, 0x01, 0x02, 0x0d, 0x01, 0x03, 0x01, 0x02, 0x04, 0x60, 0x01, // Mpeg ES Video
    ];

    let stream = Cursor::new(data);
    let mut reader = KlvReader{
      stream: stream,
      elements: vec![]
    };


    let mut klv = Klv{..Default::default()};
    let result = klv.deserialize(&mut reader);

    assert!(result.is_ok());
    assert!(result.unwrap() == true);

    assert_eq!(klv, Klv {
      key: Ul::HeaderPartition {
        status: Some(PartitionStatus::ClosedAndComplete)
      },
      value: Value {
        elements: vec![
          build_element!(Ul::PartitionMajor, uint16 => 1),
          build_element!(Ul::PartitionMinor, uint16 => 2),
          build_element!(Ul::PartitionKagSize, uint32 => 512),
          build_element!(Ul::PartitionThisPartition, uint64 => 0),
          build_element!(Ul::PartitionPreviousPartition, uint64 => 0),
          build_element!(Ul::PartitionFooterPartition, uint64 => 0),
          build_element!(Ul::PartitionHeaderByteCount, uint64 => 0),
          build_element!(Ul::PartitionIndexByteCount, uint64 => 0),
          build_element!(Ul::PartitionIndexSid, uint32 => 0),
          build_element!(Ul::PartitionBodyOffset, uint64 => 0),
          build_element!(Ul::PartitionBodySid, uint32 => 0),
          Element {
            identifier: Ul::PartitionOperationalPattern,
            value: Some(ValueData::Ul {
              data: Ul::MxfOP1aSingleItemSinglePackageMultiTrackStreamInternal
            })
          },
          Element {
            identifier: Ul::PartitionEssenceContainers,
            value: Some(ValueData::ArrayUl {
              data: vec![
                Ul::Essence_MpegEsWithStreamIdFrameWrapped,
                Ul::Essence_AesFrameWrapped
              ]
            })
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

    let header_value = Value {
      elements: vec![
        build_element!(Ul::PartitionMajor, uint16 => 1),
        build_element!(Ul::PartitionMinor, uint16 => 2),
        build_element!(Ul::PartitionKagSize, uint32 => 512),
        build_element!(Ul::PartitionThisPartition, uint64 => 0),
        build_element!(Ul::PartitionPreviousPartition, uint64 => 0),
        build_element!(Ul::PartitionFooterPartition, uint64 => 0),
        build_element!(Ul::PartitionHeaderByteCount, uint64 => 0),
        build_element!(Ul::PartitionIndexByteCount, uint64 => 0),
        build_element!(Ul::PartitionIndexSid, uint32 => 0),
        build_element!(Ul::PartitionBodyOffset, uint64 => 0),
        build_element!(Ul::PartitionBodySid, uint32 => 0),
        Element {
          identifier: Ul::PartitionOperationalPattern,
          value: Some(ValueData::Ul {
            data: Ul::MxfOP1aSingleItemSinglePackageMultiTrackStreamInternal
          })
        },
        Element {
          identifier: Ul::PartitionEssenceContainers,
          value: Some(ValueData::ArrayUl {
            data: vec![
              Ul::Essence_MpegEsWithStreamIdFrameWrapped,
              Ul::Essence_AesFrameWrapped
            ]
          })
        }
      ]
    };

    let header_klv = Klv {
      key: Ul::HeaderPartition {
        status: Some(PartitionStatus::ClosedAndComplete)
      },
      value: header_value
    };

    stream.write(Encoder::serialise(&header_klv).as_ref()).unwrap();

    // let frame_value = Value {
    //   elements: vec![
    //     Element{
    //       identifier: Ul::Unknown,
    //       value: None
    //     }
    //   ]
    // };

    // let frame_klv = Klv {
    //   key: Ul::PictureItemMpegFrameWrappedPictureElement,
    //   value: frame_value
    // };

    // stream.write(Encoder::serialise(&frame_klv).as_ref()).unwrap();

    println!("{:?}", stream.get_ref());
    assert_eq!(stream.get_ref().len(), 89);
  }
}
