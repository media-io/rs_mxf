
use klv::key::dict::*;
use klv::key::key::*;
use klv::key::partition::*;

fn get_set_kind(v: u8) -> KeyIdentifier {
  match v {
    0x0f => KeyIdentifier::SequenceSet,
    0x11 => KeyIdentifier::SourceClipSet,
    0x14 => KeyIdentifier::TimecodeComponentSet,
    0x18 => KeyIdentifier::ContentStorageSet,
    0x23 => KeyIdentifier::EssenceContainerDataSet,
    0x25 => KeyIdentifier::FileDescriptorSet,
    0x27 => KeyIdentifier::GenericPictureEssenceDescriptor,
    0x28 => KeyIdentifier::CdciVideoDescriptor,
    0x29 => KeyIdentifier::RgbaVideoDescriptor,
    0x2f => KeyIdentifier::PrefaceSet,
    0x30 => KeyIdentifier::IdentificationSet,
    0x32 => KeyIdentifier::NetworkLocatorSet,
    0x33 => KeyIdentifier::TextLocatorSet,
    0x36 => KeyIdentifier::MaterialPackageSet,
    0x37 => KeyIdentifier::FilePackageSet,
    0x3a => KeyIdentifier::StaticTrackSet,
    0x3b => KeyIdentifier::TrackSet,
    0x39 => KeyIdentifier::EventTrackSet,
    0x41 => KeyIdentifier::DmSegmentDescriptorSet,
    0x42 => KeyIdentifier::GenericSoundEssenceDescriptorSet,
    0x43 => KeyIdentifier::GenericDataEssenceDescriptorSet,
    0x44 => KeyIdentifier::MultipleDescriptorSet,
    0x45 => KeyIdentifier::DmSourceClipSet,
    0x47 => KeyIdentifier::Aes3AudioDescriptorSet,
    0x48 => KeyIdentifier::WaveAudioDescriptorSet,
    0x51 => KeyIdentifier::MpegVideoDescriptorSet,
    0x5a => KeyIdentifier::Jpeg2000SubDescriptorSet,
    0x6a => KeyIdentifier::McaLabelSubDescriptorSet,
    0x6b => KeyIdentifier::AudioChannelLabelSubDescriptorSet,
    0x6c => KeyIdentifier::SoundfieldGroupLabelSubDescriptorSet,
       _ => KeyIdentifier::Unknown,
  }
}

