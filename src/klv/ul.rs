
#[derive(Debug)]
pub enum Ul {
  HeaderPartition,
  BodyPartition,
  FooterPartition,
  PrimerPack,
  RandomIndexMetadata,
  StaticTrack,
  Track,
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
  Track,
  Essence,
  Fill
}

macro_rules! tuple_to_vec {
  ($ul:expr) => (
    vec![$ul.0, $ul.1, $ul.2, $ul.3, $ul.4, $ul.5, $ul.6, $ul.7, $ul.8, $ul.9, $ul.10, $ul.11, $ul.12, $ul.13, $ul.14, $ul.15]
  );
}

macro_rules! vec_ul {
  (Ul::HeaderPartition, $status:expr) => {
    tuple_to_vec!(partition_identifier!(Ul::HeaderPartition, $status));
  };
  (Ul::BodyPartition, $status:expr) => (
    tuple_to_vec!(partition_identifier!(Ul::BodyPartition, $status))
  );
  (Ul::FooterPartition, $status:expr) => (
    tuple_to_vec!(partition_identifier!(Ul::FooterPartition, $status))
  );
  (Ul::PrimerPack) => (
    tuple_to_vec!(partition_identifier!(Ul::PrimerPack))
  );
  (Ul::RandomIndexMetadata) => (
    tuple_to_vec!(partition_identifier!(Ul::RandomIndexMetadata))
  );
  (Ul::StaticTrack) => (
    tuple_to_vec!(smpte_identifier!(SmpteRegitery::Track, 0x3a))
  );
  (Ul::Track) => (
    tuple_to_vec!(smpte_identifier!(SmpteRegitery::Track, 0x3b))
  );
  (Ul::PictureItemMpegFrameWrappedPictureElement, $stream:tt) => (
    tuple_to_vec!(smpte_identifier!(SmpteRegitery::Essence, 0x15, 0x05, $stream))
  );
  (Ul::Jpeg2000FrameWrapped, $stream:tt) => (
    tuple_to_vec!(smpte_identifier!(SmpteRegitery::Essence, 0x15, 0x08, $stream))
  );
  (Ul::Jpeg2000ClipWrapped, $stream:tt) => (
    tuple_to_vec!(smpte_identifier!(SmpteRegitery::Essence, 0x15, 0x09, $stream))
  );
  (Ul::SoundItemDataWrappedSoundElement, $stream:tt) => (
    tuple_to_vec!(smpte_identifier!(SmpteRegitery::Essence, 0x16, 0x00, $stream))
  );
  (Ul::SoundItemWaveDataWrappedSoundElement, $stream:tt) => (
    tuple_to_vec!(smpte_identifier!(SmpteRegitery::Essence, 0x16, 0x00, $stream))
  );
  (Ul::SoundItemBwfDataWrappedSoundElement, $stream:tt) => (
    tuple_to_vec!(smpte_identifier!(SmpteRegitery::Essence, 0x06, 0x00, $stream))
  );
  (Ul::FillItem, $stream:tt) => (
    tuple_to_vec!(smpte_identifier!(SmpteRegitery::Fill, 0x00))
  );
  (Ul::FillItemAvid, $stream:tt) => (
    tuple_to_vec!(smpte_identifier!(SmpteRegitery::Fill, 0x00))
  );
}

macro_rules! ul_filter {
  (Ul::HeaderPartition) => (
    partition_identifier!(Ul::HeaderPartition)
  );
  (Ul::BodyPartition) => (
    partition_identifier!(Ul::BodyPartition)
  );
  (Ul::FooterPartition) => (
    partition_identifier!(Ul::FooterPartition)
  );
  (Ul::PrimerPack) => (
    partition_identifier!(Ul::PrimerPack)
  );
  (Ul::RandomIndexMetadata) => (
    partition_identifier!(Ul::RandomIndexMetadata)
  );
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
  (SmpteRegitery::Track, $x:tt) => (
    smpte_identifier!(0x02, 0x53, 0x01, 0x01, 0x0d, 0x01, 0x01, 0x01, 0x01, 0x01, $x, 0x00)
  );
  (SmpteRegitery::Essence, $x:tt, $y:tt, $stream:tt) => (
    smpte_identifier!(0x02, 0x02, 0x01, 0x01, 0x0d, 0x01, 0x03, 0x01, $x, 0x00, $y, $stream)
  );
  (SmpteRegitery::Fill, $x:tt) => (
    smpte_identifier!(0x01, 0x01, 0x01, 0x01, 0x03, 0x01, 0x02, 0x10, 0x01, 0x00, 0x00, 0x00)
  );

  ($x0:tt, $x1:tt, $x2:tt, $x3:tt, $x4:tt, $x5:tt, $x6:tt, $x7:tt, $x8:tt, $x9:tt, $x10:tt, $x11:tt) => (
    (0x06, 0x0e, 0x2b, 0x34, $x0, $x1, $x2, $x3, $x4, $x5, $x6, $x7, $x8, $x9, $x10, $x11)
  );
}
