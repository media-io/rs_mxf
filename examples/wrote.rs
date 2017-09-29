
extern crate mxf;

use mxf::interface::writer::*;
use std::fs::File;
use std::io::Write;

extern fn write(handle: *mut (), data: *mut u8, size: u64) -> bool {
  let file = unsafe{&mut *(handle as *mut File)};

  unsafe{
    let some_data = Vec::from_raw_parts(data, size as usize, size as usize);
    file.write_all(&some_data).unwrap();
  }

  true
}

fn main() {
  let file = File::create("my_file.mxf").unwrap();

  let box_file = Box::new(file);
  let handle = Box::into_raw(box_file) as *mut _;

  let mut w = Writer::new(handle, write);
  // let w = Writer{
  //   handle: handle,
  //   write: write
  // };

  w.add_stream(Codec::Jpeg2000, Wrapping::Frame);
  // w.add_stream(Codec::ProRes, Wrapping::Frame);

  let data = vec![66; 1000];

  w.dump_header();
  w.wrap(&data, 0);
  w.wrap(&data, 0);
  w.wrap(&data, 0);

  println!("done");
}
