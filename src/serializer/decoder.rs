
use std::io::{Read, BufReader};

pub trait Decoder {
  fn deserialize<Type, R: Read>(&self, stream: &mut BufReader<R>) -> Vec<Type>;
}