pub fn parse_key(data: Vec<u8>) -> Key {
  // println!("{:?}", data);

  match (data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8], data[9], data[10], data[11], data[12], data[13], data[14], data[15]) {
    ul_filter!(Ul::HeaderPartition) => {
      let status = parse_status(data[14]);
      Key {identifier: KeyIdentifier::HeaderPartition{status: status}}
    },
    ul_filter!(Ul::BodyPartition) => {
      let status = parse_status(data[14]);
      Key {identifier: KeyIdentifier::BodyPartition{status: status}}
    },
    ul_filter!(Ul::FooterPartition) => {
      let status = parse_status(data[14]);
      Key {identifier: KeyIdentifier::FooterPartition{status: status}}
    },
    ul_filter!(Ul::PrimerPack) => {
      Key {identifier: KeyIdentifier::PrimerPack}
    },
    ul_filter!(Ul::RandomIndexMetadata) => {
      Key {identifier: KeyIdentifier::RandomIndexMetadata}
    },
    ul_filter!(Ul::FillItem) => {
      Key {identifier: KeyIdentifier::FillItem}
    },
    ul_filter!(Ul::FillItemAvid) => {
      Key {identifier: KeyIdentifier::FillItemAvid}
    },
    ul_filter!(SmpteRegitery::Set) => {
      Key {identifier: get_set_kind(data[14])}
    },
    (0x06, 0x0e, 0x2b, 0x34, 0x02, 0x53, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, 0x01, 0x10, 0x01, 0x00) => {
      Key {identifier: KeyIdentifier::IndexTableSegment}
    },
    (0x06, 0x0e, 0x2b, 0x34, 0x02, 0x53, 0x01, 0x01, 0x0d, 0x01, 0x03, 0x01, 0x14, 0x02, 0x01, 0x00) => {
      Key {identifier: KeyIdentifier::SystemItemElement}
    },
    (0x06, 0x0e, 0x2b, 0x34, 0x02, 0x05, 0x01, 0x01, 0x0d, 0x01, 0x03, 0x01, 0x04, 0x01, 0x01, 0x00) => {
      Key {identifier: KeyIdentifier::SystemItemSystemMetadataPack}
    },
    (0x06, 0x0e, 0x2b, 0x34, 0x02, 0x43, 0x01, 0x01, 0x0d, 0x01, 0x03, 0x01, 0x04, 0x01, 0x02, _track) => {
      Key {identifier: KeyIdentifier::SystemItemPackageMetadataSet}
    },
    ul_filter!(Ul::PictureItemMpegFrameWrappedPictureElement) => {
      Key {identifier: KeyIdentifier::PictureItemMpegFrameWrappedPictureElement}
    },
    ul_filter!(Ul::Jpeg2000FrameWrapped) => {
      Key {identifier: KeyIdentifier::Jpeg2000FrameWrapped}
    },
    ul_filter!(Ul::Jpeg2000ClipWrapped) => {
      Key {identifier: KeyIdentifier::Jpeg2000ClipWrapped}
    },
    (0x06, 0x0e, 0x2b, 0x34, 0x01, 0x02, 0x01, 0x01, 0x0d, 0x01, 0x03, 0x01, 0x16, 0x00, 0x00, _stream_index) => {
      Key {identifier: KeyIdentifier::SoundItemDataWrappedSoundElement}
    },
    (0x06, 0x0e, 0x2b, 0x34, 0x01, 0x02, 0x01, 0x01, 0x0d, 0x01, 0x03, 0x01, 0x16,    _,    _, _stream_index) => {
      Key {identifier: KeyIdentifier::SoundItemWaveDataWrappedSoundElement}
    },
    (0x06, 0x0e, 0x2b, 0x34, 0x01, 0x02, 0x01, 0x01, 0x0d, 0x01, 0x03, 0x01, 0x06, 0x00, 0x00, _stream_index) => {
      Key {identifier: KeyIdentifier::SoundItemBwfDataWrappedSoundElement}
    },
    (0x06, 0x0e, 0x2b, 0x34, 0x02, 0x53, 0x01, 0x01, 0x0d, 0x01, 0x07, 0x01, 0x0a, 0x01, 0x01, 0x00) => {
      Key {identifier: KeyIdentifier::AS10CoreFramework}
    },
    (0x06, 0x0e, 0x2b, 0x34, 0x01, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x07, 0x01, 0x0a, 0x01, 0x01, 0x01) => {
      Key {identifier: KeyIdentifier::AS10ShimName}
    },
    (0x06, 0x0e, 0x2b, 0x34, 0x01, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x07, 0x01, 0x0a, 0x01, 0x01, 0x02) => {
      Key {identifier: KeyIdentifier::AS10Type}
    },
    (0x06, 0x0e, 0x2b, 0x34, 0x01, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x07, 0x01, 0x0a, 0x01, 0x01, 0x03) => {
      Key {identifier: KeyIdentifier::AS10MainTitle}
    },
    (0x06, 0x0e, 0x2b, 0x34, 0x01, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x07, 0x01, 0x0a, 0x01, 0x01, 0x04) => {
      Key {identifier: KeyIdentifier::AS10SubTitle}
    },
    (0x06, 0x0e, 0x2b, 0x34, 0x01, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x07, 0x01, 0x0a, 0x01, 0x01, 0x05) => {
      Key {identifier: KeyIdentifier::AS10TitleDescription}
    },
    (0x06, 0x0e, 0x2b, 0x34, 0x01, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x07, 0x01, 0x0a, 0x01, 0x01, 0x06) => {
      Key {identifier: KeyIdentifier::AS10OrganizationName}
    },
    (0x06, 0x0e, 0x2b, 0x34, 0x01, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x07, 0x01, 0x0a, 0x01, 0x01, 0x07) => {
      Key {identifier: KeyIdentifier::AS10PersonName}
    },
    (0x06, 0x0e, 0x2b, 0x34, 0x01, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x07, 0x01, 0x0a, 0x01, 0x01, 0x08) => {
      Key {identifier: KeyIdentifier::AS10LocationDescription}
    },
    (0x06, 0x0e, 0x2b, 0x34, 0x01, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x07, 0x01, 0x0a, 0x01, 0x01, 0x09) => {
      Key {identifier: KeyIdentifier::AS10CommonSpanningID}
    },
    (0x06, 0x0e, 0x2b, 0x34, 0x01, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x07, 0x01, 0x0a, 0x01, 0x01, 0x0a) => {
      Key {identifier: KeyIdentifier::AS10SpanningNumber}
    },
    (0x06, 0x0e, 0x2b, 0x34, 0x01, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x07, 0x01, 0x0a, 0x01, 0x01, 0x0b) => {
      Key {identifier: KeyIdentifier::AS10CumulativeDuration}
    },
    (0x06, 0x0e, 0x2b, 0x34, _, _, _, _, _, _, _, _, _, _, _, _) => {
      println!("unknown key {}", format_key(&data));
      Key {identifier: KeyIdentifier::Unknown}
    },
    _ => panic!("bad key parsing")
  }
}
