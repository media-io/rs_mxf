
use serializer::encoder::Encoder;
use klv::value::partition::*;
use klv::ul::Ul;

impl Encoder for Ul {
  fn serialise(&self) -> Vec<u8> {
    match *self {
      Ul::HeaderPartition{status} => {
        vec_ul!(Ul::HeaderPartition, status)
      },
      Ul::BodyPartition{status} => {
        vec_ul!(Ul::BodyPartition, status)
      },
      Ul::FooterPartition{status} => {
        vec_ul!(Ul::FooterPartition, status)
      },
      Ul::PrimerPack => {
        vec_ul!(Ul::PrimerPack)
      },
      // Ul::Essence_MpegEsWithStreamIdFrameWrapped => {
      //   tuple_to_vec!(
      //     build_identifier!(version_number => 0x02, mpeg_es => 0x01)
      //   )
      // },
      Ul::Jpeg2000FrameWrapped => {
        vec_ul!(Ul::Jpeg2000FrameWrapped, 0x00)
      },
      Ul::Essence_Jpeg2000_FrameWrapped => {
        vec_ul!(Ul::Essence_Jpeg2000_FrameWrapped)
      },
      Ul::MxfOP1aSingleItemSinglePackageUniTrackStreamInternal => {
        vec_ul!(Ul::MxfOP1aSingleItemSinglePackageUniTrackStreamInternal)
      },
      Ul::FillItem => {
        vec_ul!(Ul::FillItem)
      },
      _ => {
        println!("unsupported UL: {:?}", self);
        vec![]
      }
    }
  }
}
