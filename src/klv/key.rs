
use serializer::encoder::*;

#[derive(Debug)]
pub enum KeyIdentifier {
  HeaderPartition,
  BodyPartition,
  FooterPartition,
  PrimerPack,
  RandomIndexMetadata,

  StaticTrack,

  SystemItemElement,
  IndexTableSegment,

  SystemItemSystemMetadataPack,
  SystemItemPackageMetadataSet,

  SequenceSet,
  SourceClipSet,
  TimecodeComponentSet,
  ContentStorageSet,
  EssenceContainerDataSet,
  FileDescriptorSet,
  GenericPictureEssenceDescriptor,
  CdciVideoDescriptor,
  RgbaVideoDescriptor,
  PrefaceSet,
  IdentificationSet,
  NetworkLocatorSet,
  TextLocatorSet,
  MaterialPackageSet,
  FilePackageSet,
  StaticTrackSet,
  TrackSet,
  EventTrackSet,
  DmSegmentDescriptorSet,
  GenericSoundEssenceDescriptorSet,
  GenericDataEssenceDescriptorSet,
  MultipleDescriptorSet,
  DmSourceClipSet,
  Aes3AudioDescriptorSet,
  WaveAudioDescriptorSet,
  MpegVideoDescriptorSet,
  Jpeg2000SubDescriptorSet,
  McaLabelSubDescriptorSet,
  AudioChannelLabelSubDescriptorSet,
  SoundfieldGroupLabelSubDescriptorSet,

  PictureItemMpegFrameWrappedPictureElement,
  Jpeg2000FrameWrapped,
  Jpeg2000ClipWrapped,
  SoundItemDataWrappedSoundElement,
  SoundItemWaveDataWrappedSoundElement,
  SoundItemBwfDataWrappedSoundElement,

  FillItem,
  FillItemAvid,
  Unknown,
}

#[derive(Debug)]
pub struct Key {
  pub identifier: KeyIdentifier
}

fn get_smpte_identifier() -> Vec<u8> {
  vec![0x06, 0x0e, 0x2b, 0x34]
}

fn get_partition_key(identifier: &KeyIdentifier) -> Vec<u8> {
  let id =
    match *identifier {
      KeyIdentifier::HeaderPartition => 0x02,
      KeyIdentifier::BodyPartition => 0x03,
      KeyIdentifier::FooterPartition => 0x04,
      KeyIdentifier::PrimerPack => 0x04,
      KeyIdentifier::RandomIndexMetadata => 0x11,
      _ => panic!("Unknown key identifier"),
    };

  let mut partition_id = vec![0x02, 0x05, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, 0x01, id, 0x00, 0x00];
  let mut smpte_identifier = get_smpte_identifier();
  smpte_identifier.append(&mut partition_id);
  smpte_identifier
}

