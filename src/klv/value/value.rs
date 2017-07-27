
use byteorder::{BigEndian, WriteBytesExt};
use serializer::encoder::*;

use klv::ul::*;
use klv::value::tag::*;
use klv::value::essence_identifiers::*;

fn get_smpte_identifier() -> Vec<u8> {
  vec![0x06, 0x0e, 0x2b, 0x34]
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ElementIdentifier {
  PartitionMajor{ value: u16 },
  PartitionMinor{ value: u16 },
  PartitionKagSize{ size: u32 },
  PartitionThisPartition{ offset: u64 },
  PartitionPreviousPartition{ offset: u64 },
  PartitionFooterPartition{ offset: u64 },
  PartitionHeaderByteCount{ size: u64 },
  PartitionIndexByteCount{ size: u64 },
  PartitionIndexSid{ value: u32 },
  PartitionByteOffset{ offset: u64 },
  PartitionBodySid{ value: u32 },
  PartitionOperationalPattern{
    item_complexity: u8,
    package_complexity: char,
    internal_essence: bool,
    stream_file: bool,
    uni_track: bool,
  },
  PartitionEssenceContainers{essences: Vec<EssenceIdentifier>},
  PrimerPack{data: Vec<DynamicTagList>},

  Data {data: Vec<u8>},
  ContentData {
    address: usize,
    size: usize
  },


  GenerationIdentifier,
  DataDefinition,
  Duration,
  EventStartPosition,
  EventComment,
  StructuralComponents,
  SourcePackageId,
  SourceTrackId,
  StartPosition,
  RoundedTimecodeBase,
  StartTimecode,
  DropFrame,
  Packages,
  EssenceContainerData,
  LinkedPackageUid,
  Locators,
  SampleRate,
  ContainerDuration,
  EssenceContainer,
  Codec,
  LinkedTrackId,
  PictureEssenceCoding,
  StoredHeight,
  StoredWidth,
  SampledHeight,
  SampledWidth,
  SampledXOffset,
  SampledYOffset,
  DisplayHeight,
  DisplayWidth,
  DisplayXOffset,
  DisplayYOffset,
  FrameLayout,
  VideoLineMap,
  AspectRatio,
  AlphaTransparency,
  CaptureGamma,
  ImageAlignementOffset,
  FieldDominance,
  ImageStartOffset,
  ImageEndOffset,
  SignalStandard,
  StoredF2Offset,
  DisplayF2Offset,
  ActiveFormatDescriptor,
  ColorPrimaries,
  CodingEquations,
  ComponentDepth,
  HorizontalSubsampling,
  ColorSiting,
  BlackRefLevel,
  WhiteRefLevel,
  ColorRange,
  PaddingBits,
  VerticalSubsampling,
  AlphaSampleDepth,
  ReversedByteOrder,
  PixelLayout,
  Palette,
  PaletteLayout,
  ScanningDirection,
  ComponentMaxRef,
  ComponentMinRef,
  AlphaMaxRef,
  AlphaMinRef,
  LastModifiedDate,
  ContentStorage,
  Version,
  Identifications,
  ObjectModelVersion,
  PrimaryPackage,
  OperationalPattern,
  EssenceContainers,
  DmSchemes,
  CompanyName,
  ProductName,
  ProductVersion,
  VersionString,
  ProductUid,
  ModificationDate,
  ToolkitVersion,
  Platforme,
  ThisGenerationUid,
  InstanceUid,
  QuantizationBits,
  LockedUnlocked,
  AudioSamplingRate,
  AudioRefLevel,
  ElectroSpatialFormulation,
  SoundEssenceCompression,
  ChannelCount,
  AuxBitsMode,
  AverageBytesPerSecond,
  BlockAlign,
  SequenceOffset,
  DialNorm,
  Emphasis,
  BlockStartOffset,
  ChannelStatusMode,
  FixedChannelStatusData,
  UserDataMode,
  FixedUserData,
  FileSecurityReport,
  FileSecurityWave,
  CodingHistory,
  BasicData,
  StartModulation,
  QualityEvent,
  EndModulation,
  QualityParameter,
  OperatorComment,
  CueSheet,
  PeakEnvelopeVersion,
  PeakEnvelopeFormat,
  PeakEnvelopeValue,
  PeakEnvelopeBlockSize,
  PeakChannels,
  PeakFrames,
  PeakOfPeaksPositions,
  PeakEnvelopeTimestamp,
  PeakEnvelopeData,
  ChannelAssignment,
  UnknownBWFChunck,
  DataEssenceCoding,
  SubDescriptorUids,

  EditUnitByteCount,
  IndexSid,
  BodySid,
  SliceCount,
  DeltaEntryArray,
  IndexEntryArray,
  IndexEditRate,
  IndexStartPosition,
  IndexDuration,
  PositionTableCount,
  RandomIndexMetadata,

  UrlString,
  LocatorName,
  PackageUid,
  Name,
  Tracks,
  PackageModifiedDate,
  PackageCreationDate,
  Descriptor,
  TrackId,
  TrackName,
  TrackNumber,
  EditRate,
  Origin,
  Sequence,
  EventEditRate,
  EventOrigin,
  ChunkId,
  ChunkLength,
  ChunkData,
  DmFramework,
  TrackIds,

  IsRipPresent,
  SubDescriptors,

  MCALabelDictionaryID,
  MCATagSymbol,
  MCATagName,
  GroupOfSoundfieldGroupsLinkID,
  MCALinkID,
  SoundfieldGroupLinkID,
  MCAChannelID,
  MCATitle,
  MCATitleVersion,
  MCATitleSubVersion,
  MCAEpisode,
  MCAPartitionKind,
  MCAPartitionNumber,
  MCAAudioContentKind,
  MCAAudioElementKind,
  RFC5646SpokenLanguage,

  Mpeg2VideoDescriptorSingleSequence,
  Mpeg2VideoDescriptorCodedContentType,
  Mpeg2VideoDescriptorBPictureCount,
  Mpeg2VideoDescriptorProfileAndLevel,
  Mpeg2VideoDescriptorLowDelay,
  Mpeg2VideoDescriptorMaxGOP,
  Mpeg2VideoDescriptorConstantBframes,
  Mpeg2VideoDescriptorClosedGOP,
  Mpeg2VideoDescriptorIdenticalGOP,
  Mpeg2VideoDescriptorBitRate,

  Jpeg2000VideoDescriptor_Rsiz,
  Jpeg2000VideoDescriptor_Xsiz,
  Jpeg2000VideoDescriptor_Ysiz,
  Jpeg2000VideoDescriptor_XOsiz,
  Jpeg2000VideoDescriptor_YOsiz,
  Jpeg2000VideoDescriptor_XTsiz,
  Jpeg2000VideoDescriptor_YTsiz,
  Jpeg2000VideoDescriptor_XTOsiz,
  Jpeg2000VideoDescriptor_YTOsiz,
  Jpeg2000VideoDescriptor_Csiz,
  Jpeg2000VideoDescriptor_PictureComponentSizing,
  Jpeg2000VideoDescriptor_CodingStyleDefault,
  Jpeg2000VideoDescriptor_QuantizationDefault,
  Jpeg2000VideoDescriptor_J2CLayout,
}

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
    data: String
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
  RgbaLayout {
    data: Vec<Layout>
  },
  ArrayNumber {
    data: Vec<u64>
  },
  ArrayString {
    data: Vec<String>
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
  Unknown {
    identifier: Vec<u8>
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
  RgbaLayout,
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
  pub identifier: ElementIdentifier,
  pub value: Option<ValueData>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Value {
  pub elements: Vec<Element>
}

impl Encoder for Value {
  fn serialise(&self) -> Vec<u8> {
    let mut result = vec![];
    for element in self.elements.clone() {
      match element.identifier {
        ElementIdentifier::PartitionMajor{value} => {
          result.write_u16::<BigEndian>(value).unwrap();
        },
        ElementIdentifier::PartitionMinor{value} => {
          result.write_u16::<BigEndian>(value).unwrap();
        },
        ElementIdentifier::PartitionKagSize{size} => {
          result.write_u32::<BigEndian>(size).unwrap();
        },
        ElementIdentifier::PartitionThisPartition{offset} => {
          result.write_u64::<BigEndian>(offset).unwrap();
        },
        ElementIdentifier::PartitionPreviousPartition{offset} => {
          result.write_u64::<BigEndian>(offset).unwrap();
        },
        ElementIdentifier::PartitionFooterPartition{offset} => {
          result.write_u64::<BigEndian>(offset).unwrap();
        },
        ElementIdentifier::PartitionHeaderByteCount{size} => {
          result.write_u64::<BigEndian>(size).unwrap();
        },
        ElementIdentifier::PartitionIndexByteCount{size} => {
          result.write_u64::<BigEndian>(size).unwrap();
        },
        ElementIdentifier::PartitionIndexSid{value} => {
          result.write_u32::<BigEndian>(value).unwrap();
        },
        ElementIdentifier::PartitionByteOffset{offset} => {
          result.write_u64::<BigEndian>(offset).unwrap();
        },
        ElementIdentifier::PartitionBodySid{value} => {
          result.write_u32::<BigEndian>(value).unwrap();
        },
        ElementIdentifier::PartitionOperationalPattern{item_complexity, package_complexity, internal_essence, stream_file, uni_track} => {
          let mut op = serialise_operational_pattern(item_complexity, package_complexity, internal_essence, stream_file, uni_track);
          result.append(&mut op);
        },
        ElementIdentifier::PartitionEssenceContainers{essences} => {
          result.write_u32::<BigEndian>(essences.len() as u32).unwrap();
          result.write_u32::<BigEndian>(16).unwrap();
          for _ul in essences {
            // TODO wrote essence UL
          }
        },
        // ElementIdentifier::InstanceUid{uuid} => {
        //   let tag = Tag{id: [0x3c, 0x0a], data: uuid};
        //   let mut content = Encoder::serialise(&tag);
        //   result.append(&mut content);
        // },
        // ElementIdentifier::GenerationUid{uuid} => {
        //   let tag = Tag{id: [0x01, 0x02], data: uuid};
        //   let mut content = Encoder::serialise(&tag);
        //   result.append(&mut content);
        // },
        ElementIdentifier::PrimerPack{data: _data} => {
          unimplemented!();
        },
        ElementIdentifier::Data{data} => {
          let mut d = data;
          result.append(&mut d);
        },
        ElementIdentifier::ContentData{..} => {
          panic!("unable to wrap a key based on Content Data kind. Missing data.");
        },
        _ => {
          unimplemented!()
        }
      }
    }
    result
  }
}

fn serialise_operational_pattern(item_complexity: u8, package_complexity: char, internal_essence: bool, stream_file: bool, uni_track: bool) -> Vec<u8> {
  let mut result = get_smpte_identifier();
  let mut op_identifier = vec![0x04, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01];

  result.append(&mut op_identifier);
  result.push(item_complexity);
  match package_complexity {
    'a' => result.push(0x01),
    'b' => result.push(0x02),
    'c' => result.push(0x03),
    _ => panic!("unsupported package complexity")
  };

  let mut flags = 0x01;
  flags += (uni_track as u8) << 4;
  flags += (stream_file as u8) << 3;
  flags += (internal_essence as u8) << 2;

  result.push(flags);
  result.push(0x00);
  result
}

pub fn parse_operational_pattern(data: Vec<u8>) -> Option<ElementIdentifier> {
  match (data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8], data[9], data[10], data[11], data[12], data[13], data[14], data[15]) {
    (0x06, 0x0e, 0x2b, 0x34, 0x04, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, item_complexity, package_complexity_value, flags, _reserved) => {

      let package_complexity =
        match package_complexity_value {
          0x01 => {'a'},
          0x02 => {'b'},
          0x03 => {'c'},
          _ => {' '}
        };

      Some(
        ElementIdentifier::PartitionOperationalPattern{
          item_complexity: item_complexity,
          package_complexity: package_complexity,
          internal_essence: ((flags & 0b00000010) == 0),
          stream_file: ((flags & 0b00000100) == 0),
          uni_track: ((flags & 0b00001000) == 0),
        }
      )
    },
    (_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
      None
    }
  }
}

pub fn get_tag_identifier(id: u16, dynamic_tags: &mut Vec<DynamicTagList>) -> Option<(ElementIdentifier, ValueDataType)> {
  let (identifier, data_type) =
    match id {
      0x0102 => (ElementIdentifier::GenerationIdentifier, ValueDataType::Uuid),
      0x0201 => (ElementIdentifier::DataDefinition, ValueDataType::Ul),
      0x0202 => (ElementIdentifier::Duration, ValueDataType::Length),
      0x0601 => (ElementIdentifier::EventStartPosition, ValueDataType::Position),
      0x0602 => (ElementIdentifier::EventComment, ValueDataType::Utf16),
      0x1001 => (ElementIdentifier::StructuralComponents, ValueDataType::StrongRefArray),
      0x1101 => (ElementIdentifier::SourcePackageId, ValueDataType::PackageId),
      0x1102 => (ElementIdentifier::SourceTrackId, ValueDataType::Uint32),
      0x1201 => (ElementIdentifier::StartPosition, ValueDataType::Position),
      0x1502 => (ElementIdentifier::RoundedTimecodeBase, ValueDataType::Uint16),
      0x1501 => (ElementIdentifier::StartTimecode, ValueDataType::Position),
      0x1503 => (ElementIdentifier::DropFrame, ValueDataType::Boolean),
      0x1901 => (ElementIdentifier::Packages, ValueDataType::StrongRefBatch),
      0x1902 => (ElementIdentifier::EssenceContainerData, ValueDataType::StrongRefBatch),
      0x2701 => (ElementIdentifier::LinkedPackageUid, ValueDataType::Umid),
      0x2F01 => (ElementIdentifier::Locators, ValueDataType::Locators),
      0x3001 => (ElementIdentifier::SampleRate, ValueDataType::Rational),
      0x3002 => (ElementIdentifier::ContainerDuration, ValueDataType::Length),
      0x3004 => (ElementIdentifier::EssenceContainer, ValueDataType::Ul),
      0x3005 => (ElementIdentifier::Codec, ValueDataType::Ul),
      0x3006 => (ElementIdentifier::LinkedTrackId, ValueDataType::Uint32),
      0x3201 => (ElementIdentifier::PictureEssenceCoding, ValueDataType::Ul),
      0x3202 => (ElementIdentifier::StoredHeight, ValueDataType::Uint32),
      0x3203 => (ElementIdentifier::StoredWidth, ValueDataType::Uint32),
      0x3204 => (ElementIdentifier::SampledHeight, ValueDataType::Uint32),
      0x3205 => (ElementIdentifier::SampledWidth, ValueDataType::Uint32),
      0x3206 => (ElementIdentifier::SampledXOffset, ValueDataType::Uint32),
      0x3207 => (ElementIdentifier::SampledYOffset, ValueDataType::Uint32),
      0x3208 => (ElementIdentifier::DisplayHeight, ValueDataType::Uint32),
      0x3209 => (ElementIdentifier::DisplayWidth, ValueDataType::Uint32),
      0x320A => (ElementIdentifier::DisplayXOffset, ValueDataType::Uint32),
      0x320B => (ElementIdentifier::DisplayYOffset, ValueDataType::Uint32),
      0x320C => (ElementIdentifier::FrameLayout, ValueDataType::Uint8),
      0x320D => (ElementIdentifier::VideoLineMap, ValueDataType::Uint32X2),
      0x320E => (ElementIdentifier::AspectRatio, ValueDataType::Rational),
      0x320F => (ElementIdentifier::AlphaTransparency, ValueDataType::Uint8),
      0x3210 => (ElementIdentifier::CaptureGamma, ValueDataType::Ul),
      0x3211 => (ElementIdentifier::ImageAlignementOffset, ValueDataType::Uint32),
      0x3212 => (ElementIdentifier::FieldDominance, ValueDataType::Uint8),
      0x3213 => (ElementIdentifier::ImageStartOffset, ValueDataType::Uint32),
      0x3214 => (ElementIdentifier::ImageEndOffset, ValueDataType::Uint32),
      0x3215 => (ElementIdentifier::SignalStandard, ValueDataType::Uint8),
      0x3216 => (ElementIdentifier::StoredF2Offset, ValueDataType::Uint32),
      0x3217 => (ElementIdentifier::DisplayF2Offset, ValueDataType::Uint32),
      0x3218 => (ElementIdentifier::ActiveFormatDescriptor, ValueDataType::Uint8),
      0x3219 => (ElementIdentifier::ColorPrimaries, ValueDataType::Ul),
      0x321A => (ElementIdentifier::CodingEquations, ValueDataType::Ul),
      0x3301 => (ElementIdentifier::ComponentDepth, ValueDataType::Uint32),
      0x3302 => (ElementIdentifier::HorizontalSubsampling, ValueDataType::Uint32),
      0x3303 => (ElementIdentifier::ColorSiting, ValueDataType::Uint8),
      0x3304 => (ElementIdentifier::BlackRefLevel, ValueDataType::Uint32),
      0x3305 => (ElementIdentifier::WhiteRefLevel, ValueDataType::Uint32),
      0x3306 => (ElementIdentifier::ColorRange, ValueDataType::Uint32),
      0x3307 => (ElementIdentifier::PaddingBits, ValueDataType::Uint16),
      0x3308 => (ElementIdentifier::VerticalSubsampling, ValueDataType::Uint32),
      0x3309 => (ElementIdentifier::AlphaSampleDepth, ValueDataType::Uint32),
      0x330B => (ElementIdentifier::ReversedByteOrder, ValueDataType::Boolean),
      0x3401 => (ElementIdentifier::PixelLayout, ValueDataType::RgbaLayout),
      0x3403 => (ElementIdentifier::Palette, ValueDataType::DataValue),
      0x3404 => (ElementIdentifier::PaletteLayout, ValueDataType::RgbaLayout),
      0x3405 => (ElementIdentifier::ScanningDirection, ValueDataType::Orientation),
      0x3406 => (ElementIdentifier::ComponentMaxRef, ValueDataType::Uint32),
      0x3407 => (ElementIdentifier::ComponentMinRef, ValueDataType::Uint32),
      0x3408 => (ElementIdentifier::AlphaMaxRef, ValueDataType::Uint32),
      0x3409 => (ElementIdentifier::AlphaMinRef, ValueDataType::Uint32),
      0x3B02 => (ElementIdentifier::LastModifiedDate, ValueDataType::Timestamp),
      0x3B03 => (ElementIdentifier::ContentStorage, ValueDataType::StrongRef),
      0x3B05 => (ElementIdentifier::Version, ValueDataType::Version),
      0x3B06 => (ElementIdentifier::Identifications, ValueDataType::StrongRefArray),
      0x3B07 => (ElementIdentifier::ObjectModelVersion, ValueDataType::Uint32),
      0x3B08 => (ElementIdentifier::PrimaryPackage, ValueDataType::WeakRef),
      0x3B09 => (ElementIdentifier::OperationalPattern, ValueDataType::Ul),
      0x3B0A => (ElementIdentifier::EssenceContainers, ValueDataType::UlBatch),
      0x3B0B => (ElementIdentifier::DmSchemes, ValueDataType::UlBatch),
      0x3C01 => (ElementIdentifier::CompanyName, ValueDataType::Utf16),
      0x3C02 => (ElementIdentifier::ProductName, ValueDataType::Utf16),
      0x3C03 => (ElementIdentifier::ProductVersion, ValueDataType::ProductVersion),
      0x3C04 => (ElementIdentifier::VersionString, ValueDataType::Utf16),
      0x3C05 => (ElementIdentifier::ProductUid, ValueDataType::Uuid),
      0x3C06 => (ElementIdentifier::ModificationDate, ValueDataType::Timestamp),
      0x3C07 => (ElementIdentifier::ToolkitVersion, ValueDataType::ProductVersion),
      0x3C08 => (ElementIdentifier::Platforme, ValueDataType::Utf16),
      0x3C09 => (ElementIdentifier::ThisGenerationUid, ValueDataType::Uuid),
      0x3C0A => (ElementIdentifier::InstanceUid, ValueDataType::Uuid),
      0x3D01 => (ElementIdentifier::QuantizationBits, ValueDataType::Uint32),
      0x3D02 => (ElementIdentifier::LockedUnlocked, ValueDataType::Boolean),
      0x3D03 => (ElementIdentifier::AudioSamplingRate, ValueDataType::Rational),
      0x3D04 => (ElementIdentifier::AudioRefLevel, ValueDataType::Int8),
      0x3D05 => (ElementIdentifier::ElectroSpatialFormulation, ValueDataType::Uint8),
      0x3D06 => (ElementIdentifier::SoundEssenceCompression, ValueDataType::Ul),
      0x3D07 => (ElementIdentifier::ChannelCount, ValueDataType::Uint32),
      0x3D08 => (ElementIdentifier::AuxBitsMode, ValueDataType::Uint8),
      0x3D09 => (ElementIdentifier::AverageBytesPerSecond, ValueDataType::Uint32),
      0x3D0A => (ElementIdentifier::BlockAlign, ValueDataType::Uint16),
      0x3D0B => (ElementIdentifier::SequenceOffset, ValueDataType::Uint8),
      0x3D0C => (ElementIdentifier::DialNorm, ValueDataType::Int8),
      0x3D0D => (ElementIdentifier::Emphasis, ValueDataType::Int8),
      0x3D0F => (ElementIdentifier::BlockStartOffset, ValueDataType::Int16),
      0x3D10 => (ElementIdentifier::ChannelStatusMode, ValueDataType::Uint8Array),
      0x3D11 => (ElementIdentifier::FixedChannelStatusData, ValueDataType::BytesArray),
      0x3D12 => (ElementIdentifier::UserDataMode, ValueDataType::Uint8Array),
      0x3D13 => (ElementIdentifier::FixedUserData, ValueDataType::BytesArray),
      0x3D15 => (ElementIdentifier::FileSecurityReport, ValueDataType::Uint32),
      0x3D16 => (ElementIdentifier::FileSecurityWave, ValueDataType::Uint32),
      0x3D21 => (ElementIdentifier::CodingHistory, ValueDataType::Utf16),
      0x3D22 => (ElementIdentifier::BasicData, ValueDataType::Utf16),
      0x3D23 => (ElementIdentifier::StartModulation, ValueDataType::Utf16),
      0x3D24 => (ElementIdentifier::QualityEvent, ValueDataType::Utf16),
      0x3D25 => (ElementIdentifier::EndModulation, ValueDataType::Utf16),
      0x3D26 => (ElementIdentifier::QualityParameter, ValueDataType::Utf16),
      0x3D27 => (ElementIdentifier::OperatorComment, ValueDataType::Utf16),
      0x3D28 => (ElementIdentifier::CueSheet, ValueDataType::Utf16),
      0x3D29 => (ElementIdentifier::PeakEnvelopeVersion, ValueDataType::Uint32),
      0x3D2A => (ElementIdentifier::PeakEnvelopeFormat, ValueDataType::Uint32),
      0x3D2B => (ElementIdentifier::PeakEnvelopeValue, ValueDataType::Uint32),
      0x3D2C => (ElementIdentifier::PeakEnvelopeBlockSize, ValueDataType::Uint32),
      0x3D2D => (ElementIdentifier::PeakChannels, ValueDataType::Uint32),
      0x3D2E => (ElementIdentifier::PeakFrames, ValueDataType::Uint32),
      0x3D2F => (ElementIdentifier::PeakOfPeaksPositions, ValueDataType::Position),
      0x3D30 => (ElementIdentifier::PeakEnvelopeTimestamp, ValueDataType::Timestamp),
      0x3D31 => (ElementIdentifier::PeakEnvelopeData, ValueDataType::Stream),
      0x3D32 => (ElementIdentifier::ChannelAssignment, ValueDataType::Ul),
      0x3D33 => (ElementIdentifier::UnknownBWFChunck, ValueDataType::StrongRefArray),
      0x3E01 => (ElementIdentifier::DataEssenceCoding, ValueDataType::Ul),
      0x3F01 => (ElementIdentifier::SubDescriptorUids, ValueDataType::StrongRefArray),

      0x3F05 => (ElementIdentifier::EditUnitByteCount, ValueDataType::Uint32),
      0x3F06 => (ElementIdentifier::IndexSid, ValueDataType::Uint32),
      0x3F07 => (ElementIdentifier::BodySid, ValueDataType::Uint32),
      0x3F08 => (ElementIdentifier::SliceCount, ValueDataType::Uint8),
      0x3F09 => (ElementIdentifier::DeltaEntryArray, ValueDataType::DeltaEntries),
      0x3F0A => (ElementIdentifier::IndexEntryArray, ValueDataType::IndexEntries),
      0x3F0B => (ElementIdentifier::IndexEditRate, ValueDataType::Rational),
      0x3F0C => (ElementIdentifier::IndexStartPosition, ValueDataType::Position),
      0x3F0D => (ElementIdentifier::IndexDuration, ValueDataType::Length),
      0x3F0E => (ElementIdentifier::PositionTableCount, ValueDataType::Uint8),

      0x4001 => (ElementIdentifier::UrlString, ValueDataType::Utf16),
      0x4101 => (ElementIdentifier::LocatorName, ValueDataType::Utf16),
      0x4401 => (ElementIdentifier::PackageUid, ValueDataType::Umid),
      0x4402 => (ElementIdentifier::Name, ValueDataType::Utf16),
      0x4403 => (ElementIdentifier::Tracks, ValueDataType::StrongRefArray),
      0x4404 => (ElementIdentifier::PackageModifiedDate, ValueDataType::Timestamp),
      0x4405 => (ElementIdentifier::PackageCreationDate, ValueDataType::Timestamp),
      0x4701 => (ElementIdentifier::Descriptor, ValueDataType::StrongRef),
      0x4801 => (ElementIdentifier::TrackId, ValueDataType::Uint32),
      0x4802 => (ElementIdentifier::TrackName, ValueDataType::String),
      0x4804 => (ElementIdentifier::TrackNumber, ValueDataType::Uint32),
      0x4B01 => (ElementIdentifier::EditRate, ValueDataType::Rational),
      0x4B02 => (ElementIdentifier::Origin, ValueDataType::Position),
      0x4803 => (ElementIdentifier::Sequence, ValueDataType::StrongRef),
      0x4901 => (ElementIdentifier::EventEditRate, ValueDataType::Rational),
      0x4902 => (ElementIdentifier::EventOrigin, ValueDataType::Position),
      0x4F01 => (ElementIdentifier::ChunkId, ValueDataType::Uint32),
      0x4F02 => (ElementIdentifier::ChunkLength, ValueDataType::Uint32),
      0x4F03 => (ElementIdentifier::ChunkData, ValueDataType::Stream),
      0x6101 => (ElementIdentifier::DmFramework, ValueDataType::StrongRef),
      0x6102 => (ElementIdentifier::TrackIds, ValueDataType::TrackIdBatch),
      _      => {
        for dynamic_tag in dynamic_tags {
          // println!("{:?}", dynamic_tag);
          if dynamic_tag.tag == id {
            match dynamic_tag.identifier {
              Ul::IsRipPresent => return Some((ElementIdentifier::IsRipPresent, ValueDataType::Boolean)),
              Ul::SubDescriptors => return Some((ElementIdentifier::SubDescriptors, ValueDataType::StrongRefArray)),

              Ul::MCALabelDictionaryID => return Some((ElementIdentifier::MCALabelDictionaryID, ValueDataType::Ul)),
              Ul::MCATagSymbol => return Some((ElementIdentifier::MCATagSymbol, ValueDataType::Utf16)),
              Ul::MCATagName => return Some((ElementIdentifier::MCATagName, ValueDataType::Utf16)),
              // Ul::GroupOfSoundfieldGroupsLinkID => return Some((ElementIdentifier::GroupOfSoundfieldGroupsLinkID, ValueDataType::UuidArray)),
              Ul::MCALinkID => return Some((ElementIdentifier::MCALinkID, ValueDataType::Uuid)),
              Ul::SoundfieldGroupLinkID => return Some((ElementIdentifier::SoundfieldGroupLinkID, ValueDataType::Uuid)),
              Ul::MCAChannelID => return Some((ElementIdentifier::MCAChannelID, ValueDataType::Uint32)),
              Ul::MCATitle => return Some((ElementIdentifier::MCATitle, ValueDataType::Utf16)),
              Ul::MCATitleVersion => return Some((ElementIdentifier::MCATitleVersion, ValueDataType::Utf16)),
              Ul::MCATitleSubVersion => return Some((ElementIdentifier::MCATitleSubVersion, ValueDataType::Utf16)),
              Ul::MCAEpisode => return Some((ElementIdentifier::MCAEpisode, ValueDataType::Utf16)),
              Ul::MCAPartitionKind => return Some((ElementIdentifier::MCAPartitionKind, ValueDataType::Utf16)),
              Ul::MCAPartitionNumber => return Some((ElementIdentifier::MCAPartitionNumber, ValueDataType::Utf16)),
              Ul::MCAAudioContentKind => return Some((ElementIdentifier::MCAAudioContentKind, ValueDataType::Utf16)),
              Ul::MCAAudioElementKind => return Some((ElementIdentifier::MCAAudioElementKind, ValueDataType::Utf16)),
              Ul::RFC5646SpokenLanguage => return Some((ElementIdentifier::RFC5646SpokenLanguage, ValueDataType::String)),

              Ul::Mpeg2VideoDescriptorSingleSequence => return Some((ElementIdentifier::Mpeg2VideoDescriptorSingleSequence, ValueDataType::Boolean)),
              Ul::Mpeg2VideoDescriptorCodedContentType => return Some((ElementIdentifier::Mpeg2VideoDescriptorCodedContentType, ValueDataType::Uint8)),
              Ul::Mpeg2VideoDescriptorBPictureCount => return Some((ElementIdentifier::Mpeg2VideoDescriptorBPictureCount, ValueDataType::Uint16)),
              Ul::Mpeg2VideoDescriptorProfileAndLevel => return Some((ElementIdentifier::Mpeg2VideoDescriptorProfileAndLevel, ValueDataType::Uint8)),
              Ul::Mpeg2VideoDescriptorLowDelay => return Some((ElementIdentifier::Mpeg2VideoDescriptorLowDelay, ValueDataType::Boolean)),
              Ul::Mpeg2VideoDescriptorMaxGOP => return Some((ElementIdentifier::Mpeg2VideoDescriptorMaxGOP, ValueDataType::Uint16)),
              Ul::Mpeg2VideoDescriptorConstantBframes => return Some((ElementIdentifier::Mpeg2VideoDescriptorConstantBframes, ValueDataType::Boolean)),
              Ul::Mpeg2VideoDescriptorClosedGOP => return Some((ElementIdentifier::Mpeg2VideoDescriptorClosedGOP, ValueDataType::Boolean)),
              Ul::Mpeg2VideoDescriptorIdenticalGOP => return Some((ElementIdentifier::Mpeg2VideoDescriptorIdenticalGOP, ValueDataType::Boolean)),
              Ul::Mpeg2VideoDescriptorBitRate => return Some((ElementIdentifier::Mpeg2VideoDescriptorBitRate, ValueDataType::Uint32)),

              Ul::Jpeg2000VideoDescriptor_Rsiz => return Some((ElementIdentifier::Jpeg2000VideoDescriptor_Rsiz, ValueDataType::Uint16)),
              Ul::Jpeg2000VideoDescriptor_Xsiz => return Some((ElementIdentifier::Jpeg2000VideoDescriptor_Xsiz, ValueDataType::Uint32)),
              Ul::Jpeg2000VideoDescriptor_Ysiz => return Some((ElementIdentifier::Jpeg2000VideoDescriptor_Ysiz, ValueDataType::Uint32)),
              Ul::Jpeg2000VideoDescriptor_XOsiz => return Some((ElementIdentifier::Jpeg2000VideoDescriptor_XOsiz, ValueDataType::Uint32)),
              Ul::Jpeg2000VideoDescriptor_YOsiz => return Some((ElementIdentifier::Jpeg2000VideoDescriptor_YOsiz, ValueDataType::Uint32)),
              Ul::Jpeg2000VideoDescriptor_XTsiz => return Some((ElementIdentifier::Jpeg2000VideoDescriptor_XTsiz, ValueDataType::Uint32)),
              Ul::Jpeg2000VideoDescriptor_YTsiz => return Some((ElementIdentifier::Jpeg2000VideoDescriptor_YTsiz, ValueDataType::Uint32)),
              Ul::Jpeg2000VideoDescriptor_XTOsiz => return Some((ElementIdentifier::Jpeg2000VideoDescriptor_XTOsiz, ValueDataType::Uint32)),
              Ul::Jpeg2000VideoDescriptor_YTOsiz => return Some((ElementIdentifier::Jpeg2000VideoDescriptor_YTOsiz, ValueDataType::Uint32)),
              Ul::Jpeg2000VideoDescriptor_Csiz => return Some((ElementIdentifier::Jpeg2000VideoDescriptor_Csiz, ValueDataType::Uint16)),
              Ul::Jpeg2000VideoDescriptor_PictureComponentSizing => return Some((ElementIdentifier::Jpeg2000VideoDescriptor_PictureComponentSizing, ValueDataType::J2KComponentSizing)),
              // Ul::Jpeg2000VideoDescriptor_CodingStyleDefault => return Some((ElementIdentifier::Jpeg2000VideoDescriptor_CodingStyleDefault, ValueDataType::J2KComponentSizing)),
              // Ul::Jpeg2000VideoDescriptor_QuantizationDefault => return Some((ElementIdentifier::Jpeg2000VideoDescriptor_QuantizationDefault, ValueDataType::J2KComponentSizing)),
              // Ul::Jpeg2000VideoDescriptor_J2CLayout => return Some((ElementIdentifier::Jpeg2000VideoDescriptor_J2CLayout, ValueDataType::J2KComponentSizing)),
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
