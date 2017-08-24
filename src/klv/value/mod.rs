
#[macro_use] pub mod element;
pub mod partition;
pub mod primer_pack;
pub mod random_index_metadata;
pub mod set;
pub mod system_item;
pub mod tag;
pub mod value;
pub mod value_data;
pub mod value_data_type;

use klv::value::element::Element;

#[derive(Debug, Clone, PartialEq)]
pub struct Value {
  pub elements: Vec<Element>
}
