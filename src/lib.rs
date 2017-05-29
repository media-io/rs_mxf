
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

  // Require sample file on CI platform
  // #[test]
  // fn read_file() {
  //   use std::fs::File;
  //   use std::io::BufReader;

  //   let file = File::open("/Users/marco/Movies/PAL_1080i_MPEG_XDCAM-HD_colorbar.mxf").unwrap();
  //   let mut stream = BufReader::new(file);

  //   let mut count = 0;
  //   let mut done = false;

  //   while !done {
  //     let result = next_klv(&mut stream);
  //     match result {
  //       Ok(valid_klv) => {
  //         match valid_klv {
  //           Some(_klv) => {
  //             // println!("{:?}", klv);
  //             count += 1;
  //           },
  //           None => {
  //             println!("None");
  //             done = true;
  //           }
  //         }
  //       },
  //       Err(msg) => {
  //         println!("{:?}", msg);
  //         done = true;
  //       },
  //     }
  //   }
  //   println!("{:?}", count);
  //   assert!(count == 9908);
  // }

  #[test]
  fn generate_file() {
    use std::io::prelude::*;

    use std::io::Cursor;
    let mut buffer = Cursor::new(vec![0; 0]);

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

    buffer.write(Encoder::serialise(&header_klv).as_ref()).unwrap();

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

    buffer.write(Encoder::serialise(&static_track_klv).as_ref()).unwrap();

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

    buffer.write(Encoder::serialise(&frame_klv).as_ref()).unwrap();
    assert_eq!(buffer.get_ref().len(), 2165);
  }
}
