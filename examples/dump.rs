
extern crate mxf;

use std::io::BufReader;
use std::fs::File;
use std::{env, process};

use mxf::klv::klv::*;
use mxf::klv::klv_reader::*;
use mxf::klv::ul::*;

fn display_error() {
  println!("ERROR: missing filepath argument.");
  println!("usage:");
  println!("       dump [filepath.mxf]");
  process::exit(0x0f00);
}

fn main() {
  let mut filter_sound_wave = true;
  let mut filter_video_frame = true;

  let path =
    match env::args().count() {
      2 => {
        env::args().last().unwrap()
      },
      3 => {
        let arg1 = env::args().nth(1).unwrap();
        let arg2 = env::args().nth(2).unwrap();

        match (arg1.as_str(), arg2.as_str()) {
          ("-v", tmp_path) |
          (tmp_path, "-v") => {
            filter_video_frame = false;
            tmp_path.to_string()
          },
          ("-a", tmp_path) |
          (tmp_path, "-a") => {
            filter_sound_wave = false;
            tmp_path.to_string()
          },
          (_, _) => {
            display_error();
            panic!("unable to parse parameters");
          },
        }
      },
      4 => {
        let arg1 = env::args().nth(1).unwrap();
        let arg2 = env::args().nth(2).unwrap();
        let arg3 = env::args().nth(3).unwrap();

        match (arg1.as_str(), arg2.as_str(), arg3.as_str()) {
          ("-v", "-a", tmp_path) |
          ("-a", "-v", tmp_path) |
          ("-a", tmp_path, "-v") |
          ("-v", tmp_path, "-a") |
          (tmp_path, "-v", "-a") |
          (tmp_path, "-a", "-v") => {
            filter_video_frame = false;
            filter_sound_wave = false;
            tmp_path.to_string()
          },
          (_, _, _) => {
            display_error();
            panic!("unable to parse parameters");
          },
        }
      },
      _ => {
        display_error();
        panic!("Error");
      }
    };

  // println!("filter_video_frame {:?}", filter_video_frame);
  // println!("filter_sound_wave {:?}", filter_sound_wave);
  let file = File::open(path).unwrap();
  let stream = BufReader::new(file);

  let mut reader = KlvReader{
    stream: stream,
    elements: vec![]
  };

  loop {
    match next_klv(&mut reader) {
      Ok(maybe_klv) => {
        match maybe_klv {
          Some(klv) => {
            match (filter_video_frame, filter_sound_wave, klv.key.clone()) {
              (_, _, Ul::FillItem) |
              (_, _, Ul::FillItemAvid) |
              (_, _, Ul::IndexTableSegment) |
              (_, _, Ul::BodyPartition{..}) |
              (_, _, Ul::SystemItemSystemMetadataPack) |
              (_, _, Ul::SystemItemPackageMetadataSet) |
              (_, true, Ul::SoundItemWaveDataWrappedSoundElement) |
              (true, _, Ul::PictureItemMpegFrameWrappedPictureElement) |
              (true, _, Ul::Jpeg2000FrameWrapped) |
              (true, _, Ul::Jpeg2000ClipWrapped) => {
              },
              _ => {
                // println!("{:?}", klv);
                // if klv.value.elements.len() == 1 {
                //   match klv.value.elements[0].identifier {
                //     ContentData{..} => {
                //       println!("{:?}", klv);
                //     },
                //     _ => {}
                //   }
                // } else {
                //   // println!("{:?}", klv);
                // }
              },
            }
          },
          None => {
            break;
          },
        }
      },
      Err(msg) => {
        println!("ERROR: {:?}", msg);
        break;
      },
    }
    
  }
}
