
use klv::ul::Ul;
use klv::value::value_data::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Element {
  pub identifier: Ul,
  pub value: Option<ValueData>,
}

#[macro_export]
macro_rules! build_element {
  ($ul:expr, uint16 => $value:expr) => (
    Element { identifier: $ul,
      value: Some( ValueData::Uint16{
        data: $value
      })
    }
  );
  ($ul:expr, uint32 => $value:expr) => (
    Element { identifier: $ul,
      value: Some( ValueData::Uint32{
        data: $value
      })
    }
  );
  ($ul:expr, uint64 => $value:expr) => (
    Element { identifier: $ul,
      value: Some( ValueData::Uint64{
        data: $value
      })
    }
  );
  ($ul:expr, ul => $value:expr) => (
    Element { identifier: $ul,
      value: Some( ValueData::Ul{
        data: $value
      })
    }
  );
  ($ul:expr, array_ul => $value:expr) => (
    Element { identifier: $ul,
      value: Some( ValueData::ArrayUl{
        data: $value
      })
    }
  );
  ($ul:expr, unknown => $value:expr) => (
    Element { identifier: $ul,
      value: Some( ValueData::Unknown{
        data: $value
      })
    }
  )
}
