
use byteorder::{BigEndian, ReadBytesExt};

use klv::ul::*;
use klv::klv_reader::*;
use klv::value::value::*;
use std::io::{Read, Seek};
use std::str::from_utf8;

fn value_to_string_format<R: Read + Seek>(reader: &mut KlvReader<R>, size: usize) -> Result<String, String> {
  let mut value : String = "".to_string();

  let mut raw_data = vec![0; size as usize];
  try!(reader.stream.read_exact(&mut raw_data).map_err(|e| e.to_string()));

  for x in 0..size {
    value = format!("{}{:02x}", value, raw_data[x as usize]);
  };

  Ok(value)
}

pub fn parse_set<R: Read + Seek>(reader: &mut KlvReader<R>, size: usize) -> Result<Vec<Element>, String> {
  let mut readed_size = 0;
  let mut elements = vec![];

  let mut slice_count = None;
  let mut posistion_table_count = None;

  while readed_size < size {
    let tag_id = reader.stream.read_u16::<BigEndian>().unwrap();
    let tag_length = reader.stream.read_u16::<BigEndian>().unwrap();

    readed_size += tag_length as usize + 4;

    match get_tag_identifier(tag_id, &mut reader.elements) {
      Some((identifier, value_type)) => {

        let value = 
          match (value_type, tag_length) {
            (ValueDataType::Boolean, 1) => {
              let value = reader.stream.read_u8().unwrap();
              Some(ValueData::Boolean{
                data: value != 0
              })
            },
            (ValueDataType::Int8, 1) => {
              let value = reader.stream.read_i8().unwrap();
              Some(ValueData::Int8{
                data: value
              })
            },
            (ValueDataType::Int16, 2) => {
              let value = reader.stream.read_i16::<BigEndian>().unwrap();
              Some(ValueData::Int16{
                data: value
              })
            },
            (ValueDataType::Uint8, 1) => {
              let value = reader.stream.read_u8().unwrap();

              if identifier == Ul::SliceCount {
                slice_count = Some(value);
              }
              if identifier == Ul::PositionTableCount {
                posistion_table_count = Some(value);
              }

              Some(ValueData::Uint8{
                data: value
              })
            },
            (ValueDataType::Uint16, 2) => {
              let value = reader.stream.read_u16::<BigEndian>().unwrap();
              Some(ValueData::Uint16{
                data: value
              })
            },
            (ValueDataType::Uint32, 4) => {
              let value = reader.stream.read_u32::<BigEndian>().unwrap();
              Some(ValueData::Uint32{
                data: value
              })
            },
            (ValueDataType::Uint32X2, 16) => {
              let count = reader.stream.read_u32::<BigEndian>().unwrap();
              let size = reader.stream.read_u32::<BigEndian>().unwrap();

              if count != 2 && size != 4 {
                panic!("bad parameters for uint32x2 MXF type");
              }
              let value1 = reader.stream.read_u32::<BigEndian>().unwrap();
              let value2 = reader.stream.read_u32::<BigEndian>().unwrap();

              Some(ValueData::ArrayNumber{
                data: vec![value1 as u64, value2 as u64]
              })
            },
            (ValueDataType::Uint8Array, _) => {
              let count = reader.stream.read_u32::<BigEndian>().unwrap();
              let _size = reader.stream.read_u32::<BigEndian>().unwrap();
              let mut values = vec![];
              
              for _index in 0..count {
                let value = reader.stream.read_u8().unwrap();
                values.push(value as u64);
              }
              
              Some(ValueData::ArrayNumber{
                data: values
              })
            },
            (ValueDataType::BytesArray, _) => {
              let count = reader.stream.read_u32::<BigEndian>().unwrap();
              let size = reader.stream.read_u32::<BigEndian>().unwrap();
              let mut values = vec![];

              for _index in 0..count {
                let value = value_to_string_format(reader, size as usize).unwrap();
                values.push(value);
              }
              
              Some(ValueData::ArrayString{
                data: values
              })
            },
            (ValueDataType::Rational, 8) => {
              let num = reader.stream.read_u32::<BigEndian>().unwrap();
              let den = reader.stream.read_u32::<BigEndian>().unwrap();
              Some(ValueData::Rational{
                num: num as u64,
                den: den as u64
              })
            },
            (ValueDataType::Version, 2) => {
              let major = reader.stream.read_u8().unwrap();
              let minor = reader.stream.read_u8().unwrap();
              Some(ValueData::Version{
                major: major,
                minor: minor
              })
            },
            (ValueDataType::ProductVersion, 10) => {
              let major = reader.stream.read_u16::<BigEndian>().unwrap();
              let minor = reader.stream.read_u16::<BigEndian>().unwrap();
              let patch = reader.stream.read_u16::<BigEndian>().unwrap();
              let build = reader.stream.read_u16::<BigEndian>().unwrap();
              let release = reader.stream.read_u16::<BigEndian>().unwrap();

              Some(ValueData::ProductVersion {
                major: major,
                minor: minor,
                patch: patch,
                build: build,
                release: release,
              })
            },
            (ValueDataType::Position, 8) => {
              let value = reader.stream.read_u64::<BigEndian>().unwrap();
              Some(ValueData::Position{
                data: value
              })
            },
            (ValueDataType::Length, 8) => {
              let value = reader.stream.read_u64::<BigEndian>().unwrap();
              Some(ValueData::Length{
                data: value
              })
            },
            (ValueDataType::Uuid, 16) => {
              let value = value_to_string_format(reader, tag_length as usize).unwrap();
              Some(ValueData::Uuid{
                data: value
              })
            },
            (ValueDataType::Umid, 32) => {
              let value = value_to_string_format(reader, tag_length as usize).unwrap();
              Some(ValueData::Umid{
                data: value
              })
            },
            (ValueDataType::PackageId, 32) => {
              let value = value_to_string_format(reader, tag_length as usize).unwrap();
              Some(ValueData::PackageId{
                data: value
              })
            },
            (ValueDataType::Ul, 16) => {
              let mut ul_data = vec![0; 16];
              reader.stream.read_exact(&mut ul_data).unwrap();
              match match_ul(ul_data) {
                Some(ul) => Some(ValueData::Ul{data: ul}),
                None => None,
              }
            },
            (ValueDataType::StrongRef, 16) => {
              let value = value_to_string_format(reader, tag_length as usize).unwrap();
              Some(ValueData::StrongRef{
                data: value
              })
            },
            (ValueDataType::WeakRef, 16) => {
              let value = value_to_string_format(reader, tag_length as usize).unwrap();
              Some(ValueData::WeakRef{
                data: value
              })
            },
            (ValueDataType::StrongRefArray, _) |
            (ValueDataType::StrongRefBatch, _) |
            (ValueDataType::UlBatch, _) => {
              let count = reader.stream.read_u32::<BigEndian>().unwrap();
              let size = reader.stream.read_u32::<BigEndian>().unwrap();

              let mut array = vec![];

              for _index in 0..count {
                let strong_ref = value_to_string_format(reader, size as usize).unwrap();
                array.push(strong_ref);
              }

              Some(ValueData::ArrayString{
                data: array
              })
            },
            (ValueDataType::Timestamp, 8) => {
              let year = reader.stream.read_u16::<BigEndian>().unwrap();
              let month = reader.stream.read_u8().unwrap();
              let day = reader.stream.read_u8().unwrap();
              let hour = reader.stream.read_u8().unwrap();
              let minute = reader.stream.read_u8().unwrap();
              let second = reader.stream.read_u8().unwrap();
              let quarter_of_milliseconds = reader.stream.read_u8().unwrap();

              Some(ValueData::Timestamp{
                year: year,
                month: month,
                day: day,
                hour: hour,
                minute: minute,
                second: second,
                quarter_of_milliseconds: quarter_of_milliseconds,
              })
            },
            (ValueDataType::String, _) |
            (ValueDataType::Utf16, _) => {
              let mut string_data = vec![0; tag_length as usize];
              try!(reader.stream.read_exact(&mut string_data).map_err(|e| e.to_string()));

              let string = from_utf8(&string_data).unwrap().to_string();

              Some(ValueData::String{
                data: string
              })
            }
            (ValueDataType::ChannelLayout, 16) => {
              let mut layouts = vec![];
              for _i in 0..8 {
                let code = reader.stream.read_u8().unwrap();
                let bit_depth = reader.stream.read_u8().unwrap();
                if code != 0 {
                  layouts.push(Layout{
                    code: get_layout(code),
                    bit_depth: bit_depth
                  })
                }
              }
              Some(ValueData::ChannelLayout{
                data: layouts
              })
            },
            (ValueDataType::J2KComponentSizing, _) => {
              let count = reader.stream.read_u32::<BigEndian>().unwrap();
              let size = reader.stream.read_u32::<BigEndian>().unwrap();

              assert!(size == 3);
              let mut components = vec![];

              for _i in 0..count {
                let s_siz = reader.stream.read_u8().unwrap();
                let xr_siz = reader.stream.read_u8().unwrap();
                let yr_siz = reader.stream.read_u8().unwrap();

                components.push(J2KComponent{
                  s_siz: s_siz,
                  xr_siz: xr_siz,
                  yr_siz: yr_siz,
                })
              }
              Some(ValueData::J2KComponentSizing{
                components: components
              })
            },
            (ValueDataType::DeltaEntries, _) => {
              let count = reader.stream.read_u32::<BigEndian>().unwrap();
              let size = reader.stream.read_u32::<BigEndian>().unwrap();
              
              assert!(size == 6);
              let mut entries = vec![];

              for _i in 0..count {
                let position_table_index = reader.stream.read_i8().unwrap();
                let slice = reader.stream.read_u8().unwrap();
                let element_delta = reader.stream.read_u32::<BigEndian>().unwrap();

                entries.push(DeltaEntry{
                  position_table_index: position_table_index,
                  slice: slice,
                  element_delta: element_delta,
                })
              }

              Some(ValueData::DeltaEntries{
                entries: entries
              })
            }
            (ValueDataType::IndexEntries, _) => {
              let count = reader.stream.read_u32::<BigEndian>().unwrap();
              let _size = reader.stream.read_u32::<BigEndian>().unwrap();

              let mut entries = vec![];

              for _i in 0..count {
                let temporal_offset = reader.stream.read_i8().unwrap();
                let key_frame_offset = reader.stream.read_i8().unwrap();
                let flags = reader.stream.read_u8().unwrap();
                let stream_offset = reader.stream.read_u64::<BigEndian>().unwrap();

                let mut slice_offset = vec![];
                let mut position_table = vec![];
                match slice_count {
                  Some(count) => {
                    for _i in 0..count {
                      let offset = reader.stream.read_u32::<BigEndian>().unwrap();
                      slice_offset.push(offset);
                    }
                  },
                  None => {},
                }
                match posistion_table_count {
                  Some(count) => {
                    for _i in 0..count {
                      let num = reader.stream.read_u32::<BigEndian>().unwrap();
                      let den = reader.stream.read_u32::<BigEndian>().unwrap();
                      position_table.push(Rational{
                        num: num,
                        den: den
                      });
                    }
                  },
                  None => {},
                }

                entries.push(IndexEntry {
                  temporal_offset: temporal_offset,
                  key_frame_offset: key_frame_offset,
                  flags: flags,
                  stream_offset: stream_offset,
                  slice_offset: slice_offset,
                  position_table: position_table,
                })
              }
              None
            }
            (_, _) => {
              println!("unsupported {:?} for length {} with identifier {:?}", value_type, tag_length, identifier);
              let mut tag_data = vec![0; tag_length as usize];
              try!(reader.stream.read_exact(&mut tag_data).map_err(|e| e.to_string()));

              println!("{:?}", tag_data);
              None
            }
          };

        let element = Element {
          identifier: identifier,
          value: value
        };

        elements.push(element);
      },
      None => {
        // println!("Unknown tag identifier 0x{:04x}", tag_id);
        let mut tag_data = vec![0; tag_length as usize];
        try!(reader.stream.read_exact(&mut tag_data).map_err(|e| e.to_string()));
      },
    }
  }
  Ok(elements)
}
