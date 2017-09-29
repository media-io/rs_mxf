
extern crate cheddar;

fn main() {
  cheddar::Cheddar::new().expect("could not read manifest")
    // .module("mxf").expect("malformed module path")
    .run_build("include/mxf.h");

  cheddar::Cheddar::new().expect("could not read manifest")
    .module("klv::ul").expect("malformed module path")
    .run_build("include/mxf/klv.h");

  cheddar::Cheddar::new().expect("could not read manifest")
    .module("klv::klv_reader").expect("malformed module path")
    .run_build("include/mxf/klv_reader.h");

  cheddar::Cheddar::new().expect("could not read manifest")
    .module("interface::writer").expect("malformed module path")
    .run_build("include/mxf/interface/writer.h");
}
