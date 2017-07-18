
use byteorder::{BigEndian, ReadBytesExt};

use klv::value::value::*;
use std::io::Read;
use std::str::from_utf8;

fn value_to_string_format<R: Read>(stream: &mut R, size: usize) -> Result<String, String> {
  let mut value : String = "".to_string();

  let mut raw_data = vec![0; size as usize];
  try!(stream.read_exact(&mut raw_data).map_err(|e| e.to_string()));

  for x in 0..size {
    value = format!("{}{:02x}", value, raw_data[x as usize]);
  };

  Ok(value)
}

pub fn parse_set<R: Read>(stream: &mut R, size: usize) -> Result<Vec<Element>, String> {
  let mut readed_size = 0;
  let mut elements = vec![];

  while readed_size < size {
    let tag_id = stream.read_u16::<BigEndian>().unwrap();
    let tag_length = stream.read_u16::<BigEndian>().unwrap();

    readed_size += tag_length as usize + 4;

    match get_tag_identifier(tag_id) {
      Some((identifier, value_type)) => {

        let value = 
          match (value_type, tag_length) {
            (ValueDataType::Boolean, 1) => {
              let value = stream.read_u8().unwrap();
              Some(ValueData::Boolean{
                data: value != 0
              })
            },
            (ValueDataType::Int8, 1) => {
              let value = stream.read_i8().unwrap();
              Some(ValueData::Int8{
                data: value
              })
            },
            (ValueDataType::Int16, 2) => {
              let value = stream.read_i16::<BigEndian>().unwrap();
              Some(ValueData::Int16{
                data: value
              })
            },
            (ValueDataType::Uint8, 1) => {
              let value = stream.read_u8().unwrap();
              Some(ValueData::Uint8{
                data: value
              })
            },
            (ValueDataType::Uint16, 2) => {
              let value = stream.read_u16::<BigEndian>().unwrap();
              Some(ValueData::Uint16{
                data: value
              })
            },
            (ValueDataType::Uint32, 4) => {
              let value = stream.read_u32::<BigEndian>().unwrap();
              Some(ValueData::Uint32{
                data: value
              })
            },
            (ValueDataType::Uint32X2, 16) => {
              let count = stream.read_u32::<BigEndian>().unwrap();
              let size = stream.read_u32::<BigEndian>().unwrap();

              if count != 2 && size != 4 {
                panic!("bad parameters for uint32x2 MXF type");
              }
              let value1 = stream.read_u32::<BigEndian>().unwrap();
              let value2 = stream.read_u32::<BigEndian>().unwrap();

              Some(ValueData::ArrayNumber{
                data: vec![value1 as u64, value2 as u64]
              })
            },
            (ValueDataType::Uint8Array, _) => {
              let count = stream.read_u32::<BigEndian>().unwrap();
              let size = stream.read_u32::<BigEndian>().unwrap();
              let mut values = vec![];
              
              for _index in 0..count {
                let value = stream.read_u8().unwrap();
                values.push(value as u64);
              }
              
              Some(ValueData::ArrayNumber{
                data: values
              })
            },
            (ValueDataType::BytesArray, _) => {
              let count = stream.read_u32::<BigEndian>().unwrap();
              let size = stream.read_u32::<BigEndian>().unwrap();
              let mut values = vec![];

              for _index in 0..count {
                let value = value_to_string_format(stream, size as usize).unwrap();
                values.push(value);
              }
              
              Some(ValueData::ArrayString{
                data: values
              })
            },
            (ValueDataType::Rational, 8) => {
              let num = stream.read_u32::<BigEndian>().unwrap();
              let den = stream.read_u32::<BigEndian>().unwrap();
              Some(ValueData::Rational{
                num: num as u64,
                den: den as u64
              })
            },
            (ValueDataType::Version, 2) => {
              let major = stream.read_u8().unwrap();
              let minor = stream.read_u8().unwrap();
              Some(ValueData::Version{
                major: major,
                minor: minor
              })
            },
            (ValueDataType::ProductVersion, 10) => {
              let major = stream.read_u16::<BigEndian>().unwrap();
              let minor = stream.read_u16::<BigEndian>().unwrap();
              let patch = stream.read_u16::<BigEndian>().unwrap();
              let build = stream.read_u16::<BigEndian>().unwrap();
              let release = stream.read_u16::<BigEndian>().unwrap();

              Some(ValueData::ProductVersion {
                major: major,
                minor: minor,
                patch: patch,
                build: build,
                release: release,
              })
            },
            (ValueDataType::Position, 8) => {
              let value = stream.read_u64::<BigEndian>().unwrap();
              Some(ValueData::Position{
                data: value
              })
            },
            (ValueDataType::Length, 8) => {
              let value = stream.read_u64::<BigEndian>().unwrap();
              Some(ValueData::Length{
                data: value
              })
            },
            (ValueDataType::Uuid, 16) => {
              let value = value_to_string_format(stream, tag_length as usize).unwrap();
              Some(ValueData::Uuid{
                data: value
              })
            },
            (ValueDataType::Umid, 32) => {
              let value = value_to_string_format(stream, tag_length as usize).unwrap();
              Some(ValueData::Umid{
                data: value
              })
            },
            (ValueDataType::PackageId, 32) => {
              let value = value_to_string_format(stream, tag_length as usize).unwrap();
              Some(ValueData::PackageId{
                data: value
              })
            },
            (ValueDataType::Ul, 16) => {
              let value = value_to_string_format(stream, tag_length as usize).unwrap();
              Some(ValueData::Ul{
                data: value
              })
            },
            (ValueDataType::StrongRef, 16) => {
              let value = value_to_string_format(stream, tag_length as usize).unwrap();
              Some(ValueData::StrongRef{
                data: value
              })
            },
            (ValueDataType::WeakRef, 16) => {
              let value = value_to_string_format(stream, tag_length as usize).unwrap();
              Some(ValueData::WeakRef{
                data: value
              })
            },
            (ValueDataType::StrongRefArray, _) |
            (ValueDataType::StrongRefBatch, _) |
            (ValueDataType::UlBatch, _) => {
              let count = stream.read_u32::<BigEndian>().unwrap();
              let size = stream.read_u32::<BigEndian>().unwrap();

              let mut array = vec![];

              for _index in 0..count {
                let strong_ref = value_to_string_format(stream, size as usize).unwrap();
                array.push(strong_ref);
              }

              Some(ValueData::ArrayString{
                data: array
              })
            },
            (ValueDataType::Timestamp, 8) => {
              let year = stream.read_u16::<BigEndian>().unwrap();
              let month = stream.read_u8().unwrap();
              let day = stream.read_u8().unwrap();
              let hour = stream.read_u8().unwrap();
              let minute = stream.read_u8().unwrap();
              let second = stream.read_u8().unwrap();
              let quarter_of_milliseconds = stream.read_u8().unwrap();

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
              try!(stream.read_exact(&mut string_data).map_err(|e| e.to_string()));

              let string = from_utf8(&string_data).unwrap().to_string();

              Some(ValueData::String{
                data: string
              })
            }
            (_, _) => {
              println!("unsupported {:?} for length {} with identifier {:?}", value_type, tag_length, identifier);
              let mut tag_data = vec![0; tag_length as usize];
              try!(stream.read_exact(&mut tag_data).map_err(|e| e.to_string()));

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
        let mut tag_data = vec![0; tag_length as usize];
        try!(stream.read_exact(&mut tag_data).map_err(|e| e.to_string()));
      },
    }
  }
  Ok(elements)
}
