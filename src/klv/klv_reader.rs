
use std::io::{Read, Seek};
use klv::value::value::DynamicTag;

#[derive(Debug, PartialEq)]
pub struct KlvReader<R: Read + Seek> {
  pub stream: R,
  pub elements: Vec<DynamicTag>
}