impl Encoder for Key {
  fn serialise(&self) -> Vec<u8> {
    match self.identifier {
      KeyIdentifier::HeaderPartition |
      KeyIdentifier::BodyPartition |
      KeyIdentifier::FooterPartition =>
        get_partition_key(&self.identifier),
      KeyIdentifier::StaticTrack => {
        let mut partition_id = vec![0x02, 0x53, 0x01, 0x01, 0x0d, 0x01, 0x01, 0x01, 0x01, 0x01, 0x3a, 0x00];
        let mut smpte_identifier = get_smpte_identifier();
        smpte_identifier.append(&mut partition_id);
        smpte_identifier
      },
      KeyIdentifier::PictureItemMpegFrameWrappedPictureElement => {
        let mut frame_id = vec![0x01, 0x02, 0x01, 0x01, 0x0d, 0x01, 0x03, 0x01, 0x15, 0x01, 0x05, 0x00];
        let mut smpte_identifier = get_smpte_identifier();
        smpte_identifier.append(&mut frame_id);
        smpte_identifier
      },
      KeyIdentifier::Jpeg2000FrameWrapped => {
        let mut frame_id = vec![0x01, 0x02, 0x01, 0x01, 0x0d, 0x01, 0x03, 0x01, 0x15, 0x01, 0x08, 0x00];
        let mut smpte_identifier = get_smpte_identifier();
        smpte_identifier.append(&mut frame_id);
        smpte_identifier
      },
      KeyIdentifier::Jpeg2000ClipWrapped => {
        let mut frame_id = vec![0x01, 0x02, 0x01, 0x01, 0x0d, 0x01, 0x03, 0x01, 0x15, 0x01, 0x09, 0x00];
        let mut smpte_identifier = get_smpte_identifier();
        smpte_identifier.append(&mut frame_id);
        smpte_identifier
      },
      KeyIdentifier::FillItem => {
        let mut id = vec![0x01, 0x01, 0x01, 0x02, 0x03, 0x01, 0x02, 0x10, 0x01, 0x00, 0x00, 0x00];
        let mut smpte_identifier = get_smpte_identifier();
        smpte_identifier.append(&mut id);
        smpte_identifier
      },
      KeyIdentifier::FillItemAvid => {
        let mut id = vec![0x01, 0x01, 0x01, 0x01, 0x03, 0x01, 0x02, 0x10, 0x01, 0x00, 0x00, 0x00];
        let mut smpte_identifier = get_smpte_identifier();
        smpte_identifier.append(&mut id);
        smpte_identifier
      },
      KeyIdentifier::Unknown => panic!("Unknown key identifier"),
      _ => {
        unimplemented!();
      }
    }
  }
}

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
    (0x06, 0x0e, 0x2b, 0x34, 0x02, 0x05, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, 0x01, 0x02,    _, 0x00) => {
      Key {identifier: KeyIdentifier::HeaderPartition}
    },
    (0x06, 0x0e, 0x2b, 0x34, 0x02, 0x05, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, 0x01, 0x03,    _, 0x00) => {
      Key {identifier: KeyIdentifier::BodyPartition}
    },
    (0x06, 0x0e, 0x2b, 0x34, 0x02, 0x05, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, 0x01, 0x04,    _, 0x00) => {
      Key {identifier: KeyIdentifier::FooterPartition}
    },
    (0x06, 0x0e, 0x2b, 0x34, 0x02, 0x05, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, 0x01, 0x05,    _, 0x00) => {
      Key {identifier: KeyIdentifier::PrimerPack}
    },
    (0x06, 0x0e, 0x2b, 0x34, 0x02, 0x05, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, 0x01, 0x11,    _, 0x00) => {
      Key {identifier: KeyIdentifier::RandomIndexMetadata}
    },
    (0x06, 0x0e, 0x2b, 0x34, 0x01, 0x01, 0x01, 0x02, 0x03, 0x01, 0x02, 0x10, 0x01, 0x00, 0x00, 0x00) => {
      Key {identifier: KeyIdentifier::FillItem}
    },
    (0x06, 0x0e, 0x2b, 0x34, 0x01, 0x01, 0x01, 0x01, 0x03, 0x01, 0x02, 0x10, 0x01, 0x00, 0x00, 0x00) => {
      Key {identifier: KeyIdentifier::FillItemAvid}
    },
    (0x06, 0x0e, 0x2b, 0x34, 0x02, 0x53, 0x01, 0x01, 0x0d, 0x01, 0x01, 0x01, 0x01, 0x01, kind, 0x00) => {
      Key {identifier: get_set_kind(kind)}
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
    (0x06, 0x0e, 0x2b, 0x34, 0x01, 0x02, 0x01, 0x01, 0x0d, 0x01, 0x03, 0x01, 0x15, 0x01, 0x05, 0x00) => {
      Key {identifier: KeyIdentifier::PictureItemMpegFrameWrappedPictureElement}
    },
    (0x06, 0x0e, 0x2b, 0x34, 0x01, 0x02, 0x01, 0x01, 0x0d, 0x01, 0x03, 0x01, 0x15, 0x01, 0x08, 0x00) => {
      Key {identifier: KeyIdentifier::Jpeg2000FrameWrapped}
    },
    (0x06, 0x0e, 0x2b, 0x34, 0x01, 0x02, 0x01, 0x01, 0x0d, 0x01, 0x03, 0x01, 0x15, 0x01, 0x09, 0x00) => {
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
    (0x06, 0x0e, 0x2b, 0x34, _, _, _, _, _, _, _, _, _, _, _, _) => {
      Key {identifier: KeyIdentifier::Unknown}
    },
    _ => panic!("bad key parsing")
  }
}
