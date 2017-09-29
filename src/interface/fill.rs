
use interface::writer::Writer;
use klv::klv::*;
use klv::ul::*;
use klv::value::*;
use klv::value::element::Element;
use klv::value::value_data::*;

use std::io::prelude::*;
use std::io::Cursor;
use serializer::encoder::*;

pub fn fill(writer: &Writer, prev_data_len: usize, alignement : usize) {

  let mut fill_size = prev_data_len % alignement;
  // if fill_size < 17 {

  // }
  fill_size -= 16 + 1;

  if fill_size > 0x7f {
    fill_size -= 2;
  }
  if fill_size > 0xffff {
    fill_size -= 1;
  }
  if fill_size > 0xffffff {
    fill_size -= 1;
  }

  let fill_data = vec![0; fill_size];

  let header_klv = Klv {
    key: Ul::FillItem,
    value: Value {
      elements: vec![
        build_element!(Ul::Unknown, unknown => fill_data)
      ]
    }
  };

  let mut stream = Cursor::new(vec![0; 0]);
  stream.write(Encoder::serialise(&header_klv).as_ref()).unwrap();

  let data = stream.get_mut();
  (writer.write)(writer.handle, data.as_mut_ptr(), data.len() as u64);
}
