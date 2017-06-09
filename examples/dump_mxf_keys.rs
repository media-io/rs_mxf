
extern crate mxf;

use std::io::BufReader;
use std::fs::File;
// use std::process;
use std::env;

use mxf::klv::key::*;
use mxf::klv::klv::*;

// fn display_error() {
//   println!("ERROR: missing filepath argument.");
//   println!("usage:");
//   println!("       dump_mxf_keys [filepath.mxf]");
//   process::exit(0x0f00);
// }

fn main() {

  if env::args().count() != 2 {
    
  }

  let filter_sound_wave = true;
  let filter_mpeg_frame = true;

  let path = env::args().last().unwrap();
  // let path =
  //   match env::args().count() {
  //     2 => {
  //       env::args().last().unwrap();
  //     },
  //     3 => {
  //       match env::args().nth(1).unwrap().as_ref() {
  //         "-v" => {
  //           filter_mpeg_frame = false;
  //           env::args().nth(2).unwrap();
  //         },
  //         "-a" => {
  //           filter_sound_wave = false;
  //           env::args().nth(2).unwrap();
  //         },
  //         _ => {
  //           env::args().nth(1).unwrap();
  //         },
  //       }
  //       match env::args().nth(2).unwrap().as_ref() {
  //         "-v" => {
  //           filter_mpeg_frame = false;
  //           env::args().nth(1).unwrap();
  //         },
  //         "-a" => {
  //           filter_sound_wave = false;
  //           env::args().nth(1).unwrap();
  //         },
  //         _ => {
  //           env::args().nth(2).unwrap();
  //         },
  //       }
  //     },
  //     _ => {
  //       display_error();
  //       panic!("Error");
  //     }
  //   };


  // println!("filter_mpeg_frame {:?}", filter_mpeg_frame);
  let file = File::open(path).unwrap();
  let mut stream = BufReader::new(file);

  loop {
    let klv = next_klv(&mut stream).unwrap().unwrap();
    match (filter_mpeg_frame, filter_sound_wave, klv.key.identifier.clone()) {
      (_, _, KeyIdentifier::FillItem) |
      (_, _, KeyIdentifier::SystemItemSystemMetadataPack) |
      (_, _, KeyIdentifier::SystemItemPackageMetadataSet) |
      (_, true, KeyIdentifier::SoundItemWaveDataWrappedSoundElement) |
      (true, _, KeyIdentifier::PictureItemMpegFrameWrappedPictureElement) => {
      },
      _ => {
        println!("{:?}", klv);
      },
    }
    
  }
}
