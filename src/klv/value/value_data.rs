
use klv::ul::Ul;
use timecode::timecode::Timecode;

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
  SystemItemSystem {
    channel_handle: u16,
    continuity_count: u16,
    essence_ul: Ul,
    creation_timestamp: Option<Timecode>,
    user_timestamp: Option<Timecode>,
  },
  Unknown {
    data: Vec<u8>
  }
}
