
#[derive(Debug)]
pub enum Ul {
  HeaderPartition,
  BodyPartition,
  FooterPartition,
  PrimerPack,
  RandomIndexMetadata,
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
}

#[derive(Debug)]
pub enum SmpteRegitery {
  Partition,
  Set,
  Essence,
  Fill
}

macro_rules! tuple_to_vec {
  ($ul:expr) => (
    vec![$ul.0, $ul.1, $ul.2, $ul.3, $ul.4, $ul.5, $ul.6, $ul.7, $ul.8, $ul.9, $ul.10, $ul.11, $ul.12, $ul.13, $ul.14, $ul.15]
  );
}

macro_rules! vec_ul {
  (Ul::HeaderPartition, $status:expr) => (tuple_to_vec!(partition_identifier!(Ul::HeaderPartition, $status)););
  (Ul::BodyPartition, $status:expr) => (tuple_to_vec!(partition_identifier!(Ul::BodyPartition, $status)));
  (Ul::FooterPartition, $status:expr) => (tuple_to_vec!(partition_identifier!(Ul::FooterPartition, $status)));
  (Ul::PrimerPack) => (tuple_to_vec!(partition_identifier!(Ul::PrimerPack)));
  (Ul::RandomIndexMetadata) => (tuple_to_vec!(partition_identifier!(Ul::RandomIndexMetadata)));
  (Ul::StaticTrack) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x3a)));
  (Ul::Track) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x3b)));
  (Ul::PictureItemMpegFrameWrappedPictureElement, $stream:tt) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Essence, 0x15, 0x05, $stream)));
  (Ul::Jpeg2000FrameWrapped, $stream:tt) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Essence, 0x15, 0x08, $stream)));
  (Ul::Jpeg2000ClipWrapped, $stream:tt) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Essence, 0x15, 0x09, $stream)));
  (Ul::SoundItemDataWrappedSoundElement, $stream:tt) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Essence, 0x16, 0x00, $stream)));
  (Ul::SoundItemWaveDataWrappedSoundElement, $stream:tt) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Essence, 0x16, 0x00, $stream)));
  (Ul::SoundItemBwfDataWrappedSoundElement, $stream:tt) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Essence, 0x06, 0x00, $stream)));
  (Ul::FillItem) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Fill, 0x02)));
  (Ul::FillItemAvid) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Fill, 0x01)));

  (Ul::SequenceSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x0f)));
  (Ul::SourceClipSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x11)));
  (Ul::TimecodeComponentSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x14)));
  (Ul::ContentStorageSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x18)));
  (Ul::EssenceContainerDataSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x23)));
  (Ul::FileDescriptorSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x25)));
  (Ul::GenericPictureEssenceDescriptor) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x27)));
  (Ul::CdciVideoDescriptor) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x28)));
  (Ul::RgbaVideoDescriptor) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x29)));
  (Ul::PrefaceSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x2f)));
  (Ul::IdentificationSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x30)));
  (Ul::NetworkLocatorSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x32)));
  (Ul::TextLocatorSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x33)));
  (Ul::MaterialPackageSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x36)));
  (Ul::FilePackageSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x37)));
  (Ul::StaticTrackSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x3a)));
  (Ul::TrackSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x3b)));
  (Ul::EventTrackSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x39)));
  (Ul::DmSegmentDescriptorSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x41)));
  (Ul::GenericSoundEssenceDescriptorSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x42)));
  (Ul::GenericDataEssenceDescriptorSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x43)));
  (Ul::MultipleDescriptorSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x44)));
  (Ul::DmSourceClipSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x45)));
  (Ul::Aes3AudioDescriptorSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x47)));
  (Ul::WaveAudioDescriptorSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x48)));
  (Ul::MpegVideoDescriptorSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x51)));
  (Ul::Jpeg2000SubDescriptorSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x5a)));
  (Ul::McaLabelSubDescriptorSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x6a)));
  (Ul::AudioChannelLabelSubDescriptorSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x6b)));
  (Ul::SoundfieldGroupLabelSubDescriptorSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x6c)));
}

