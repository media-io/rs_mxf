
use std::io::{Read, Seek};
use std::io::BufReader;
use klv::value::value_data::DynamicTag;

#[derive(Debug)]
// #[repr(C)]
pub struct KlvReader<R: Read + Seek> {
  pub stream: BufReader<R>,
  // pub stream: Box<Read>,
  pub elements: Vec<DynamicTag>
}
