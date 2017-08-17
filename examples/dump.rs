
extern crate mxf;

use std::io::BufReader;
use std::fs::File;
use std::{env, process};

use mxf::klv::klv::*;
use mxf::klv::klv_reader::*;
use mxf::klv::ul::Ul;
use mxf::serializer::decoder::Decoder;

fn display_error() {
  println!("ERROR: missing filepath argument.");
  println!("usage:");
  println!("       dump [-avhf] [filepath.mxf]");
  println!("");
  println!(" --all: display all elements");
  println!(" -a, --audio: display audio elements");
  println!(" -v, --video: display video elements");
  println!(" -h, --header: display header elements");
  println!(" -f, --footer: display footer elements");
  println!(" --fill: display fill elements");
  println!(" --si: display System Item elements");
  println!(" --anc: display ANC elements");
  println!("");
  process::exit(0x0f00);
}

#[derive(Debug)]
struct Options {
  filter_header: bool,
  filter_footer: bool,
  filter_body: bool,
  filter_sound_wave: bool,
  filter_video_frame: bool,
  filter_fill: bool,
  filter_system_item: bool,
  filter_anc: bool,
}

impl Default for Options {
  fn default() -> Options {
    Options {
      filter_header: true,
      filter_footer: true,
      filter_body: true,
      filter_sound_wave: true,
      filter_video_frame: true,
      filter_fill: true,
      filter_system_item: true,
      filter_anc: true,
    }
  }
}

fn main() {
  let mut options = Options{..Default::default()};

  let mut path = None;

  for arg in env::args().skip(1) {
    match arg.as_str() {
      "--all" => {
        options.filter_sound_wave = false;
        options.filter_video_frame = false;
        options.filter_header = false;
        options.filter_footer = false;
        options.filter_body = false;
        options.filter_fill = false;
        options.filter_system_item = false;
        options.filter_anc = false;
      },
      "-a" | "--audio" => {options.filter_sound_wave = false;},
      "-v" | "--video" => {options.filter_video_frame = false;},
      "-h" | "--header" => {options.filter_header = false;},
      "-f" | "--footer"  => {options.filter_footer = false;},
      "-b" | "--body"  => {options.filter_body = false;},
      "--fill"  => {options.filter_fill = false;},
      "--si"  => {options.filter_system_item = false;},
      "--anc"  => {options.filter_anc = false;},
      _ => path = Some(arg),
    }
  }

  let file =
    match path {
      Some(path) => File::open(path).unwrap(),
      None => {
        display_error();
        return;
      },
    };

  // println!("filter_video_frame {:?}", filter_video_frame);
  // println!("filter_sound_wave {:?}", filter_sound_wave);
  let stream = BufReader::new(file);

  let mut reader = KlvReader{
    stream: stream,
    elements: vec![]
  };

  loop {
    let mut klv = Klv{..Default::default()};
    match klv.deserialize(&mut reader) {
      Ok(false) => {
        break;
      }
      Ok(true) => {
        match (
          options.filter_header,
          options.filter_footer,
          options.filter_body,
          options.filter_video_frame,
          options.filter_sound_wave,
          options.filter_fill,
          options.filter_system_item,
          options.filter_anc,
          klv.key.clone()) {
          (true, _, _, _, _, _, _, _, Ul::HeaderPartition{..}) |
          (true, _, _, _, _, _, _, _, Ul::PrimerPack) |
          (true, _, _, _, _, _, _, _, Ul::MaterialPackageSet) |
          (true, _, _, _, _, _, _, _, Ul::TrackSet) |
          (true, _, _, _, _, _, _, _, Ul::StaticTrackSet) |
          (true, _, _, _, _, _, _, _, Ul::ContentStorageSet) |
          (true, _, _, _, _, _, _, _, Ul::FilePackageSet) |
          (true, _, _, _, _, _, _, _, Ul::PrefaceSet) |
          (true, _, _, _, _, _, _, _, Ul::SequenceSet) |
          (true, _, _, _, _, _, _, _, Ul::SourceClipSet) |
          (true, _, _, _, _, _, _, _, Ul::IdentificationSet) |
          (true, _, _, _, _, _, _, _, Ul::TimecodeComponentSet) |
          (true, _, _, _, _, _, _, _, Ul::MultipleDescriptorSet) |
          (true, _, _, _, _, _, _, _, Ul::RgbaVideoDescriptor) |
          (true, _, _, _, _, _, _, _, Ul::CdciVideoDescriptor) |
          (true, _, _, _, _, _, _, _, Ul::Jpeg2000SubDescriptorSet) |
          (true, _, _, _, _, _, _, _, Ul::MpegVideoDescriptorSet) |
          (true, _, _, _, _, _, _, _, Ul::WaveAudioDescriptorSet) |
          (true, _, _, _, _, _, _, _, Ul::Aes3AudioDescriptorSet) |
          (true, _, _, _, _, _, _, _, Ul::EssenceContainerDataSet) |
          (true, _, _, _, _, _, _, _, Ul::DmSegmentDescriptorSet) |
          (true, _, _, _, _, _, _, _, Ul::AudioChannelLabelSubDescriptorSet) |
          (true, _, _, _, _, _, _, _, Ul::SoundfieldGroupLabelSubDescriptorSet) |
          (true, _, _, _, _, _, _, _, Ul::AS10CoreFramework) |
          (_, true, _, _, _, _, _, _, Ul::FooterPartition{..}) |
          (_, true, _, _, _, _, _, _, Ul::RandomIndexMetadata) |
          (_, true, _, _, _, _, _, _, Ul::IndexTableSegment) |
          (_, _, true, _, _, _, _, _, Ul::BodyPartition{..}) |
          (_, _, _, true, _, _, _, _, Ul::PictureItemMpegFrameWrappedPictureElement) |
          (_, _, _, true, _, _, _, _, Ul::Jpeg2000FrameWrapped) |
          (_, _, _, true, _, _, _, _, Ul::Jpeg2000ClipWrapped) |
          (_, _, _, _, true, _, _, _, Ul::SoundItemWaveDataWrappedSoundElement) |
          (_, _, _, _, _, true, _, _, Ul::FillItem) |
          (_, _, _, _, _, true, _, _, Ul::FillItemAvid) |
          (_, _, _, _, _, _, true, _, Ul::SystemItemSystemMetadataPack) |
          (_, _, _, _, _, _, true, _, Ul::SystemItemPackageMetadataSet) |
          (_, _, _, _, _, _, _, true, Ul::Essence_AncFrameElement) => {
          },
          _ => {
            println!("{:?}", klv);
          },
        }
      },
      Err(msg) => {
        println!("ERROR: {:?}", msg);
        break;
      }
    }
    
  }
}
