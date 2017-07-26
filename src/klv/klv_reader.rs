
use std::io::{Read, Seek};
use klv::value::tag::DynamicTagList;

#[derive(Debug, PartialEq)]
pub struct KlvReader<R: Read + Seek> {
  pub stream: R,
  pub elements: Vec<DynamicTagList>
}
