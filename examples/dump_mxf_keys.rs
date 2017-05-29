
extern crate mxf;

use std::io::BufReader;
use std::fs::File;
use std::process;
use std::env;

use mxf::klv::key::*;
use mxf::klv::klv::*;

fn main() {

  if env::args().count() != 2 {
    println!("ERROR: missing filepath argument.");
    println!("usage:");
    println!("       dump_mxf_keys [filepath.mxf]");
    process::exit(0x0f00);
  }

  let path = env::args().last().unwrap();

  let file = File::open(path).unwrap();
  let mut stream = BufReader::new(file);

  loop {
    let klv = next_klv(&mut stream).unwrap().unwrap();
    match klv.key.identifier {
      KeyIdentifier::FillItem |
      KeyIdentifier::SystemItemSystemMetadataPack |
      KeyIdentifier::SystemItemPackageMetadataSet |
      KeyIdentifier::SoundItemWaveDataWrappedSoundElement |
      KeyIdentifier::PictureItemMpegFrameWrappedPictureElement => {
      },
      _ => {
        println!("{:?}", klv);
      },
    }
    
  }
}
