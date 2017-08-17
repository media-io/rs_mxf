
use klv::klv_reader::KlvReader;
use std::io::{Error, Read, Seek};

pub trait Decoder {
  fn deserialize<R: Read + Seek>(&mut self, stream: &mut KlvReader<R>) -> Result<bool, Error>;
}
