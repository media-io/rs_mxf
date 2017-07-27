
extern crate byteorder;

pub mod klv;
pub mod serializer;

#[cfg(test)]
mod test {
  // use serializer::decoder::*;
  // use serializer::encoder::*;
  use klv::ul::*;
  use klv::klv::*;
  use klv::klv_reader::*;
  use klv::value::value::*;
  use klv::value::value::Element;

  #[test]
  fn read_empty_file() {
    use std::io::Cursor;

    let stream = Cursor::new(vec![0; 0]);
    let mut reader = KlvReader{
      stream: stream,
      elements: vec![]
    };

    let result = next_klv(&mut reader);

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

    let stream = Cursor::new(data);
    let mut reader = KlvReader{
      stream: stream,
      elements: vec![]
    };

    let valid_klv = next_klv(&mut reader).unwrap().unwrap();
    
    assert_eq!(valid_klv, Klv {
      key: Ul::HeaderPartition,
      value: Value {
        elements: vec![
          Element {
            identifier: Ul::PartitionMajor,
            value: Some(ValueData::Uint16{ data: 1 })
          },
          Element {
            identifier: Ul::PartitionMinor,
            value: Some(ValueData::Uint16{ data: 2 })
          },
          Element {
            identifier: Ul::PartitionKagSize,
            value: Some(ValueData::Uint32{ data: 256 })
          },
          Element {
            identifier: Ul::PartitionThisPartition,
            value: Some(ValueData::Uint64{ data: 0 })
          },
          Element {
            identifier: Ul::PartitionPreviousPartition,
            value: Some(ValueData::Uint64{ data: 0 })
          },
          Element {
            identifier: Ul::PartitionFooterPartition,
            value: Some(ValueData::Uint64{ data: 0 })
          },
          Element {
            identifier: Ul::PartitionHeaderByteCount,
            value: Some(ValueData::Uint64{ data: 0 })
          },
          Element {
            identifier: Ul::PartitionIndexByteCount,
            value: Some(ValueData::Uint64{ data: 0 })
          },
          Element {
            identifier: Ul::PartitionIndexSid,
            value: Some(ValueData::Uint32{ data: 0 })
          },
          Element {
            identifier: Ul::PartitionByteOffset,
            value: Some(ValueData::Uint64{ data: 0 })
          },
          Element {
            identifier: Ul::PartitionBodySid,
            value: Some(ValueData::Uint32{ data: 0 })
          },
          // Element {
          //   identifier: Ul::PartitionOperationalPattern,
          //   value: None
          // },
          Element {
            identifier: Ul::PartitionEssenceContainers,
            value: Some(ValueData::ArrayUl {
              data: vec![Ul::Essence_MpegEsWithStreamIdFrameWrapped]
            })
          }
        ]
      }
    });
  }

  // #[test]
  // fn generate_file() {
  //   use std::io::prelude::*;
  //   use std::io::Cursor;

  //   let mut stream = Cursor::new(vec![0; 0]);

  //   let header_value = Value {
  //     elements: vec![
  //       Element{
  //         identifier: Ul::PartitionMajor,
  //         value: None
  //       },
  //       Element{
  //         identifier: Ul::PartitionMinor,
  //         value: None
  //       },
  //       Element{
  //         identifier: Ul::PartitionKagSize,
  //         value: None
  //       },
  //       Element{
  //         identifier: Ul::PartitionThisPartition,
  //         value: None
  //       },
  //       Element{
  //         identifier: Ul::PartitionPreviousPartition,
  //         value: None
  //       },
  //       Element{
  //         identifier: Ul::PartitionFooterPartition,
  //         value: None
  //       },
  //       Element{
  //         identifier: Ul::PartitionHeaderByteCount,
  //         value: None
  //       },
  //       Element{
  //         identifier: Ul::PartitionIndexByteCount,
  //         value: None
  //       },
  //       Element{
  //         identifier: Ul::PartitionIndexSid,
  //         value: None
  //       },
  //       Element{
  //         identifier: Ul::PartitionByteOffset,
  //         value: None
  //       },
  //       Element{
  //         identifier: Ul::PartitionBodySid,
  //         value: None
  //       },
  //       Element{
  //         identifier: Ul::PartitionOperationalPattern,
  //         value: None
  //       },
  //       Element{
  //         identifier: Ul::PartitionEssenceContainers,
  //         value: None
  //       },
  //     ]
  //   };

  //   let header_klv = Klv {
  //     key: Ul::HeaderPartition,
  //     value: header_value
  //   };

  //   stream.write(Encoder::serialise(&header_klv).as_ref()).unwrap();

  //   let frame_value = Value {
  //     elements: vec![
  //       Element{
  //         identifier: Ul::Unknown,
  //         value: None
  //       }
  //     ]
  //   };

  //   let frame_klv = Klv {
  //     key: Ul::PictureItemMpegFrameWrappedPictureElement,
  //     value: frame_value
  //   };

  //   stream.write(Encoder::serialise(&frame_klv).as_ref()).unwrap();
  //   assert_eq!(stream.get_ref().len(), 2124);
  // }
}
