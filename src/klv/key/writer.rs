
use std::io::Write;
use klv::key::dict::*;
use klv::key::key::*;
use klv::key::partition::*;
use klv::ul::*;

pub fn write_key<W: Write>(stream: &mut W, key: &Key) {
  let ul =
    match key.identifier {
      KeyIdentifier::HeaderPartition{ref status} => vec_ul!(Ul::HeaderPartition, *status),
      KeyIdentifier::BodyPartition{ref status} => vec_ul!(Ul::BodyPartition, *status),
      KeyIdentifier::FooterPartition{ref status} => vec_ul!(Ul::FooterPartition, *status),
      
      KeyIdentifier::SequenceSet => vec_ul!(Ul::SequenceSet),
      KeyIdentifier::SourceClipSet => vec_ul!(Ul::SourceClipSet),
      KeyIdentifier::TimecodeComponentSet => vec_ul!(Ul::TimecodeComponentSet),
      KeyIdentifier::ContentStorageSet => vec_ul!(Ul::ContentStorageSet),
      KeyIdentifier::EssenceContainerDataSet => vec_ul!(Ul::EssenceContainerDataSet),
      KeyIdentifier::FileDescriptorSet => vec_ul!(Ul::FileDescriptorSet),
      KeyIdentifier::GenericPictureEssenceDescriptor => vec_ul!(Ul::GenericPictureEssenceDescriptor),
      KeyIdentifier::CdciVideoDescriptor => vec_ul!(Ul::CdciVideoDescriptor),
      KeyIdentifier::RgbaVideoDescriptor => vec_ul!(Ul::RgbaVideoDescriptor),
      KeyIdentifier::PrefaceSet => vec_ul!(Ul::PrefaceSet),
      KeyIdentifier::IdentificationSet => vec_ul!(Ul::IdentificationSet),
      KeyIdentifier::NetworkLocatorSet => vec_ul!(Ul::NetworkLocatorSet),
      KeyIdentifier::TextLocatorSet => vec_ul!(Ul::TextLocatorSet),
      KeyIdentifier::MaterialPackageSet => vec_ul!(Ul::MaterialPackageSet),
      KeyIdentifier::FilePackageSet => vec_ul!(Ul::FilePackageSet),
      KeyIdentifier::StaticTrackSet => vec_ul!(Ul::StaticTrackSet),
      KeyIdentifier::TrackSet => vec_ul!(Ul::TrackSet),
      KeyIdentifier::EventTrackSet => vec_ul!(Ul::EventTrackSet),
      KeyIdentifier::DmSegmentDescriptorSet => vec_ul!(Ul::DmSegmentDescriptorSet),
      KeyIdentifier::GenericSoundEssenceDescriptorSet => vec_ul!(Ul::GenericSoundEssenceDescriptorSet),
      KeyIdentifier::GenericDataEssenceDescriptorSet => vec_ul!(Ul::GenericDataEssenceDescriptorSet),
      KeyIdentifier::MultipleDescriptorSet => vec_ul!(Ul::MultipleDescriptorSet),
      KeyIdentifier::DmSourceClipSet => vec_ul!(Ul::DmSourceClipSet),
      KeyIdentifier::Aes3AudioDescriptorSet => vec_ul!(Ul::Aes3AudioDescriptorSet),
      KeyIdentifier::WaveAudioDescriptorSet => vec_ul!(Ul::WaveAudioDescriptorSet),
      KeyIdentifier::MpegVideoDescriptorSet => vec_ul!(Ul::MpegVideoDescriptorSet),
      KeyIdentifier::Jpeg2000SubDescriptorSet => vec_ul!(Ul::Jpeg2000SubDescriptorSet),
      KeyIdentifier::McaLabelSubDescriptorSet => vec_ul!(Ul::McaLabelSubDescriptorSet),
      KeyIdentifier::AudioChannelLabelSubDescriptorSet => vec_ul!(Ul::AudioChannelLabelSubDescriptorSet),
      KeyIdentifier::SoundfieldGroupLabelSubDescriptorSet => vec_ul!(Ul::SoundfieldGroupLabelSubDescriptorSet),
      
      KeyIdentifier::PictureItemMpegFrameWrappedPictureElement => vec_ul!(Ul::PictureItemMpegFrameWrappedPictureElement, 0x00),
      KeyIdentifier::Jpeg2000FrameWrapped => vec_ul!(Ul::Jpeg2000FrameWrapped, 0x00),
      KeyIdentifier::Jpeg2000ClipWrapped => vec_ul!(Ul::Jpeg2000ClipWrapped, 0x00),
      KeyIdentifier::FillItem => vec_ul!(Ul::FillItem),
      KeyIdentifier::FillItemAvid => vec_ul!(Ul::FillItemAvid),
      KeyIdentifier::Unknown => panic!("Unknown key identifier"),
      _ => {
        unimplemented!();
      }
    };

  stream.write(&ul).unwrap();
}
