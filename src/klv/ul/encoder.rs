
use serializer::encoder::Encoder;
use klv::value::partition::*;
use klv::ul::ul::Ul;

impl Encoder for Ul {
  fn serialise(&self) -> Vec<u8> {
    match *self {
      Ul::MxfOP1aSingleItemSinglePackageMultiTrackStreamInternal => {
        tuple_to_vec!(
          smpte_identifier!(
            Ul::MxfOP1aSingleItemSinglePackageMultiTrackStreamInternal
          )
        )
      },
      Ul::HeaderPartition { status } => {
        vec_ul!( Ul::HeaderPartition, status.unwrap())
      },
      Ul::BodyPartition { status } => {
        vec_ul!( Ul::BodyPartition, status.unwrap())
      },
      Ul::FooterPartition { status } => {
        vec_ul!( Ul::FooterPartition, status.unwrap())
      },
      // Ul::Essence_MpegEsWithStreamIdFrameWrapped => {
      //   tuple_to_vec!(
      //     build_identifier!(version_number => 0x02, mpeg_es => 0x01)
      //   )
      // },
      _ => {
        println!("{:?}", self);
        vec![]
      }
    }
  }
}
