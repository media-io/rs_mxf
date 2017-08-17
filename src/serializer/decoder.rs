
use std::io::{Error, Read};

pub trait Decoder {
  fn deserialize<R: Read>(&mut self, stream: &mut R) -> Result<bool, Error>;
}
