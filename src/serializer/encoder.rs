
pub trait Encoder {
  fn serialise(&self) -> Vec<u8>;
}
