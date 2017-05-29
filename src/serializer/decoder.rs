
use std::io::{Read};

pub trait Decoder {
  fn deserialize<Type, R: Read>(&self, stream: &mut R) -> Vec<Type>;
}
