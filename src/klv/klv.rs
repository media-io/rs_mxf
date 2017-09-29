
use klv::ul::Ul;
use klv::value::Value;

#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct Klv {
  pub key: Ul,
  pub value: Value
}

impl Default for Klv {
  fn default() -> Klv {
    Klv {
      key: Ul::Unknown,
      value: Value{elements: vec![]}
    }
  }
}