macro_rules! ul_filter {
  (Ul::HeaderPartition) => (partition_identifier!(Ul::HeaderPartition));
  (Ul::BodyPartition) => (partition_identifier!(Ul::BodyPartition));
  (Ul::FooterPartition) => (partition_identifier!(Ul::FooterPartition));
  (Ul::PrimerPack) => (partition_identifier!(Ul::PrimerPack));
  (Ul::RandomIndexMetadata) => (partition_identifier!(Ul::RandomIndexMetadata));
  (Ul::FillItem) => (smpte_identifier!(SmpteRegitery::Fill, 0x02));
  (Ul::FillItemAvid) => (smpte_identifier!(SmpteRegitery::Fill, 0x01));
  (SmpteRegitery::Set) => (smpte_identifier!(SmpteRegitery::Set));
}

macro_rules! partition_status_identifier {
  ($ul:tt, $status:expr) => (
    match $status {
      PartitionStatus::OpenAndIncomplete => smpte_identifier!(SmpteRegitery::Partition, $ul, 0x01),
      PartitionStatus::ClosedAndIncomplete => smpte_identifier!(SmpteRegitery::Partition, $ul, 0x02),
      PartitionStatus::OpenAndComplete => smpte_identifier!(SmpteRegitery::Partition, $ul, 0x03),
      PartitionStatus::ClosedAndComplete => smpte_identifier!(SmpteRegitery::Partition, $ul, 0x04),
    }
  );
}

macro_rules! partition_identifier {
  ($ul:expr, $status:expr) => (
    match $ul {
      Ul::HeaderPartition => partition_status_identifier!(0x02, $status),
      Ul::BodyPartition => partition_status_identifier!(0x03, $status),
      Ul::FooterPartition => partition_status_identifier!(0x04, $status),
      _ => panic!("Not covered Partition identifier")
    }
  );
  (Ul::HeaderPartition) => (
    smpte_identifier!(SmpteRegitery::Partition, 0x02)
  );
  (Ul::BodyPartition) => (
    smpte_identifier!(SmpteRegitery::Partition, 0x03)
  );
  (Ul::FooterPartition) => (
    smpte_identifier!(SmpteRegitery::Partition, 0x04)
  );
  (Ul::PrimerPack) => (
    smpte_identifier!(SmpteRegitery::Partition, 0x05)
  );
  (Ul::RandomIndexMetadata) => (
    smpte_identifier!(SmpteRegitery::Partition, 0x11)
  );
}

macro_rules! smpte_identifier {
  (SmpteRegitery::Partition, $x:tt, $status:tt) => (
    smpte_identifier!(0x02, 0x05, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, 0x01, $x, $status, 0x00)
  );
  (SmpteRegitery::Partition, $x:tt) => (
    smpte_identifier!(0x02, 0x05, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, 0x01, $x, _, 0x00)
  );
  (SmpteRegitery::Set, $x:tt) => (
    smpte_identifier!(0x02, 0x53, 0x01, 0x01, 0x0d, 0x01, 0x01, 0x01, 0x01, 0x01, $x, 0x00)
  );
  (SmpteRegitery::Set) => (
    smpte_identifier!(0x02, 0x53, 0x01, 0x01, 0x0d, 0x01, 0x01, 0x01, 0x01, 0x01, _, 0x00)
  );
  (SmpteRegitery::Essence, $x:tt, $y:tt, $stream:tt) => (
    smpte_identifier!(0x02, 0x02, 0x01, 0x01, 0x0d, 0x01, 0x03, 0x01, $x, 0x00, $y, $stream)
  );
  (SmpteRegitery::Fill, $x:tt) => (
    smpte_identifier!(0x01, 0x01, 0x01, $x, 0x03, 0x01, 0x02, 0x10, 0x01, 0x00, 0x00, 0x00)
  );

  ($x0:tt, $x1:tt, $x2:tt, $x3:tt, $x4:tt, $x5:tt, $x6:tt, $x7:tt, $x8:tt, $x9:tt, $x10:tt, $x11:tt) => (
    (0x06, 0x0e, 0x2b, 0x34, $x0, $x1, $x2, $x3, $x4, $x5, $x6, $x7, $x8, $x9, $x10, $x11)
  );
}
