
#[derive(Debug)]
pub struct Length {
  pub value: usize
}

impl Default for Length {
  fn default() -> Length {
    Length {
      value: 0,
    }
  }
}

pub mod encoder;
pub mod decoder;
