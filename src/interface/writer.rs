
use interface::fill::fill;
use klv::klv::*;
use klv::ul::*;
use klv::value::*;
use klv::value::element::Element;
use klv::value::value_data::*;
use klv::value::partition::PartitionStatus::*;

use std::io::prelude::*;
use std::io::Cursor;
use serializer::encoder::*;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub enum Codec {
  Jpeg2000,
  AvcIntra,
  ProRes,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub enum Wrapping {
  Frame,
  Clip,
}

#[derive(Debug, Clone)]
pub struct Stream {
  pub codec: Codec,
  pub wrapping: Wrapping
}

#[derive(Debug, Clone)]
pub struct Properties {
  pub kag_size: Option<u32>,
  pub streams: Vec<Stream>,
}

#[derive(Clone)]
#[repr(C)]
pub struct Writer {
  pub properties: *mut (),
  pub handle: *mut (),
  pub write: extern fn(*mut(), *mut u8, u64) -> bool
}

impl Writer {
  pub fn new(handle: *mut(), write: extern fn(*mut(), *mut u8, u64) -> bool) -> Writer {

    let properties = Properties {
      kag_size: None,
      streams: vec![]
    };

    let box_properties = Box::new(properties);
    let handle_properties = Box::into_raw(box_properties) as *mut _;

    Writer {
      properties: handle_properties,
      handle: handle,
      write: write,
    }
  }

  pub fn add_stream(&mut self, codec: Codec, wrapping: Wrapping) {
    let mut properties : Box<Properties> = unsafe{ Box::from_raw(self.properties as *mut Properties) };
    
    properties.streams.push(Stream{
      codec: codec,
      wrapping: wrapping
    });

    self.properties = Box::into_raw(properties) as *mut _;
  }

  pub fn dump_header(&mut self) {
    let properties : Box<Properties> = unsafe{ Box::from_raw(self.properties as *mut Properties) };
    
    let header_klv = Klv {
      key: Ul::HeaderPartition {
        status: ClosedAndComplete
      },
      value: Value {
        elements: vec![
          build_element!(Ul::PartitionMajor, uint16 => 1),
          build_element!(Ul::PartitionMinor, uint16 => 3),
          build_element!(Ul::PartitionKagSize, uint32 => 512),
          build_element!(Ul::PartitionThisPartition, uint64 => 0),
          build_element!(Ul::PartitionPreviousPartition, uint64 => 0),
          build_element!(Ul::PartitionFooterPartition, uint64 => 131611136),
          build_element!(Ul::PartitionHeaderByteCount, uint64 => 5120),
          build_element!(Ul::PartitionIndexByteCount, uint64 => 0),
          build_element!(Ul::PartitionIndexSid, uint32 => 0),
          build_element!(Ul::PartitionBodyOffset, uint64 => 0),
          build_element!(Ul::PartitionBodySid, uint32 => 1),
          build_element!(Ul::PartitionOperationalPattern, ul => Ul::MxfOP1aSingleItemSinglePackageUniTrackStreamInternal),
          build_element!(Ul::PartitionEssenceContainers, array_ul => vec![Ul::Essence_Jpeg2000_FrameWrapped]),
        ]
      }
    };

    let mut stream = Cursor::new(vec![0; 0]);
    stream.write(Encoder::serialise(&header_klv).as_ref()).unwrap();

    let data = stream.get_mut();

    (self.write)(self.handle, data.as_mut_ptr(), data.len() as u64);

    match properties.kag_size {
      Some(kag_size) => {
        fill(&self, data.len(), kag_size as usize);
      },
      None => {},
    }

    let primer_pack_klv = Klv {
      key: Ul::PrimerPack,
      value: Value {
        elements: vec![
          build_element!(Ul::Unknown, uint32 => 0),
          build_element!(Ul::Unknown, uint32 => 18),
        ]
      }
    };

    let mut stream = Cursor::new(vec![0; 0]);
    stream.write(Encoder::serialise(&primer_pack_klv).as_ref()).unwrap();

    let data = stream.get_mut();

    (self.write)(self.handle, data.as_mut_ptr(), data.len() as u64);


    self.properties = Box::into_raw(properties) as *mut _;
  }

  pub fn wrap(&mut self, data: &Vec<u8>, stream_id: u8) -> bool {

    let properties : Box<Properties> = unsafe{ Box::from_raw(self.properties as *mut Properties) };
    if stream_id as usize >= properties.streams.len() {
      return false
    }

    {
      let ref stream_properties = &properties.streams[stream_id as usize];
      let ul =
        match (stream_properties.codec, stream_properties.wrapping) {
          (Codec::Jpeg2000, Wrapping::Frame) => Ul::Jpeg2000FrameWrapped,
          (Codec::Jpeg2000, Wrapping::Clip) => Ul::Jpeg2000ClipWrapped,
          (_, _) => unimplemented!(),
        };
      let header_klv = Klv {
        key: ul,
        value: Value {
          elements: vec![
            build_element!(Ul::Unknown, unknown => data.to_vec())
          ]
        }
      };

      let mut stream = Cursor::new(vec![0; 0]);
      stream.write(Encoder::serialise(&header_klv).as_ref()).unwrap();

      let data = stream.get_mut();
      (self.write)(self.handle, data.as_mut_ptr(), data.len() as u64);
    }
    self.properties = Box::into_raw(properties) as *mut _;

    true
  }
}

#[no_mangle]
pub extern fn new_writer(handle: *mut(), writer: extern fn(*mut(), *mut u8, u64) -> bool) -> Writer {
  Writer::new(handle, writer)
}

#[no_mangle]
pub extern fn add_stream(mut writer: Writer, codec: Codec, wrapping: Wrapping) {
  writer.add_stream(codec, wrapping);
}

#[no_mangle]
pub extern fn dump_header(mut writer: Writer) {
  writer.dump_header();
}

#[no_mangle]
pub extern fn wrap(mut writer: Writer, data: *mut u8, size: usize, stream_id: u8) -> bool {
  unsafe {
    let mut some_data = Vec::from_raw_parts(data, size, size);
    writer.wrap(&mut some_data, stream_id)
  }
}
