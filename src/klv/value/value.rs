
use serializer::encoder::*;
use klv::ul::*;

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutCode {
    RedComponent,
    GreenComponent,
    BlueComponent,
    AlphaComponent,
    FillComponent,
    PaletteCode,
    UChromaSample,
    VChromaSample,
    WCompositeVideo,
    XNonCositedLuminanceComponent,
    YLuminanceComponent,
    ZDepthComponent,
    CompColorX,
    CompColorY,
    CompColorZ,
    Reserved,
}

pub fn get_layout(value: u8) -> LayoutCode {
  match value {
    0x52 => LayoutCode::RedComponent,
    0x47 => LayoutCode::GreenComponent,
    0x42 => LayoutCode::BlueComponent,
    0x41 => LayoutCode::AlphaComponent,
    0x72 => LayoutCode::RedComponent,
    0x67 => LayoutCode::GreenComponent,
    0x62 => LayoutCode::BlueComponent,
    0x61 => LayoutCode::AlphaComponent,
    0x46 => LayoutCode::FillComponent,
    0x50 => LayoutCode::PaletteCode,
    0x55 => LayoutCode::UChromaSample,
    0x56 => LayoutCode::VChromaSample,
    0x57 => LayoutCode::WCompositeVideo,
    0x58 => LayoutCode::XNonCositedLuminanceComponent,
    0x59 => LayoutCode::YLuminanceComponent,
    0x5A => LayoutCode::ZDepthComponent,
    0x75 => LayoutCode::UChromaSample,
    0x76 => LayoutCode::VChromaSample,
    0x77 => LayoutCode::WCompositeVideo,
    0x78 => LayoutCode::XNonCositedLuminanceComponent,
    0x79 => LayoutCode::YLuminanceComponent,
    0x7A => LayoutCode::ZDepthComponent,
    0xD8 => LayoutCode::CompColorX,
    0xD9 => LayoutCode::CompColorY,
    0xDA => LayoutCode::CompColorZ,
    _ => LayoutCode::Reserved,
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Layout {
  pub code: LayoutCode,
  pub bit_depth: u8
}

#[derive(Debug, Clone, PartialEq)]
pub enum Orientation {
  LeftToRightTopToBottom,
  RightToLeftTopToBottom,
  LeftToRightBottomToTop,
  RightToLeftBottomToTop,
  TopToBottomLeftToRight,
  TopToBottomRightToLeft,
  BottomToTopLeftToRight,
  BottomToTopRightToLeft,
  Reserved
}

#[derive(Debug, Clone, PartialEq)]
pub enum Mpeg2Profile {
  Simple,
  Main,
  SnrScalable,
  SpatiallyScalable,
  High,
  FourTwoTwo,
  Reserved
}

#[derive(Debug, Clone, PartialEq)]
pub enum Mpeg2Level {
  Low,
  Main,
  High1440,
  High,
  HighP,
  Reserved,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Mpeg2CodedContentType {
  Unknown,
  Progressive,
  Interlaced,
  Mixed,
}

#[derive(Debug, Clone, PartialEq)]
pub struct J2KComponent {
  pub s_siz: u8,
  pub xr_siz: u8,
  pub yr_siz: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeltaEntry {
  pub position_table_index: i8,
  pub slice: u8,
  pub element_delta: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rational {
  pub num: u32,
  pub den: u32
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndexEntry {
  pub temporal_offset: i8,
  pub key_frame_offset: i8,
  pub flags: u8,
  pub stream_offset: u64,
  pub slice_offset: Vec<u32>,
  pub position_table: Vec<Rational>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RandomIndexEntry {
  pub body_sid: u32,
  pub byte_offset: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DynamicTag {
  pub tag: u16,
  pub identifier: Ul
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueData {
  Boolean {
    data: bool
  },
  Int8 {
      data: i8
  },
  Int16 {
      data: i16
  },
  Uint8 {
    data: u8
  },
  Uint16 {
    data: u16
  },
  Uint32 {
    data: u32
  },
  Uint64 {
    data: u64
  },
  Length {
    data: u64
  },
  Position {
    data: u64
  },
  String {
    data: String
  },
  Ul {
    data: Ul
  },
  Uuid {
    data: String
  },
  Umid {
    data: String
  },
  PackageId {
    data: String
  },
  StrongRef {
    data: String
  },
  WeakRef {
    data: String
  },
  Rational {
    num: u64,
    den: u64
  },
  Version {
    major: u8,
    minor: u8
  },
  ProductVersion {
    major: u16,
    minor: u16,
    patch: u16,
    build: u16,
    release: u16,
  },
  Timestamp {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    quarter_of_milliseconds: u8,
  },
  ChannelLayout {
    data: Vec<Layout>
  },
  ArrayNumber {
    data: Vec<u64>
  },
  ArrayString {
    data: Vec<String>
  },
  ArrayUl {
    data: Vec<Ul>
  },
  Orientation {
    data: Orientation
  },
  ProfileAndLevel {
    profile: Mpeg2Profile,
    level: Mpeg2Level
  },
  CodedContentType {
    mode: Mpeg2CodedContentType
  },
  J2KComponentSizing {
    components: Vec<J2KComponent>
  },
  DeltaEntries {
    entries: Vec<DeltaEntry>
  },
  IndexEntries {
    entries: Vec<IndexEntry>
  },
  RandomIndexEntries {
    entries: Vec<RandomIndexEntry>
  },
  DynamicTags {
    entries: Vec<DynamicTag>
  },
  ContentData {
    address: u64,
    size: usize
  },
  Unknown {
    data: Vec<u8>
  }
}

#[derive(Debug, Clone, Copy)]
pub enum ValueDataType {
  Boolean,
  Uint8Array,
  BytesArray,
  Int8,
  Int16,
  Uint8,
  Uint16,
  Uint32,
  Uint32X2,
  Length,
  Position,
  Ul,
  UlBatch,
  String,
  Utf16,
  Uuid,
  Umid,
  Locators,
  Rational,
  Timestamp,
  PackageId,
  Stream,
  Version,
  ProductVersion,
  ChannelLayout,
  Orientation,
  DataValue,
  WeakRef,
  StrongRef,
  StrongRefArray,
  StrongRefBatch,
  TrackIdBatch,
  J2KComponentSizing,
  J2KCodingStyleDefault,
  J2KQuantisationDefault,
  DeltaEntries,
  IndexEntries,
  Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Element {
  pub identifier: Ul,
  pub value: Option<ValueData>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Value {
  pub elements: Vec<Element>
}

impl Encoder for Value {
  fn serialise(&self) -> Vec<u8> {
    let result = vec![];
    for _element in self.elements.clone() {
      // match element.identifier {
      //   _ => {
      //     unimplemented!()
      //   }
      // }
    }
    result
  }
}

// fn serialise_operational_pattern(item_complexity: u8, package_complexity: char, internal_essence: bool, stream_file: bool, uni_track: bool) -> Vec<u8> {
//   let mut result = get_smpte_identifier();
//   let mut op_identifier = vec![0x04, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01];

//   result.append(&mut op_identifier);
//   result.push(item_complexity);
//   match package_complexity {
//     'a' => result.push(0x01),
//     'b' => result.push(0x02),
//     'c' => result.push(0x03),
//     _ => panic!("unsupported package complexity")
//   };

//   let mut flags = 0x01;
//   flags += (uni_track as u8) << 4;
//   flags += (stream_file as u8) << 3;
//   flags += (internal_essence as u8) << 2;

//   result.push(flags);
//   result.push(0x00);
//   result
// }

// pub fn parse_operational_pattern(data: Vec<u8>) -> Option<ElementIdentifier> {
//   match (data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8], data[9], data[10], data[11], data[12], data[13], data[14], data[15]) {
//     (0x06, 0x0e, 0x2b, 0x34, 0x04, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, item_complexity, package_complexity_value, flags, _reserved) => {

//       let package_complexity =
//         match package_complexity_value {
//           0x01 => {'a'},
//           0x02 => {'b'},
//           0x03 => {'c'},
//           _ => {' '}
//         };

//       Some(
//         ElementIdentifier::PartitionOperationalPattern{
//           item_complexity: item_complexity,
//           package_complexity: package_complexity,
//           internal_essence: ((flags & 0b00000010) == 0),
//           stream_file: ((flags & 0b00000100) == 0),
//           uni_track: ((flags & 0b00001000) == 0),
//         }
//       )
//     },
//     (_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
//       None
//     }
//   }
// }

pub fn get_tag_identifier(id: u16, dynamic_tags: &mut Vec<DynamicTag>) -> Option<(Ul, ValueDataType)> {
  let (identifier, data_type) =
    match id {
      0x0102 => (Ul::GenerationIdentifier, ValueDataType::Uuid),
      0x0201 => (Ul::DataDefinition, ValueDataType::Ul),
      0x0202 => (Ul::Duration, ValueDataType::Length),
      0x0601 => (Ul::EventStartPosition, ValueDataType::Position),
      0x0602 => (Ul::EventComment, ValueDataType::Utf16),
      0x1001 => (Ul::StructuralComponents, ValueDataType::StrongRefArray),
      0x1101 => (Ul::SourcePackageId, ValueDataType::PackageId),
      0x1102 => (Ul::SourceTrackId, ValueDataType::Uint32),
      0x1201 => (Ul::StartPosition, ValueDataType::Position),
      0x1502 => (Ul::RoundedTimecodeBase, ValueDataType::Uint16),
      0x1501 => (Ul::StartTimecode, ValueDataType::Position),
      0x1503 => (Ul::DropFrame, ValueDataType::Boolean),
      0x1901 => (Ul::Packages, ValueDataType::StrongRefBatch),
      0x1902 => (Ul::EssenceContainerData, ValueDataType::StrongRefBatch),
      0x2701 => (Ul::LinkedPackageUid, ValueDataType::Umid),
      0x2F01 => (Ul::Locators, ValueDataType::Locators),
      0x3001 => (Ul::SampleRate, ValueDataType::Rational),
      0x3002 => (Ul::ContainerDuration, ValueDataType::Length),
      0x3004 => (Ul::EssenceContainer, ValueDataType::Ul),
      0x3005 => (Ul::Codec, ValueDataType::Ul),
      0x3006 => (Ul::LinkedTrackId, ValueDataType::Uint32),
      0x3201 => (Ul::PictureEssenceCoding, ValueDataType::Ul),
      0x3202 => (Ul::StoredHeight, ValueDataType::Uint32),
      0x3203 => (Ul::StoredWidth, ValueDataType::Uint32),
      0x3204 => (Ul::SampledHeight, ValueDataType::Uint32),
      0x3205 => (Ul::SampledWidth, ValueDataType::Uint32),
      0x3206 => (Ul::SampledXOffset, ValueDataType::Uint32),
      0x3207 => (Ul::SampledYOffset, ValueDataType::Uint32),
      0x3208 => (Ul::DisplayHeight, ValueDataType::Uint32),
      0x3209 => (Ul::DisplayWidth, ValueDataType::Uint32),
      0x320A => (Ul::DisplayXOffset, ValueDataType::Uint32),
      0x320B => (Ul::DisplayYOffset, ValueDataType::Uint32),
      0x320C => (Ul::FrameLayout, ValueDataType::Uint8),
      0x320D => (Ul::VideoLineMap, ValueDataType::Uint32X2),
      0x320E => (Ul::AspectRatio, ValueDataType::Rational),
      0x320F => (Ul::AlphaTransparency, ValueDataType::Uint8),
      0x3210 => (Ul::CaptureGamma, ValueDataType::Ul),
      0x3211 => (Ul::ImageAlignementOffset, ValueDataType::Uint32),
      0x3212 => (Ul::FieldDominance, ValueDataType::Uint8),
      0x3213 => (Ul::ImageStartOffset, ValueDataType::Uint32),
      0x3214 => (Ul::ImageEndOffset, ValueDataType::Uint32),
      0x3215 => (Ul::SignalStandard, ValueDataType::Uint8),
      0x3216 => (Ul::StoredF2Offset, ValueDataType::Uint32),
      0x3217 => (Ul::DisplayF2Offset, ValueDataType::Uint32),
      0x3218 => (Ul::ActiveFormatDescriptor, ValueDataType::Uint8),
      0x3219 => (Ul::ColorPrimaries, ValueDataType::Ul),
      0x321A => (Ul::CodingEquations, ValueDataType::Ul),
      0x3301 => (Ul::ComponentDepth, ValueDataType::Uint32),
      0x3302 => (Ul::HorizontalSubsampling, ValueDataType::Uint32),
      0x3303 => (Ul::ColorSiting, ValueDataType::Uint8),
      0x3304 => (Ul::BlackRefLevel, ValueDataType::Uint32),
      0x3305 => (Ul::WhiteRefLevel, ValueDataType::Uint32),
      0x3306 => (Ul::ColorRange, ValueDataType::Uint32),
      0x3307 => (Ul::PaddingBits, ValueDataType::Uint16),
      0x3308 => (Ul::VerticalSubsampling, ValueDataType::Uint32),
      0x3309 => (Ul::AlphaSampleDepth, ValueDataType::Uint32),
      0x330B => (Ul::ReversedByteOrder, ValueDataType::Boolean),
      0x3401 => (Ul::PixelLayout, ValueDataType::ChannelLayout),
      0x3403 => (Ul::Palette, ValueDataType::DataValue),
      0x3404 => (Ul::PaletteLayout, ValueDataType::ChannelLayout),
      0x3405 => (Ul::ScanningDirection, ValueDataType::Orientation),
      0x3406 => (Ul::ComponentMaxRef, ValueDataType::Uint32),
      0x3407 => (Ul::ComponentMinRef, ValueDataType::Uint32),
      0x3408 => (Ul::AlphaMaxRef, ValueDataType::Uint32),
      0x3409 => (Ul::AlphaMinRef, ValueDataType::Uint32),
      0x3B02 => (Ul::LastModifiedDate, ValueDataType::Timestamp),
      0x3B03 => (Ul::ContentStorage, ValueDataType::StrongRef),
      0x3B05 => (Ul::Version, ValueDataType::Version),
      0x3B06 => (Ul::Identifications, ValueDataType::StrongRefArray),
      0x3B07 => (Ul::ObjectModelVersion, ValueDataType::Uint32),
      0x3B08 => (Ul::PrimaryPackage, ValueDataType::WeakRef),
      0x3B09 => (Ul::OperationalPattern, ValueDataType::Ul),
      0x3B0A => (Ul::EssenceContainers, ValueDataType::UlBatch),
      0x3B0B => (Ul::DmSchemes, ValueDataType::UlBatch),
      0x3C01 => (Ul::CompanyName, ValueDataType::Utf16),
      0x3C02 => (Ul::ProductName, ValueDataType::Utf16),
      0x3C03 => (Ul::ProductVersion, ValueDataType::ProductVersion),
      0x3C04 => (Ul::VersionString, ValueDataType::Utf16),
      0x3C05 => (Ul::ProductUid, ValueDataType::Uuid),
      0x3C06 => (Ul::ModificationDate, ValueDataType::Timestamp),
      0x3C07 => (Ul::ToolkitVersion, ValueDataType::ProductVersion),
      0x3C08 => (Ul::Platforme, ValueDataType::Utf16),
      0x3C09 => (Ul::ThisGenerationUid, ValueDataType::Uuid),
      0x3C0A => (Ul::InstanceUid, ValueDataType::Uuid),
      0x3D01 => (Ul::QuantizationBits, ValueDataType::Uint32),
      0x3D02 => (Ul::LockedUnlocked, ValueDataType::Boolean),
      0x3D03 => (Ul::AudioSamplingRate, ValueDataType::Rational),
      0x3D04 => (Ul::AudioRefLevel, ValueDataType::Int8),
      0x3D05 => (Ul::ElectroSpatialFormulation, ValueDataType::Uint8),
      0x3D06 => (Ul::SoundEssenceCompression, ValueDataType::Ul),
      0x3D07 => (Ul::ChannelCount, ValueDataType::Uint32),
      0x3D08 => (Ul::AuxBitsMode, ValueDataType::Uint8),
      0x3D09 => (Ul::AverageBytesPerSecond, ValueDataType::Uint32),
      0x3D0A => (Ul::BlockAlign, ValueDataType::Uint16),
      0x3D0B => (Ul::SequenceOffset, ValueDataType::Uint8),
      0x3D0C => (Ul::DialNorm, ValueDataType::Int8),
      0x3D0D => (Ul::Emphasis, ValueDataType::Int8),
      0x3D0F => (Ul::BlockStartOffset, ValueDataType::Int16),
      0x3D10 => (Ul::ChannelStatusMode, ValueDataType::Uint8Array),
      0x3D11 => (Ul::FixedChannelStatusData, ValueDataType::BytesArray),
      0x3D12 => (Ul::UserDataMode, ValueDataType::Uint8Array),
      0x3D13 => (Ul::FixedUserData, ValueDataType::BytesArray),
      0x3D15 => (Ul::FileSecurityReport, ValueDataType::Uint32),
      0x3D16 => (Ul::FileSecurityWave, ValueDataType::Uint32),
      0x3D21 => (Ul::CodingHistory, ValueDataType::Utf16),
      0x3D22 => (Ul::BasicData, ValueDataType::Utf16),
      0x3D23 => (Ul::StartModulation, ValueDataType::Utf16),
      0x3D24 => (Ul::QualityEvent, ValueDataType::Utf16),
      0x3D25 => (Ul::EndModulation, ValueDataType::Utf16),
      0x3D26 => (Ul::QualityParameter, ValueDataType::Utf16),
      0x3D27 => (Ul::OperatorComment, ValueDataType::Utf16),
      0x3D28 => (Ul::CueSheet, ValueDataType::Utf16),
      0x3D29 => (Ul::PeakEnvelopeVersion, ValueDataType::Uint32),
      0x3D2A => (Ul::PeakEnvelopeFormat, ValueDataType::Uint32),
      0x3D2B => (Ul::PeakEnvelopeValue, ValueDataType::Uint32),
      0x3D2C => (Ul::PeakEnvelopeBlockSize, ValueDataType::Uint32),
      0x3D2D => (Ul::PeakChannels, ValueDataType::Uint32),
      0x3D2E => (Ul::PeakFrames, ValueDataType::Uint32),
      0x3D2F => (Ul::PeakOfPeaksPositions, ValueDataType::Position),
      0x3D30 => (Ul::PeakEnvelopeTimestamp, ValueDataType::Timestamp),
      0x3D31 => (Ul::PeakEnvelopeData, ValueDataType::Stream),
      0x3D32 => (Ul::ChannelAssignment, ValueDataType::Ul),
      0x3D33 => (Ul::UnknownBWFChunck, ValueDataType::StrongRefArray),
      0x3E01 => (Ul::DataEssenceCoding, ValueDataType::Ul),
      0x3F01 => (Ul::SubDescriptorUids, ValueDataType::StrongRefArray),

      0x3F05 => (Ul::EditUnitByteCount, ValueDataType::Uint32),
      0x3F06 => (Ul::IndexSid, ValueDataType::Uint32),
      0x3F07 => (Ul::BodySid, ValueDataType::Uint32),
      0x3F08 => (Ul::SliceCount, ValueDataType::Uint8),
      0x3F09 => (Ul::DeltaEntryArray, ValueDataType::DeltaEntries),
      0x3F0A => (Ul::IndexEntryArray, ValueDataType::IndexEntries),
      0x3F0B => (Ul::IndexEditRate, ValueDataType::Rational),
      0x3F0C => (Ul::IndexStartPosition, ValueDataType::Position),
      0x3F0D => (Ul::IndexDuration, ValueDataType::Length),
      0x3F0E => (Ul::PositionTableCount, ValueDataType::Uint8),

      0x4001 => (Ul::UrlString, ValueDataType::Utf16),
      0x4101 => (Ul::LocatorName, ValueDataType::Utf16),
      0x4401 => (Ul::PackageUid, ValueDataType::Umid),
      0x4402 => (Ul::Name, ValueDataType::Utf16),
      0x4403 => (Ul::Tracks, ValueDataType::StrongRefArray),
      0x4404 => (Ul::PackageModifiedDate, ValueDataType::Timestamp),
      0x4405 => (Ul::PackageCreationDate, ValueDataType::Timestamp),
      0x4701 => (Ul::Descriptor, ValueDataType::StrongRef),
      0x4801 => (Ul::TrackId, ValueDataType::Uint32),
      0x4802 => (Ul::TrackName, ValueDataType::String),
      0x4804 => (Ul::TrackNumber, ValueDataType::Uint32),
      0x4B01 => (Ul::EditRate, ValueDataType::Rational),
      0x4B02 => (Ul::Origin, ValueDataType::Position),
      0x4803 => (Ul::Sequence, ValueDataType::StrongRef),
      0x4901 => (Ul::EventEditRate, ValueDataType::Rational),
      0x4902 => (Ul::EventOrigin, ValueDataType::Position),
      0x4F01 => (Ul::ChunkId, ValueDataType::Uint32),
      0x4F02 => (Ul::ChunkLength, ValueDataType::Uint32),
      0x4F03 => (Ul::ChunkData, ValueDataType::Stream),
      0x6101 => (Ul::DMSegmentDMFramework, ValueDataType::StrongRef),
      0x6102 => (Ul::TrackIds, ValueDataType::TrackIdBatch),
      _      => {
        for dynamic_tag in dynamic_tags {
          // println!("{:?}", dynamic_tag);
          if dynamic_tag.tag == id {
            match dynamic_tag.identifier {
              Ul::IsRipPresent => return Some((Ul::IsRipPresent, ValueDataType::Boolean)),
              Ul::SubDescriptors => return Some((Ul::SubDescriptors, ValueDataType::StrongRefArray)),

              Ul::GenericPictureEssenceDescriptor_ActiveWidth => return Some((Ul::GenericPictureEssenceDescriptor_ActiveWidth, ValueDataType::Uint32)),
              Ul::GenericPictureEssenceDescriptor_ActiveHeight => return Some((Ul::GenericPictureEssenceDescriptor_ActiveWidth, ValueDataType::Uint32)),
              Ul::GenericPictureEssenceDescriptor_ActiveXOffset => return Some((Ul::GenericPictureEssenceDescriptor_ActiveWidth, ValueDataType::Uint32)),
              Ul::GenericPictureEssenceDescriptor_ActiveYOffset => return Some((Ul::GenericPictureEssenceDescriptor_ActiveWidth, ValueDataType::Uint32)),

              Ul::MCALabelDictionaryID => return Some((Ul::MCALabelDictionaryID, ValueDataType::Ul)),
              Ul::MCATagSymbol => return Some((Ul::MCATagSymbol, ValueDataType::Utf16)),
              Ul::MCATagName => return Some((Ul::MCATagName, ValueDataType::Utf16)),
              // Ul::GroupOfSoundfieldGroupsLinkID => return Some((Ul::GroupOfSoundfieldGroupsLinkID, ValueDataType::UuidArray)),
              Ul::MCALinkID => return Some((Ul::MCALinkID, ValueDataType::Uuid)),
              Ul::SoundfieldGroupLinkID => return Some((Ul::SoundfieldGroupLinkID, ValueDataType::Uuid)),
              Ul::MCAChannelID => return Some((Ul::MCAChannelID, ValueDataType::Uint32)),
              Ul::MCATitle => return Some((Ul::MCATitle, ValueDataType::Utf16)),
              Ul::MCATitleVersion => return Some((Ul::MCATitleVersion, ValueDataType::Utf16)),
              Ul::MCATitleSubVersion => return Some((Ul::MCATitleSubVersion, ValueDataType::Utf16)),
              Ul::MCAEpisode => return Some((Ul::MCAEpisode, ValueDataType::Utf16)),
              Ul::MCAPartitionKind => return Some((Ul::MCAPartitionKind, ValueDataType::Utf16)),
              Ul::MCAPartitionNumber => return Some((Ul::MCAPartitionNumber, ValueDataType::Utf16)),
              Ul::MCAAudioContentKind => return Some((Ul::MCAAudioContentKind, ValueDataType::Utf16)),
              Ul::MCAAudioElementKind => return Some((Ul::MCAAudioElementKind, ValueDataType::Utf16)),
              Ul::RFC5646SpokenLanguage => return Some((Ul::RFC5646SpokenLanguage, ValueDataType::String)),

              Ul::Mpeg2VideoDescriptorSingleSequence => return Some((Ul::Mpeg2VideoDescriptorSingleSequence, ValueDataType::Boolean)),
              Ul::Mpeg2VideoDescriptorCodedContentType => return Some((Ul::Mpeg2VideoDescriptorCodedContentType, ValueDataType::Uint8)),
              Ul::Mpeg2VideoDescriptorBPictureCount => return Some((Ul::Mpeg2VideoDescriptorBPictureCount, ValueDataType::Uint16)),
              Ul::Mpeg2VideoDescriptorProfileAndLevel => return Some((Ul::Mpeg2VideoDescriptorProfileAndLevel, ValueDataType::Uint8)),
              Ul::Mpeg2VideoDescriptorLowDelay => return Some((Ul::Mpeg2VideoDescriptorLowDelay, ValueDataType::Boolean)),
              Ul::Mpeg2VideoDescriptorMaxGOP => return Some((Ul::Mpeg2VideoDescriptorMaxGOP, ValueDataType::Uint16)),
              Ul::Mpeg2VideoDescriptorConstantBframes => return Some((Ul::Mpeg2VideoDescriptorConstantBframes, ValueDataType::Boolean)),
              Ul::Mpeg2VideoDescriptorClosedGOP => return Some((Ul::Mpeg2VideoDescriptorClosedGOP, ValueDataType::Boolean)),
              Ul::Mpeg2VideoDescriptorIdenticalGOP => return Some((Ul::Mpeg2VideoDescriptorIdenticalGOP, ValueDataType::Boolean)),
              Ul::Mpeg2VideoDescriptorBitRate => return Some((Ul::Mpeg2VideoDescriptorBitRate, ValueDataType::Uint32)),

              Ul::Jpeg2000VideoDescriptor_Rsiz => return Some((Ul::Jpeg2000VideoDescriptor_Rsiz, ValueDataType::Uint16)),
              Ul::Jpeg2000VideoDescriptor_Xsiz => return Some((Ul::Jpeg2000VideoDescriptor_Xsiz, ValueDataType::Uint32)),
              Ul::Jpeg2000VideoDescriptor_Ysiz => return Some((Ul::Jpeg2000VideoDescriptor_Ysiz, ValueDataType::Uint32)),
              Ul::Jpeg2000VideoDescriptor_XOsiz => return Some((Ul::Jpeg2000VideoDescriptor_XOsiz, ValueDataType::Uint32)),
              Ul::Jpeg2000VideoDescriptor_YOsiz => return Some((Ul::Jpeg2000VideoDescriptor_YOsiz, ValueDataType::Uint32)),
              Ul::Jpeg2000VideoDescriptor_XTsiz => return Some((Ul::Jpeg2000VideoDescriptor_XTsiz, ValueDataType::Uint32)),
              Ul::Jpeg2000VideoDescriptor_YTsiz => return Some((Ul::Jpeg2000VideoDescriptor_YTsiz, ValueDataType::Uint32)),
              Ul::Jpeg2000VideoDescriptor_XTOsiz => return Some((Ul::Jpeg2000VideoDescriptor_XTOsiz, ValueDataType::Uint32)),
              Ul::Jpeg2000VideoDescriptor_YTOsiz => return Some((Ul::Jpeg2000VideoDescriptor_YTOsiz, ValueDataType::Uint32)),
              Ul::Jpeg2000VideoDescriptor_Csiz => return Some((Ul::Jpeg2000VideoDescriptor_Csiz, ValueDataType::Uint16)),
              Ul::Jpeg2000VideoDescriptor_PictureComponentSizing => return Some((Ul::Jpeg2000VideoDescriptor_PictureComponentSizing, ValueDataType::J2KComponentSizing)),
              Ul::Jpeg2000VideoDescriptor_CodingStyleDefault => return Some((Ul::Jpeg2000VideoDescriptor_CodingStyleDefault, ValueDataType::Unknown)),
              Ul::Jpeg2000VideoDescriptor_QuantizationDefault => return Some((Ul::Jpeg2000VideoDescriptor_QuantizationDefault, ValueDataType::Unknown)),
              Ul::Jpeg2000VideoDescriptor_J2CLayout => return Some((Ul::Jpeg2000VideoDescriptor_J2CLayout, ValueDataType::ChannelLayout)),

              _ => {

                println!("dynamic tag {:0x} not supported: {:?}", id, dynamic_tag.identifier);
                return None
              }
            }
          }
        }
        println!("unknown tag with id 0x{:x}", id);
        return None
      }
    };

  Some((identifier, data_type))
}
