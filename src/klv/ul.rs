
#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
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

  PictureEssenceTrack,
  SoundEssenceTrack,
  DataEssenceTrack,
  DescriptiveMetadataTrack,
  SMPTE12MTimecodeTrackInactiveUserBits,
  SMPTE12MTimecodeTrackActiveUserBits,
  PictureItemMpegFrameWrappedPictureElement,
  Jpeg2000FrameWrapped,
  Jpeg2000ClipWrapped,
  SoundItemDataWrappedSoundElement,
  SoundItemWaveDataWrappedSoundElement,
  SoundItemBwfDataWrappedSoundElement,

  SystemItemElement,
  SystemItemSystemMetadataPack,
  SystemItemPackageMetadataSet,

  FillItem,
  FillItemAvid,

  StructuralComponentDataDefinition,
  StructuralComponentDuration,
  SequenceStructuralComponents,
  SourceClipSourcePackageID,
  SourceClipSourceTrackID,
  SourceClipStartPosition,
  TimecodeComponentStartTimecode,
  TimecodeComponentRoundedTimecodeBase,
  TimecodeComponentDropFrame,
  ContentStoragePackages,
  ContentStorageEssenceContainerData,
  EssenceContainerDataLinkedPackageUID,

  IndexTableSegment,
  IndexTableSegment_IndexEditRate,
  IndexTableSegment_IndexStartPosition,
  IndexTableSegment_IndexDuration,
  IndexTableSegment_EditUnitByteCount,
  IndexTableSegment_IndexSID,
  IndexTableSegment_BodySID,
  IndexTableSegment_SliceCount,
  IndexTableSegment_PositionTableCount,
  IndexTableSegment_DeltaEntryArray,
  IndexTableSegment_IndexEntryArray,

  IndexTableLayout_SingleIndexLocation,
  IndexTableLayout_SingleEssenceLocation,
  IndexTableLayout_ForwardIndexDirection,

  FileDescriptor_SampleRate,
  FileDescriptor_ContainerDuration,
  FileDescriptor_EssenceContainer,
  FileDescriptor_LinkedTrackID,
  FileDescriptor_Codec,
  FileDescriptor_Locators,

  NetworkLocator_Url,

  GenericPictureEssenceDescriptor_SignalStandard,
  GenericPictureEssenceDescriptor_FrameLayout,
  GenericPictureEssenceDescriptor_StoredWidth,
  GenericPictureEssenceDescriptor_StoredHeight,
  GenericPictureEssenceDescriptor_StoredF2Offset,
  GenericPictureEssenceDescriptor_SampledWidth,
  GenericPictureEssenceDescriptor_SampledHeight,
  GenericPictureEssenceDescriptor_SampledXOffset,
  GenericPictureEssenceDescriptor_SampledYOffset,
  GenericPictureEssenceDescriptor_DisplayHeight,
  GenericPictureEssenceDescriptor_DisplayWidth,
  GenericPictureEssenceDescriptor_DisplayXOffset,
  GenericPictureEssenceDescriptor_DisplayYOffset,
  GenericPictureEssenceDescriptor_DisplayF2Offset,
  GenericPictureEssenceDescriptor_AspectRatio,
  GenericPictureEssenceDescriptor_ActiveFormatDescriptor,
  GenericPictureEssenceDescriptor_VideoLineMap,
  GenericPictureEssenceDescriptor_AlphaTransparency,
  GenericPictureEssenceDescriptor_TransferCharacteristic,
  GenericPictureEssenceDescriptor_ImageAlignmentOffset,
  GenericPictureEssenceDescriptor_ImageStartOffset,
  GenericPictureEssenceDescriptor_ImageEndOffset,
  GenericPictureEssenceDescriptor_FieldDominance,
  GenericPictureEssenceDescriptor_PictureEssenceCoding,
  CodingEquations,
  GenericPictureEssenceDescriptor_ActiveHeight,
  GenericPictureEssenceDescriptor_ActiveWidth,
  GenericPictureEssenceDescriptor_ActiveXOffset,
  GenericPictureEssenceDescriptor_ActiveYOffset,

  CdciEssenceDescriptor_ComponentDepth,
  CdciEssenceDescriptor_HorizontalSubsampling,
  CdciEssenceDescriptor_VerticalSubsampling,
  CdciEssenceDescriptor_ColorSiting,
  CdciEssenceDescriptor_ReversedByteOrder,
  CdciEssenceDescriptor_PaddingBits,
  CdciEssenceDescriptor_AlphaSampleDepth,
  CdciEssenceDescriptor_BlackRefLevel,
  CdciEssenceDescriptor_WhiteReflevel,
  CdciEssenceDescriptor_ColorRange,
  CdciEssenceDescriptor_ComponentMaxRef,
  CdciEssenceDescriptor_ComponentMinRef,
  CdciEssenceDescriptor_AlphaMaxRef,
  CdciEssenceDescriptor_AlphaMinRef,
  CdciEssenceDescriptor_ScanningDirection,
  CdciEssenceDescriptor_PixelLayout,
  CdciEssenceDescriptor_Palette,
  CdciEssenceDescriptor_PaletteLayout,

  Preface_LastModifiedDate,
  Preface_ContentStorage,
  Preface_Version,
  Preface_Identifications,
  Preface_DMSchemes,

  Preface_ObjectModelVersion,
  Preface_PrimaryPackage,

  LinkedGenerationID,

  OperationalPattern,
  EssenceContainers,
  IsRipPresent,
  Identification_CompanyName,
  Identification_ProductName,
  Identification_ProductVersion,
  Identification_VersionString,
  Identification_ProductUid,
  Identification_ModificationDate,
  Identification_ToolkitVersion,
  Identification_Platform,
  Identification_ThisGenerationUid,
  InterchangeObjectInstanceUid,

  GenericSoundEssenceDescriptorQuantizationBits,
  GenericSoundEssenceDescriptorLocked,
  GenericSoundEssenceDescriptorAudioSamplingRate,
  GenericSoundEssenceDescriptorAudioRefLevel,
  GenericSoundEssenceDescriptorChannelCount,

  WaveEssenceDescriptor_BlockAlign,
  WaveEssenceDescriptor_SequenceOffset,
  WaveEssenceDescriptor_AvgBps,
  WaveEssenceDescriptor_ChannelAssignment,
  WaveEssenceDescriptor_PeakEnvelopeVersion,
  WaveEssenceDescriptor_PeakEnvelopeFormat,
  WaveEssenceDescriptor_PointsPerPeakValue,
  WaveEssenceDescriptor_PeakEnvelopeBlockSize,
  WaveEssenceDescriptor_PeakChannels,
  WaveEssenceDescriptor_PeakFrames,
  WaveEssenceDescriptor_PeakOfPeaksPosition,
  WaveEssenceDescriptor_PeakEnvelopeTimestamp,
  WaveEssenceDescriptor_PeakEnvelopeData,

  Aes3AudioDescriptorAuxBitsMode,
  Aes3AudioDescriptorBlockStartOffset,
  Aes3AudioDescriptorChannelStatusMode,
  Aes3AudioDescriptorFixedChannelStatusData,
  Aes3AudioDescriptorUserDataMode,
  Aes3AudioDescriptorFixedUserData,
  Aes3AudioDescriptorEmphasis,

  MultipleDescriptorSubDescriptorUids,
  IndexSid,
  BodySid,
  GenericPackagePackageUid,
  GenericPackageTracks,
  GenericPackagePackageModifiedDate,
  GenericPackagePackageCreationDate,
  SourcePackageDescriptor,
  GenericTrackTrackId,
  GenericTrackTrackName,
  GenericTrackSequence,
  GenericTrackTrackNumber,

  ColorPrimaries,
  ElectrospatialFormulation,
  SoundCompression,
  DialNorm,
  ChannelAssignment,
  RFC5646SpokenLanguage,

  Mpeg2VideoDescriptorSingleSequence,
  Mpeg2VideoDescriptorConstantBframes,
  Mpeg2VideoDescriptorCodedContentType,
  Mpeg2VideoDescriptorLowDelay,
  Mpeg2VideoDescriptorClosedGOP,
  Mpeg2VideoDescriptorIdenticalGOP,
  Mpeg2VideoDescriptorMaxGOP,
  Mpeg2VideoDescriptorBPictureCount,
  Mpeg2VideoDescriptorProfileAndLevel,
  Mpeg2VideoDescriptorBitRate,

  IsoIec154441Jpeg2002,
  JPEG2000DigitalCinemaProfile,
  JPEG2000Amd12KDigitalCinemaProfile,
  JPEG2000Amd14KDigitalCinemaProfile,
  JPEG2000BroadcastContributionSingleTileProfileLevel1,
  JPEG2000BroadcastContributionSingleTileProfileLevel2,
  JPEG2000BroadcastContributionSingleTileProfileLevel3,
  JPEG2000BroadcastContributionSingleTileProfileLevel4,
  JPEG2000BroadcastContributionSingleTileProfileLevel5,
  JPEG2000BroadcastContributionMultiTileReversibleProfileLevel6,
  JPEG2000UndefinedDigitalCinemaProfile,

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

  TrackEditRate,
  TrackOrigin,
  DMSegmentDMFramework,
  SubDescriptors,

  AS10CoreFramework,
  AS10ShimName,
  AS10Type,
  AS10MainTitle,
  AS10SubTitle,
  AS10TitleDescription,
  AS10OrganizationName,
  AS10PersonName,
  AS10LocationDescription,
  AS10CommonSpanningID,
  AS10SpanningNumber,
  AS10CumulativeDuration,
  
  SourceDescriptor_ChannelIDs,
  SourceDescriptor_MonoSourceTrackIDs,

  PackageName,

  Eidr_CanonicalDOIName,
  Eidr_CanonicalEIDRIdentifier,
  Eidr_CompactAdIDIdentifier,
  Eidr_DMSEssenceID,

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

  ReferenceImageEditRate,
  ReferenceAudioAlignmentLevel,

  Essence_BwfFrameWrapped,
  Essence_BwfClipWrapped,
  Essence_AesFrameWrapped,
  Essence_AesClipWrapped,
  Essence_MpegEsWithStreamIdFrameWrapped,
  Essence_MpegEsWithStreamIdClipWrapped,
  Essence_MpegEsWithStreamIdStripeWrapped,
  Essence_MpegEsWithStreamIdFixedAudioSizeWrapped,
  Essence_MpegEsWithStreamIdSpliceWrapped,
  Essence_MpegEsWithStreamIdClosedGopWrapped,
  Essence_MpegEsWithStreamIdSlaveWrapped,
  Essence_MpegEsWithStreamIdNoSpecificWrappingConstraints,
  Essence_AVCByteStream_FrameWrapped,
  Essence_AVCByteStream_ClipedWrapped,
  Essence_AVCByteStream_StripedWrapped,
  Essence_AVCByteStream_PesWrapped,
  Essence_AVCByteStream_FixedAudioSizeWrapped,
  Essence_AVCByteStream_SpliceWrapped,
  Essence_AVCByteStream_ClosedGopWrapped,
  Essence_AVCByteStream_SlaveWrapped,
  Essence_Jpeg2000_FrameWrapped,
  Essence_Jpeg2000_ClipedWrapped,
  Essence_MXFGCP1FrameWrappedPictureElement,
  Essence_GenericEssenceContainerMultipleWrappings,
  Essence_ANCData,

  PartitionMajor,
  PartitionMinor,
  PartitionKagSize,
  PartitionThisPartition,
  PartitionPreviousPartition,
  PartitionFooterPartition,
  PartitionHeaderByteCount,
  PartitionIndexByteCount,
  PartitionIndexSid,
  PartitionByteOffset,
  PartitionBodySid,
  PartitionOperationalPattern,
  PartitionEssenceContainers,

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
  UnknownBWFChunck,
  DataEssenceCoding,
  SubDescriptorUids,

  EditUnitByteCount,
  SliceCount,
  DeltaEntryArray,
  IndexEntryArray,
  IndexEditRate,
  IndexStartPosition,
  IndexDuration,
  PositionTableCount,

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
  TrackIds,

  MxfOP1aSingleItemSinglePackageUniTrackStreamInternal,
  MxfOP1aSingleItemSinglePackageUniTrackStreamExternal,
  MxfOP1aSingleItemSinglePackageUniTrackNonStreamInternal,
  MxfOP1aSingleItemSinglePackageUniTrackNonStreamExternal,
  MxfOP1aSingleItemSinglePackageMultiTrackStreamInternal,
  MxfOP1aSingleItemSinglePackageMultiTrackStreamExternal,
  MxfOP1aSingleItemSinglePackageMultiTrackNonStreamInternal,
  MxfOP1aSingleItemSinglePackageMultiTrackNonStreamExternal,
  MxfOP1bSingleItemGangedPackagesUniTrackStreamInternal,
  MxfOP1bSingleItemGangedPackagesUniTrackStreamExternal,
  MxfOP1bSingleItemGangedPackagesUniTrackNonStreamInternal,
  MxfOP1bSingleItemGangedPackagesUniTrackNonStreamExternal,
  MxfOP1bSingleItemGangedPackagesMultiTrackStreamInternal,
  MxfOP1bSingleItemGangedPackagesMultiTrackStreamExternal,
  MxfOP1bSingleItemGangedPackagesMultiTrackNonStreamInternal,
  MxfOP1bSingleItemGangedPackagesMultiTrackNonStreamExternal,

  CodingEquations_ITU601,
  CodingEquations_ITU709,
  CodingEquations_SMPTE240M,

  ColorPrimaries_SMPTE170M,
  ColorPrimaries_ITU470_PAL,
  ColorPrimaries_ITU709,
  ColorPrimaries_ITU2020,

  TransferCharacteristic_ITU470_PAL,
  TransferCharacteristic_ITU709,
  TransferCharacteristic_SMPTE240M,
  TransferCharacteristic_274M_296M,
  TransferCharacteristic_ITU1361,
  TransferCharacteristic_linear,
  TransferCharacteristic_SMPTE_DCDM,
  TransferCharacteristic_IEC6196624_xvYCC,
  TransferCharacteristic_ITU2020,

  Mpeg2_422_PHL_IFrame,
  Mpeg2_422_PHL_LongGOP,
  Mpeg2_422_PHL_NoIFrames,
  Mpeg2_MPH14_IFrame,
  Mpeg2_MPH14_LongGOP,
  Mpeg2_MPH14_NoIFrames,

  H264_Mpeg4_AVC_High422Intra_RP2027ConstrainedClass100_1080_5994iCoding,
  H264_Mpeg4_AVC_High422Intra_RP2027ConstrainedClass100_1080_50iCoding,
  H264_Mpeg4_AVC_High422Intra_RP2027ConstrainedClass100_1080_2997pCoding,
  H264_Mpeg4_AVC_High422Intra_RP2027ConstrainedClass100_1080_25pCoding,

  SMPTE382MDefaultUncompressedSoundCoding,
  AIFFUncompressedCoding,
  UndefinedSoundCoding,

  LeftAudioChannel,
  RightAudioChannel,
  CenterAudioChannel,
  LFEAudioChannel,
  LeftSurroundAudioChannel,
  RightSurroundAudioChannel,
  LeftSideSurroundAudioChannel,
  RightSideSurroundAudioChannel,
  LeftRearSurroundAudioChannel,
  RightRearSurroundAudioChannel,
  LeftCenterAudioChannel,
  RightCenterAudioChannel,
  CenterSurroundAudioChannel,
  HearingImpairedAudioChannel,
  VisuallyImpairedNarrativeAudioChannel,
  Smpte_ST_20678_MonoOne,
  Smpte_ST_20678_MonoTwo,
  Smpte_ST_20678_LeftTotal,
  Smpte_ST_20678_RightTotal,
  Smpte_ST_20678_LeftSurroundTotal,
  Smpte_ST_20678_RightSurroundTotal,
  Smpte_ST_20678_Surround,
  Smpte_ST_20678_NumberedSourceChannel_001,
  Smpte_ST_20678_NumberedSourceChannel_002,
  Smpte_ST_20678_NumberedSourceChannel_003,
  Smpte_ST_20678_NumberedSourceChannel_004,
  Smpte_ST_20678_NumberedSourceChannel_005,
  Smpte_ST_20678_NumberedSourceChannel_006,
  Smpte_ST_20678_NumberedSourceChannel_007,
  Smpte_ST_20678_NumberedSourceChannel_008,
  Smpte_ST_20678_NumberedSourceChannel_009,
  Smpte_ST_20678_NumberedSourceChannel_00A,
  Smpte_ST_20678_NumberedSourceChannel_00B,
  Smpte_ST_20678_NumberedSourceChannel_00C,
  Smpte_ST_20678_NumberedSourceChannel_00D,
  Smpte_ST_20678_NumberedSourceChannel_00E,
  Smpte_ST_20678_NumberedSourceChannel_00F,

  Smpte_ST_20678_StandardStereo,
  Smpte_ST_20678_DualMono,
  Smpte_ST_20678_DiscreteNumberedSources,
  Smpte_ST_20678_30,
  Smpte_ST_20678_40,
  Smpte_ST_20678_50,
  Smpte_ST_20678_60,
  Smpte_ST_20678_70DS,
  Smpte_ST_20678_LtRt,
  Smpte_ST_20678_51EX,
  Smpte_ST_20678_HearingAccessibility,
  Smpte_ST_20678_VisualAccessibility,

  Smpte_ST_20672_ApplicationOfTheMXFMultichannelAudioFramework,

  SoundfieldGroup_51,
  SoundfieldGroup_71DS,
  SoundfieldGroup_71SDS,
  SoundfieldGroup_61,
  SoundfieldGroup_Monaural,

  Unknown
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
  (Ul::PictureItemMpegFrameWrappedPictureElement) => (smpte_identifier!(SmpteRegitery::Essence, 0x15, 0x05));
  (Ul::Jpeg2000FrameWrapped) => (smpte_identifier!(SmpteRegitery::Essence, 0x15, 0x08));
  (Ul::Jpeg2000ClipWrapped) => (smpte_identifier!(SmpteRegitery::Essence, 0x15, 0x09));
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

macro_rules! build_identifier {
  (version_number => $vn:expr, kind => $k:expr) => (
    smpte_identifier!(0x04, 0x01, 0x01, $vn, 0x0d, 0x01, 0x03, 0x01, 0x02, 0x06,   $k, 0x00)
  );
  (version_number => $vn:expr, mpeg_es => $me:expr) => (
    smpte_identifier!(0x04, 0x01, 0x01, $vn, 0x0d, 0x01, 0x03, 0x01, 0x02, 0x04,    _,  $me)
  );
  (version_number => $vn:expr, avc => $k:expr) => (
    smpte_identifier!(0x04, 0x01, 0x01, $vn, 0x0d, 0x01, 0x03, 0x01, 0x02, 0x10, 0x60,   $k)
  );
  (version_number => $vn:expr, jpeg2000 => $k:expr) => (
    smpte_identifier!(0x04, 0x01, 0x01, $vn, 0x0d, 0x01, 0x03, 0x01, 0x02, 0x0c,   $k, 0x00)
  );
  (version_number => $vn:expr, mpeg2_profil => $k:expr, mode => $m:expr) => (
    smpte_identifier!(0x04, 0x01, 0x01, $vn, 0x04, 0x01, 0x02, 0x02, 0x01,   $k,   $m, 0x00)
  );
  (version_number => $vn:expr, mpeg4_avc => $k:expr) => (
    smpte_identifier!(0x04, 0x01, 0x01, $vn, 0x04, 0x01, 0x02, 0x02, 0x01, 0x32, 0x31,   $k)
  );
  (version_number => $vn:expr, transfer_carateristic => $k:expr) => (
    smpte_identifier!(0x04, 0x01, 0x01, $vn, 0x04, 0x01, 0x01, 0x01, 0x01,   $k, 0x00, 0x00)
  );
  (version_number => $vn:expr, coding_equations => $k:expr) => (
    smpte_identifier!(0x04, 0x01, 0x01, $vn, 0x04, 0x01, 0x01, 0x01, 0x02,   $k, 0x00, 0x00)
  );
  (version_number => $vn:expr, color_primaries => $k:expr) => (
    smpte_identifier!(0x04, 0x01, 0x01, $vn, 0x04, 0x01, 0x01, 0x01, 0x03,   $k, 0x00, 0x00)
  );
  (version_number => $vn:expr, uncompressed_sound_coding => $k:expr) => (
    smpte_identifier!(0x04, 0x01, 0x01, $vn, 0x04, 0x02, 0x02, 0x01,   $k, 0x00, 0x00, 0x00)
  );
  (version_number => $vn:expr, audio_channel => $k:expr) => (
    smpte_identifier!(0x04, 0x01, 0x01, $vn, 0x03, 0x02, 0x01,   $k, 0x00, 0x00, 0x00, 0x00)
  );
  (version_number => $vn:expr, audio_st20678 => $k:expr) => (
    smpte_identifier!(0x04, 0x01, 0x01, $vn, 0x03, 0x02, 0x01, 0x20,   $k, 0x00, 0x00, 0x00)
  );
  (version_number => $vn:expr, audio_st20678 => $k:expr, index => $i:expr) => (
    smpte_identifier!(0x04, 0x01, 0x01, $vn, 0x03, 0x02, 0x01, 0x20,   $k,   $i, 0x00, 0x00)
  );
  (version_number => $vn:expr, soundfield => $k:expr) => (
    smpte_identifier!(0x04, 0x01, 0x01, $vn, 0x03, 0x02, 0x02,   $k, 0x00, 0x00, 0x00, 0x00)
  );
  (version_number => $vn:expr, soundfield_st20678 => $k:expr) => (
    smpte_identifier!(0x04, 0x01, 0x01, $vn, 0x03, 0x02, 0x02, 0x20,   $k, 0x00, 0x00, 0x00)
  );
  (version_number => $vn:expr, gc => $k:expr) => (
    smpte_identifier!(0x04, 0x01, 0x01, $vn, 0x0d, 0x01, 0x03, 0x01, 0x02, 0x0c, 0x06, 0x00)
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
    smpte_identifier!(0x01, 0x02, 0x01, 0x01, 0x0d, 0x01, 0x03, 0x01, $x, 0x01, $y, $stream)
  );
  (SmpteRegitery::Essence, $x:tt, $y:tt) => (
    smpte_identifier!(0x01, 0x02, 0x01, 0x01, 0x0d, 0x01, 0x03, 0x01, $x, 0x01, $y, _)
  );
  (SmpteRegitery::Fill, $x:tt) => (
    smpte_identifier!(0x01, 0x01, 0x01, $x, 0x03, 0x01, 0x02, 0x10, 0x01, 0x00, 0x00, 0x00)
  );

  ($x0:tt, $x1:tt, $x2:tt, $x3:tt, $x4:tt, $x5:tt, $x6:tt, $x7:tt, $x8:tt, $x9:tt, $x10:tt, $x11:tt) => (
    (0x06, 0x0e, 0x2b, 0x34, $x0, $x1, $x2, $x3, $x4, $x5, $x6, $x7, $x8, $x9, $x10, $x11)
  );
}


pub fn match_ul(data: Vec<u8>) -> Option<Ul> {
  get_ul(data)
}


fn get_set_kind(v: u8) -> Ul {
  match v {
    0x0f => Ul::SequenceSet,
    0x11 => Ul::SourceClipSet,
    0x14 => Ul::TimecodeComponentSet,
    0x18 => Ul::ContentStorageSet,
    0x23 => Ul::EssenceContainerDataSet,
    0x25 => Ul::FileDescriptorSet,
    0x27 => Ul::GenericPictureEssenceDescriptor,
    0x28 => Ul::CdciVideoDescriptor,
    0x29 => Ul::RgbaVideoDescriptor,
    0x2f => Ul::PrefaceSet,
    0x30 => Ul::IdentificationSet,
    0x32 => Ul::NetworkLocatorSet,
    0x33 => Ul::TextLocatorSet,
    0x36 => Ul::MaterialPackageSet,
    0x37 => Ul::FilePackageSet,
    0x3a => Ul::StaticTrackSet,
    0x3b => Ul::TrackSet,
    0x39 => Ul::EventTrackSet,
    0x41 => Ul::DmSegmentDescriptorSet,
    0x42 => Ul::GenericSoundEssenceDescriptorSet,
    0x43 => Ul::GenericDataEssenceDescriptorSet,
    0x44 => Ul::MultipleDescriptorSet,
    0x45 => Ul::DmSourceClipSet,
    0x47 => Ul::Aes3AudioDescriptorSet,
    0x48 => Ul::WaveAudioDescriptorSet,
    0x51 => Ul::MpegVideoDescriptorSet,
    0x5a => Ul::Jpeg2000SubDescriptorSet,
    0x6a => Ul::McaLabelSubDescriptorSet,
    0x6b => Ul::AudioChannelLabelSubDescriptorSet,
    0x6c => Ul::SoundfieldGroupLabelSubDescriptorSet,
       _ => Ul::Unknown,
  }
}

fn get_ul(data: Vec<u8>) -> Option<Ul> {
  let ul =
    match (data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8], data[9], data[10], data[11], data[12], data[13], data[14], data[15]) {
      ul_filter!(Ul::HeaderPartition) => {
        Ul::HeaderPartition
      },
      ul_filter!(Ul::BodyPartition) => {
        Ul::BodyPartition
      },
      ul_filter!(Ul::FooterPartition) => {
        Ul::FooterPartition
      },
      ul_filter!(Ul::PrimerPack) => {
        Ul::PrimerPack
      },
      ul_filter!(Ul::RandomIndexMetadata) => {
        Ul::RandomIndexMetadata
      },
      ul_filter!(Ul::FillItem) => {
        Ul::FillItem
      },
      ul_filter!(Ul::FillItemAvid) => {
        Ul::FillItemAvid
      },
      ul_filter!(SmpteRegitery::Set) => {
        get_set_kind(data[14])
      },


      smpte_identifier!(0x02, 0x53, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, 0x01, 0x10, 0x01, 0x00) => {
        Ul::IndexTableSegment
      },
      smpte_identifier!(0x02, 0x53, 0x01, 0x01, 0x0d, 0x01, 0x03, 0x01, 0x14, 0x02, 0x01, 0x00) => {
        Ul::SystemItemElement
      },
      smpte_identifier!(0x02, 0x05, 0x01, 0x01, 0x0d, 0x01, 0x03, 0x01, 0x04, 0x01, 0x01, 0x00) => {
        Ul::SystemItemSystemMetadataPack
      },
      smpte_identifier!(0x02, 0x43, 0x01, 0x01, 0x0d, 0x01, 0x03, 0x01, 0x04, 0x01, 0x02, _track) => {
        Ul::SystemItemPackageMetadataSet
      },
      smpte_identifier!(0x04, 0x01, 0x01, 0x01, 0x01, 0x03, 0x02, 0x02, 0x01, 0x00, 0x0, 0x00) => {
        Ul::PictureEssenceTrack
      }
      smpte_identifier!(0x04, 0x01, 0x01, 0x01, 0x01, 0x03, 0x02, 0x02, 0x02, 0x00, 0x0, 0x00) => {
        Ul::SoundEssenceTrack
      }
      smpte_identifier!(0x04, 0x01, 0x01, 0x01, 0x01, 0x03, 0x02, 0x02, 0x03, 0x00, 0x0, 0x00) => {
        Ul::SoundEssenceTrack
      }
      smpte_identifier!(0x04, 0x01, 0x01, 0x01, 0x01, 0x03, 0x02, 0x01, 0x10, 0x00, 0x0, 0x00) => {
        Ul::DescriptiveMetadataTrack
      }
      smpte_identifier!(0x04, 0x01, 0x01, 0x01, 0x01, 0x03, 0x02, 0x01, 0x01, 0x00, 0x0, 0x00) => {
        Ul::SMPTE12MTimecodeTrackInactiveUserBits
      }
      smpte_identifier!(0x04, 0x01, 0x01, 0x01, 0x01, 0x03, 0x02, 0x01, 0x02, 0x00, 0x0, 0x00) => {
        Ul::SMPTE12MTimecodeTrackActiveUserBits
      }
      ul_filter!(Ul::PictureItemMpegFrameWrappedPictureElement) => {
        Ul::PictureItemMpegFrameWrappedPictureElement
      },
      ul_filter!(Ul::Jpeg2000FrameWrapped) => {
        Ul::Jpeg2000FrameWrapped
      },
      ul_filter!(Ul::Jpeg2000ClipWrapped) => {
        Ul::Jpeg2000ClipWrapped
      },
      smpte_identifier!(0x01, 0x02, 0x01, 0x01, 0x0d, 0x01, 0x03, 0x01, 0x16, 0x00, 0x00, _stream_index) => {
        Ul::SoundItemDataWrappedSoundElement
      },
      smpte_identifier!(0x01, 0x02, 0x01, 0x01, 0x0d, 0x01, 0x03, 0x01, 0x16,    _,    _, _stream_index) => {
        Ul::SoundItemWaveDataWrappedSoundElement
      },
      smpte_identifier!(0x01, 0x02, 0x01, 0x01, 0x0d, 0x01, 0x03, 0x01, 0x06, 0x00, 0x00, _stream_index) => {
        Ul::SoundItemBwfDataWrappedSoundElement
      },

      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x04, 0x07, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00) =>
        Ul::StructuralComponentDataDefinition,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x07, 0x02, 0x02, 0x01, 0x01, 0x03, 0x00, 0x00) =>
        Ul::StructuralComponentDuration,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x06, 0x01, 0x01, 0x04, 0x06, 0x09, 0x00, 0x00) =>
        Ul::SequenceStructuralComponents,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x06, 0x01, 0x01, 0x03, 0x01, 0x00, 0x00, 0x00) =>
        Ul::SourceClipSourcePackageID,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x06, 0x01, 0x01, 0x03, 0x02, 0x00, 0x00, 0x00) =>
        Ul::SourceClipSourceTrackID,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x07, 0x02, 0x01, 0x03, 0x01, 0x04, 0x00, 0x00) =>
        Ul::SourceClipStartPosition,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x07, 0x02, 0x01, 0x03, 0x01, 0x05, 0x00, 0x00) =>
        Ul::TimecodeComponentStartTimecode,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x04, 0x04, 0x01, 0x01, 0x02, 0x06, 0x00, 0x00) =>
        Ul::TimecodeComponentRoundedTimecodeBase,
      smpte_identifier!(0x01, 0x01, 0x01, 0x01, 0x04, 0x04, 0x01, 0x01, 0x05, 0x00, 0x00, 0x00) =>
        Ul::TimecodeComponentDropFrame,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x06, 0x01, 0x01, 0x04, 0x05, 0x01, 0x00, 0x00) =>
        Ul::ContentStoragePackages,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x06, 0x01, 0x01, 0x04, 0x05, 0x02, 0x00, 0x00) =>
        Ul::ContentStorageEssenceContainerData,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x06, 0x01, 0x01, 0x06, 0x01, 0x00, 0x00, 0x00) =>
        Ul::EssenceContainerDataLinkedPackageUID,

      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x05, 0x30, 0x04, 0x06, 0x00, 0x00, 0x00, 0x00) =>
        Ul::IndexTableSegment_IndexEditRate,
      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x07, 0x02, 0x01, 0x03, 0x01, 0x0a, 0x00, 0x00) =>
        Ul::IndexTableSegment_IndexStartPosition,
      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x07, 0x02, 0x02, 0x01, 0x01, 0x02, 0x00, 0x00) =>
        Ul::IndexTableSegment_IndexDuration,
      smpte_identifier!(0x01, 0x01, 0x01, 0x04, 0x04, 0x06, 0x02, 0x01, 0x00, 0x00, 0x00, 0x00) =>
        Ul::IndexTableSegment_EditUnitByteCount,
      smpte_identifier!(0x01, 0x01, 0x01, 0x04, 0x04, 0x04, 0x04, 0x01, 0x01, 0x00, 0x00, 0x00) =>
        Ul::IndexTableSegment_SliceCount,
      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x04, 0x04, 0x04, 0x01, 0x07, 0x00, 0x00, 0x00) =>
        Ul::IndexTableSegment_PositionTableCount,
      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x04, 0x04, 0x04, 0x01, 0x06, 0x00, 0x00, 0x00) =>
        Ul::IndexTableSegment_DeltaEntryArray,
      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x04, 0x04, 0x04, 0x02, 0x05, 0x00, 0x00, 0x00) =>
        Ul::IndexTableSegment_IndexEntryArray,

      smpte_identifier!(0x01, 0x01, 0x01, 0x0e, 0x04, 0x04, 0x05, 0x01, 0x00, 0x00, 0x00, 0x00) =>
        Ul::IndexTableLayout_SingleIndexLocation,
      smpte_identifier!(0x01, 0x01, 0x01, 0x0e, 0x04, 0x06, 0x02, 0x06, 0x00, 0x00, 0x00, 0x00) =>
        Ul::IndexTableLayout_SingleEssenceLocation,
      smpte_identifier!(0x01, 0x01, 0x01, 0x0e, 0x04, 0x04, 0x05, 0x02, 0x00, 0x00, 0x00, 0x00) =>
        Ul::IndexTableLayout_ForwardIndexDirection,

      smpte_identifier!(0x01, 0x01, 0x01, 0x01, 0x04, 0x06, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00) =>
        Ul::FileDescriptor_SampleRate,
      smpte_identifier!(0x01, 0x01, 0x01, 0x01, 0x04, 0x06, 0x01, 0x02, 0x00, 0x00, 0x00, 0x00) =>
        Ul::FileDescriptor_ContainerDuration,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x06, 0x01, 0x01, 0x04, 0x01, 0x02, 0x00, 0x00) =>
        Ul::FileDescriptor_EssenceContainer,
      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x06, 0x01, 0x01, 0x03, 0x05, 0x00, 0x00, 0x00) =>
        Ul::FileDescriptor_LinkedTrackID,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x06, 0x01, 0x01, 0x04, 0x01, 0x03, 0x00, 0x00) =>
        Ul::FileDescriptor_Codec,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x06, 0x01, 0x01, 0x04, 0x06, 0x03, 0x00, 0x00) =>
        Ul::FileDescriptor_Locators,

      smpte_identifier!(0x01, 0x01, 0x01, 0x01, 0x01, 0x02, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00) =>
        Ul::NetworkLocator_Url,

      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x04, 0x05, 0x01, 0x13, 0x00, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_SignalStandard,
      smpte_identifier!(0x01, 0x01, 0x01, 0x01, 0x04, 0x01, 0x03, 0x01, 0x04, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_FrameLayout,
      smpte_identifier!(0x01, 0x01, 0x01, 0x01, 0x04, 0x01, 0x05, 0x02, 0x02, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_StoredWidth,
      smpte_identifier!(0x01, 0x01, 0x01, 0x01, 0x04, 0x01, 0x05, 0x02, 0x01, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_StoredHeight,
      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x04, 0x01, 0x03, 0x02, 0x08, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_StoredF2Offset,
      smpte_identifier!(0x01, 0x01, 0x01, 0x01, 0x04, 0x01, 0x05, 0x01, 0x08, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_SampledWidth,
      smpte_identifier!(0x01, 0x01, 0x01, 0x01, 0x04, 0x01, 0x05, 0x01, 0x07, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_SampledHeight,
      smpte_identifier!(0x01, 0x01, 0x01, 0x01, 0x04, 0x01, 0x05, 0x01, 0x09, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_SampledXOffset,
      smpte_identifier!(0x01, 0x01, 0x01, 0x01, 0x04, 0x01, 0x05, 0x01, 0x0a, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_SampledYOffset,
      smpte_identifier!(0x01, 0x01, 0x01, 0x01, 0x04, 0x01, 0x05, 0x01, 0x0b, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_DisplayHeight,
      smpte_identifier!(0x01, 0x01, 0x01, 0x01, 0x04, 0x01, 0x05, 0x01, 0x0c, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_DisplayWidth,
      smpte_identifier!(0x01, 0x01, 0x01, 0x01, 0x04, 0x01, 0x05, 0x01, 0x0d, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_DisplayXOffset,
      smpte_identifier!(0x01, 0x01, 0x01, 0x01, 0x04, 0x01, 0x05, 0x01, 0x0e, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_DisplayYOffset,
      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x04, 0x01, 0x03, 0x02, 0x07, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_DisplayF2Offset,
      smpte_identifier!(0x01, 0x01, 0x01, 0x01, 0x04, 0x01, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_AspectRatio,
      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x04, 0x01, 0x03, 0x02, 0x09, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_ActiveFormatDescriptor,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x04, 0x01, 0x03, 0x02, 0x05, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_VideoLineMap,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x05, 0x20, 0x01, 0x02, 0x00, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_AlphaTransparency,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x04, 0x01, 0x02, 0x01, 0x01, 0x01, 0x02, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_TransferCharacteristic,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x04, 0x18, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_ImageAlignmentOffset,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x04, 0x18, 0x01, 0x02, 0x00, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_ImageStartOffset,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x04, 0x18, 0x01, 0x03, 0x00, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_ImageEndOffset,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x04, 0x01, 0x03, 0x01, 0x06, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_FieldDominance,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x04, 0x01, 0x06, 0x01, 0x00, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_PictureEssenceCoding,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x04, 0x01, 0x02, 0x01, 0x01, 0x03, 0x01, 0x00) =>
        Ul::CodingEquations,
      smpte_identifier!(0x01, 0x01, 0x01, 0x0e, 0x04, 0x01, 0x05, 0x01, 0x13, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_ActiveHeight,
      smpte_identifier!(0x01, 0x01, 0x01, 0x0e, 0x04, 0x01, 0x05, 0x01, 0x14, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_ActiveWidth,
      smpte_identifier!(0x01, 0x01, 0x01, 0x0e, 0x04, 0x01, 0x05, 0x01, 0x15, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_ActiveXOffset,
      smpte_identifier!(0x01, 0x01, 0x01, 0x0e, 0x04, 0x01, 0x05, 0x01, 0x16, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_ActiveYOffset,

      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x04, 0x01, 0x05, 0x03, 0x0a, 0x00, 0x00, 0x00) =>
        Ul::CdciEssenceDescriptor_ComponentDepth,
      smpte_identifier!(0x01, 0x01, 0x01, 0x01, 0x04, 0x01, 0x05, 0x01, 0x05, 0x00, 0x00, 0x00) =>
        Ul::CdciEssenceDescriptor_HorizontalSubsampling,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x04, 0x01, 0x05, 0x01, 0x10, 0x00, 0x00, 0x00) =>
        Ul::CdciEssenceDescriptor_VerticalSubsampling,
      smpte_identifier!(0x01, 0x01, 0x01, 0x01, 0x04, 0x01, 0x05, 0x01, 0x06, 0x00, 0x00, 0x00) =>
        Ul::CdciEssenceDescriptor_ColorSiting,
      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x03, 0x01, 0x02, 0x01, 0x0a, 0x00, 0x00, 0x00) =>
        Ul::CdciEssenceDescriptor_ReversedByteOrder,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x04, 0x18, 0x01, 0x04, 0x00, 0x00, 0x00, 0x00) =>
        Ul::CdciEssenceDescriptor_PaddingBits,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x04, 0x01, 0x05, 0x03, 0x07, 0x00, 0x00, 0x00) =>
        Ul::CdciEssenceDescriptor_AlphaSampleDepth,
      smpte_identifier!(0x01, 0x01, 0x01, 0x01, 0x04, 0x01, 0x05, 0x03, 0x03, 0x00, 0x00, 0x00) =>
        Ul::CdciEssenceDescriptor_BlackRefLevel,
      smpte_identifier!(0x01, 0x01, 0x01, 0x01, 0x04, 0x01, 0x05, 0x03, 0x04, 0x00, 0x00, 0x00) =>
        Ul::CdciEssenceDescriptor_WhiteReflevel,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x04, 0x01, 0x05, 0x03, 0x05, 0x00, 0x00, 0x00) =>
        Ul::CdciEssenceDescriptor_ColorRange,
      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x04, 0x01, 0x05, 0x03, 0x0b, 0x00, 0x00, 0x00) =>
        Ul::CdciEssenceDescriptor_ComponentMaxRef,
      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x04, 0x01, 0x05, 0x03, 0x0c, 0x00, 0x00, 0x00) =>
        Ul::CdciEssenceDescriptor_ComponentMinRef,
      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x04, 0x01, 0x05, 0x03, 0x0d, 0x00, 0x00, 0x00) =>
        Ul::CdciEssenceDescriptor_AlphaMaxRef,
      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x04, 0x01, 0x05, 0x03, 0x0e, 0x00, 0x00, 0x00) =>
        Ul::CdciEssenceDescriptor_AlphaMinRef,
      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x04, 0x01, 0x04, 0x04, 0x01, 0x00, 0x00, 0x00) =>
        Ul::CdciEssenceDescriptor_ScanningDirection,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x04, 0x01, 0x05, 0x03, 0x06, 0x00, 0x00, 0x00) =>
        Ul::CdciEssenceDescriptor_PixelLayout,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x04, 0x01, 0x05, 0x03, 0x08, 0x00, 0x00, 0x00) =>
        Ul::CdciEssenceDescriptor_Palette,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x04, 0x01, 0x05, 0x03, 0x09, 0x00, 0x00, 0x00) =>
        Ul::CdciEssenceDescriptor_PaletteLayout,

      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x07, 0x02, 0x01, 0x10, 0x02, 0x04, 0x00, 0x00) =>
        Ul::Preface_LastModifiedDate,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x06, 0x01, 0x01, 0x04, 0x02, 0x01, 0x00, 0x00) =>
        Ul::Preface_ContentStorage,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x03, 0x01, 0x02, 0x01, 0x05, 0x00, 0x00, 0x00) =>
        Ul::Preface_Version,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x06, 0x01, 0x01, 0x04, 0x06, 0x04, 0x00, 0x00) =>
        Ul::Preface_Identifications,
      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x01, 0x02, 0x02, 0x10, 0x02, 0x02, 0x00, 0x00) =>
        Ul::Preface_DMSchemes,

      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x03, 0x01, 0x02, 0x01, 0x04, 0x00, 0x00, 0x00) =>
        Ul::Preface_ObjectModelVersion,
      smpte_identifier!(0x01, 0x01, 0x01, 0x04, 0x06, 0x01, 0x01, 0x04, 0x01, 0x08, 0x00, 0x00) =>
        Ul::Preface_PrimaryPackage,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x05, 0x20, 0x07, 0x01, 0x08, 0x00, 0x00, 0x00) =>
        Ul::LinkedGenerationID,

      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x01, 0x02, 0x02, 0x03, 0x00, 0x00, 0x00, 0x00) =>
        Ul::OperationalPattern,
      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x01, 0x02, 0x02, 0x10, 0x02, 0x01, 0x00, 0x00) =>
        Ul::EssenceContainers,
      smpte_identifier!(0x01, 0x01, 0x01, 0x0e, 0x04, 0x04, 0x05, 0x03, 0x00, 0x00, 0x00, 0x00) =>
        Ul::IsRipPresent,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x05, 0x20, 0x07, 0x01, 0x02, 0x01, 0x00, 0x00) =>
        Ul::Identification_CompanyName,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x05, 0x20, 0x07, 0x01, 0x03, 0x01, 0x00, 0x00) =>
        Ul::Identification_ProductName,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x05, 0x20, 0x07, 0x01, 0x04, 0x00, 0x00, 0x00) =>
        Ul::Identification_ProductVersion,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x05, 0x20, 0x07, 0x01, 0x05, 0x01, 0x00, 0x00) =>
        Ul::Identification_VersionString,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x05, 0x20, 0x07, 0x01, 0x07, 0x00, 0x00, 0x00) =>
        Ul::Identification_ProductUid,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x07, 0x02, 0x01, 0x10, 0x02, 0x03, 0x00, 0x00) =>
        Ul::Identification_ModificationDate,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x05, 0x20, 0x07, 0x01, 0x0a, 0x00, 0x00, 0x00) =>
        Ul::Identification_ToolkitVersion,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x05, 0x20, 0x07, 0x01, 0x06, 0x01, 0x00, 0x00) =>
        Ul::Identification_Platform,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x05, 0x20, 0x07, 0x01, 0x01, 0x00, 0x00, 0x00) =>
        Ul::Identification_ThisGenerationUid,
      smpte_identifier!(0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x15, 0x02, 0x00, 0x00, 0x00, 0x00) =>
        Ul::InterchangeObjectInstanceUid,
      smpte_identifier!(0x01, 0x01, 0x01, 0x04, 0x04, 0x02, 0x03, 0x03, 0x04, 0x00, 0x00, 0x00) =>
        Ul::GenericSoundEssenceDescriptorQuantizationBits,
      smpte_identifier!(0x01, 0x01, 0x01, 0x04, 0x04, 0x02, 0x03, 0x01, 0x04, 0x00, 0x00, 0x00) =>
        Ul::GenericSoundEssenceDescriptorLocked,
      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x04, 0x02, 0x03, 0x01, 0x01, 0x01, 0x00, 0x00) =>
        Ul::GenericSoundEssenceDescriptorAudioSamplingRate,
      smpte_identifier!(0x01, 0x01, 0x01, 0x01, 0x04, 0x02, 0x01, 0x01, 0x03, 0x00, 0x00, 0x00) =>
        Ul::GenericSoundEssenceDescriptorAudioRefLevel,
      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x04, 0x02, 0x01, 0x01, 0x04, 0x00, 0x00, 0x00) =>
        Ul::GenericSoundEssenceDescriptorChannelCount,


      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x04, 0x02, 0x03, 0x02, 0x01, 0x00, 0x00, 0x00) |
      smpte_identifier!(0x01, 0x01, 0x01, 0x08, 0x04, 0x02, 0x03, 0x02, 0x01, 0x00, 0x00, 0x00) =>
        Ul::WaveEssenceDescriptor_BlockAlign,
      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x04, 0x02, 0x03, 0x02, 0x02, 0x00, 0x00, 0x00) |
      smpte_identifier!(0x01, 0x01, 0x01, 0x08, 0x04, 0x02, 0x03, 0x02, 0x02, 0x00, 0x00, 0x00) =>
        Ul::WaveEssenceDescriptor_SequenceOffset,
      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x04, 0x02, 0x03, 0x03, 0x05, 0x00, 0x00, 0x00) |
      smpte_identifier!(0x01, 0x01, 0x01, 0x08, 0x04, 0x02, 0x03, 0x03, 0x05, 0x00, 0x00, 0x00) =>
        Ul::WaveEssenceDescriptor_AvgBps,
      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x04, 0x02, 0x01, 0x01, 0x05, 0x00, 0x00, 0x00) |
      smpte_identifier!(0x01, 0x01, 0x01, 0x08, 0x04, 0x02, 0x01, 0x01, 0x05, 0x00, 0x00, 0x00) =>
        Ul::WaveEssenceDescriptor_ChannelAssignment,
      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x04, 0x02, 0x03, 0x01, 0x06, 0x00, 0x00, 0x00) |
      smpte_identifier!(0x01, 0x01, 0x01, 0x08, 0x04, 0x02, 0x03, 0x01, 0x06, 0x00, 0x00, 0x00) =>
        Ul::WaveEssenceDescriptor_PeakEnvelopeVersion,
      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x04, 0x02, 0x03, 0x01, 0x07, 0x00, 0x00, 0x00) |
      smpte_identifier!(0x01, 0x01, 0x01, 0x08, 0x04, 0x02, 0x03, 0x01, 0x07, 0x00, 0x00, 0x00) =>
        Ul::WaveEssenceDescriptor_PeakEnvelopeFormat,
      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x04, 0x02, 0x03, 0x01, 0x08, 0x00, 0x00, 0x00) |
      smpte_identifier!(0x01, 0x01, 0x01, 0x08, 0x04, 0x02, 0x03, 0x01, 0x08, 0x00, 0x00, 0x00) =>
        Ul::WaveEssenceDescriptor_PointsPerPeakValue,
      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x04, 0x02, 0x03, 0x01, 0x09, 0x00, 0x00, 0x00) |
      smpte_identifier!(0x01, 0x01, 0x01, 0x08, 0x04, 0x02, 0x03, 0x01, 0x09, 0x00, 0x00, 0x00) =>
        Ul::WaveEssenceDescriptor_PeakEnvelopeBlockSize,
      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x04, 0x02, 0x03, 0x01, 0x0a, 0x00, 0x00, 0x00) |
      smpte_identifier!(0x01, 0x01, 0x01, 0x08, 0x04, 0x02, 0x03, 0x01, 0x0a, 0x00, 0x00, 0x00) =>
        Ul::WaveEssenceDescriptor_PeakChannels,
      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x04, 0x02, 0x03, 0x01, 0x0b, 0x00, 0x00, 0x00) |
      smpte_identifier!(0x01, 0x01, 0x01, 0x08, 0x04, 0x02, 0x03, 0x01, 0x0b, 0x00, 0x00, 0x00) =>
        Ul::WaveEssenceDescriptor_PeakFrames,
      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x04, 0x02, 0x03, 0x01, 0x0c, 0x00, 0x00, 0x00) |
      smpte_identifier!(0x01, 0x01, 0x01, 0x08, 0x04, 0x02, 0x03, 0x01, 0x0c, 0x00, 0x00, 0x00) =>
        Ul::WaveEssenceDescriptor_PeakOfPeaksPosition,
      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x04, 0x02, 0x03, 0x01, 0x0d, 0x00, 0x00, 0x00) |
      smpte_identifier!(0x01, 0x01, 0x01, 0x08, 0x04, 0x02, 0x03, 0x01, 0x0d, 0x00, 0x00, 0x00) =>
        Ul::WaveEssenceDescriptor_PeakEnvelopeTimestamp,
      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x04, 0x02, 0x03, 0x01, 0x0e, 0x00, 0x00, 0x00) |
      smpte_identifier!(0x01, 0x01, 0x01, 0x08, 0x04, 0x02, 0x03, 0x01, 0x0e, 0x00, 0x00, 0x00) =>
        Ul::WaveEssenceDescriptor_PeakEnvelopeData,

      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x04, 0x02, 0x05, 0x01, 0x01, 0x00, 0x00, 0x00) =>
        Ul::Aes3AudioDescriptorAuxBitsMode,
      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x04, 0x02, 0x03, 0x02, 0x03, 0x00, 0x00, 0x00) =>
        Ul::Aes3AudioDescriptorBlockStartOffset,
      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x04, 0x02, 0x05, 0x01, 0x02, 0x00, 0x00, 0x00) =>
        Ul::Aes3AudioDescriptorChannelStatusMode,
      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x04, 0x02, 0x05, 0x01, 0x03, 0x00, 0x00, 0x00) =>
        Ul::Aes3AudioDescriptorFixedChannelStatusData,
      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x04, 0x02, 0x05, 0x01, 0x04, 0x00, 0x00, 0x00) =>
        Ul::Aes3AudioDescriptorUserDataMode,
      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x04, 0x02, 0x05, 0x01, 0x05, 0x00, 0x00, 0x00) =>
        Ul::Aes3AudioDescriptorFixedUserData,
      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x04, 0x02, 0x05, 0x01, 0x06, 0x00, 0x00, 0x00) =>
        Ul::Aes3AudioDescriptorEmphasis,

      smpte_identifier!(0x01, 0x01, 0x01, 0x04, 0x06, 0x01, 0x01, 0x04, 0x06, 0x0b, 0x00, 0x00) =>
        Ul::MultipleDescriptorSubDescriptorUids,
      smpte_identifier!(0x01, 0x01, 0x01, 0x04, 0x01, 0x03, 0x04, 0x05, 0x00, 0x00, 0x00, 0x00) =>
        Ul::IndexSid,
      smpte_identifier!(0x01, 0x01, 0x01, 0x04, 0x01, 0x03, 0x04, 0x04, 0x00, 0x00, 0x00, 0x00) =>
        Ul::BodySid,
      smpte_identifier!(0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x15, 0x10, 0x00, 0x00, 0x00, 0x00) =>
        Ul::GenericPackagePackageUid,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x06, 0x01, 0x01, 0x04, 0x06, 0x05, 0x00, 0x00) =>
        Ul::GenericPackageTracks,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x07, 0x02, 0x01, 0x10, 0x02, 0x05, 0x00, 0x00) =>
        Ul::GenericPackagePackageModifiedDate,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x07, 0x02, 0x01, 0x10, 0x01, 0x03, 0x00, 0x00) =>
        Ul::GenericPackagePackageCreationDate,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x06, 0x01, 0x01, 0x04, 0x02, 0x03, 0x00, 0x00) =>
        Ul::SourcePackageDescriptor,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x01, 0x07, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00) =>
        Ul::GenericTrackTrackId,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x01, 0x07, 0x01, 0x02, 0x01, 0x00, 0x00, 0x00) =>
        Ul::GenericTrackTrackName,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x06, 0x01, 0x01, 0x04, 0x02, 0x04, 0x00, 0x00) =>
        Ul::GenericTrackSequence,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x01, 0x04, 0x01, 0x03, 0x00, 0x00, 0x00, 0x00) =>
        Ul::GenericTrackTrackNumber,

      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x05, 0x30, 0x04, 0x05, 0x00, 0x00, 0x00, 0x00) =>
        Ul::TrackEditRate,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x07, 0x02, 0x01, 0x03, 0x01, 0x03, 0x00, 0x00) =>
        Ul::TrackOrigin,

      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x06, 0x01, 0x01, 0x04, 0x02, 0x0c, 0x00, 0x00) =>
        Ul::DMSegmentDMFramework,

      smpte_identifier!(0x01, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x07, 0x01, 0x0a, 0x01, 0x01, 0x00) |
      smpte_identifier!(0x02, 0x53, 0x01, 0x01, 0x0d, 0x01, 0x07, 0x01, 0x0a, 0x01, 0x01, 0x00) =>
        Ul::AS10CoreFramework,
      smpte_identifier!(0x01, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x07, 0x01, 0x0a, 0x01, 0x01, 0x01) =>
        Ul::AS10ShimName,


      smpte_identifier!(0x01, 0x01, 0x01, 0x01, 0x01, 0x03, 0x03, 0x02, 0x01, 0x00, 0x00, 0x00) =>
        Ul::PackageName,

      smpte_identifier!(0x01, 0x01, 0x01, 0x09, 0x06, 0x01, 0x01, 0x04, 0x06, 0x10, 0x00, 0x00) =>
        Ul::SubDescriptors,

      smpte_identifier!(0x01, 0x01, 0x01, 0x07, 0x06, 0x01, 0x01, 0x03, 0x07, 0x00, 0x00, 0x00) =>
        Ul::SourceDescriptor_ChannelIDs,
      smpte_identifier!(0x01, 0x01, 0x01, 0x08, 0x06, 0x01, 0x01, 0x03, 0x08, 0x00, 0x00, 0x00) =>
        Ul::SourceDescriptor_MonoSourceTrackIDs,
      smpte_identifier!(0x01, 0x01, 0x01, 0x09, 0x04, 0x01, 0x02, 0x01, 0x01, 0x06, 0x01, 0x00) =>
        Ul::ColorPrimaries,
      smpte_identifier!(0x01, 0x01, 0x01, 0x01, 0x04, 0x02, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00) =>
        Ul::ElectrospatialFormulation,
      smpte_identifier!(0x01, 0x01, 0x01, 0x02, 0x04, 0x02, 0x04, 0x02, 0x00, 0x00, 0x00, 0x00) =>
        Ul::SoundCompression,
      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x04, 0x02, 0x07, 0x01, 0x00, 0x00, 0x00, 0x00) =>
        Ul::DialNorm,
      smpte_identifier!(0x01, 0x01, 0x01, 0x07, 0x04, 0x02, 0x01, 0x01, 0x05, 0x00, 0x00, 0x00) =>
        Ul::ChannelAssignment,
      smpte_identifier!(0x01, 0x01, 0x01, 0x0d, 0x03, 0x01, 0x01, 0x02, 0x03, 0x15, 0x00, 0x00) =>
        Ul::RFC5646SpokenLanguage,

      smpte_identifier!(0x01, 0x01, 0x01, 0x0e, 0x01, 0x01, 0x11, 0x0c, 0x00, 0x00, 0x00, 0x00) =>
        Ul::Eidr_CanonicalDOIName,
      smpte_identifier!(0x01, 0x01, 0x01, 0x0e, 0x01, 0x01, 0x11, 0x0d, 0x00, 0x00, 0x00, 0x00) =>
        Ul::Eidr_CanonicalEIDRIdentifier,
      smpte_identifier!(0x01, 0x01, 0x01, 0x0e, 0x01, 0x01, 0x11, 0x0e, 0x00, 0x00, 0x00, 0x00) =>
        Ul::Eidr_CompactAdIDIdentifier,
      smpte_identifier!(0x01, 0x01, 0x01, 0x0e, 0x01, 0x01, 0x15, 0x14, 0x00, 0x00, 0x00, 0x00) =>
        Ul::Eidr_DMSEssenceID,

      smpte_identifier!(0x01, 0x01, 0x01, 0x0e, 0x01, 0x03, 0x07, 0x01, 0x01, 0x00, 0x00, 0x00) =>
        Ul::MCALabelDictionaryID,
      smpte_identifier!(0x01, 0x01, 0x01, 0x0e, 0x01, 0x03, 0x07, 0x01, 0x02, 0x00, 0x00, 0x00) =>
        Ul::MCATagSymbol,
      smpte_identifier!(0x01, 0x01, 0x01, 0x0e, 0x01, 0x03, 0x07, 0x01, 0x03, 0x00, 0x00, 0x00) =>
        Ul::MCATagName,
      smpte_identifier!(0x01, 0x01, 0x01, 0x0e, 0x01, 0x03, 0x07, 0x01, 0x04, 0x00, 0x00, 0x00) =>
        Ul::GroupOfSoundfieldGroupsLinkID,
      smpte_identifier!(0x01, 0x01, 0x01, 0x0e, 0x01, 0x03, 0x07, 0x01, 0x05, 0x00, 0x00, 0x00) =>
        Ul::MCALinkID,
      smpte_identifier!(0x01, 0x01, 0x01, 0x0e, 0x01, 0x03, 0x07, 0x01, 0x06, 0x00, 0x00, 0x00) =>
        Ul::SoundfieldGroupLinkID,
      smpte_identifier!(0x01, 0x01, 0x01, 0x0e, 0x01, 0x03, 0x04, 0x0a, 0x00, 0x00, 0x00, 0x00) =>
        Ul::MCAChannelID,
      smpte_identifier!(0x01, 0x01, 0x01, 0x0e, 0x01, 0x05, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00) =>
        Ul::MCATitle,
      smpte_identifier!(0x01, 0x01, 0x01, 0x0e, 0x01, 0x05, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00) =>
        Ul::MCATitleVersion,
      smpte_identifier!(0x01, 0x01, 0x01, 0x0e, 0x01, 0x05, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00) =>
        Ul::MCATitleSubVersion,
      smpte_identifier!(0x01, 0x01, 0x01, 0x0e, 0x01, 0x05, 0x13, 0x00, 0x00, 0x00, 0x00, 0x00) =>
        Ul::MCAEpisode,
      smpte_identifier!(0x01, 0x01, 0x01, 0x0e, 0x01, 0x04, 0x01, 0x05, 0x00, 0x00, 0x00, 0x00) =>
        Ul::MCAPartitionKind,
      smpte_identifier!(0x01, 0x01, 0x01, 0x0e, 0x01, 0x04, 0x01, 0x06, 0x00, 0x00, 0x00, 0x00) =>
        Ul::MCAPartitionNumber,
      smpte_identifier!(0x01, 0x01, 0x01, 0x0e, 0x03, 0x02, 0x01, 0x02, 0x20, 0x00, 0x00, 0x00) =>
        Ul::MCAAudioContentKind,
      smpte_identifier!(0x01, 0x01, 0x01, 0x0e, 0x03, 0x02, 0x01, 0x02, 0x21, 0x00, 0x00, 0x00) =>
        Ul::MCAAudioElementKind,

      smpte_identifier!(0x01, 0x01, 0x01, 0x0e, 0x04, 0x02, 0x01, 0x01, 0x06, 0x00, 0x00, 0x00) =>
        Ul::ReferenceImageEditRate,
      smpte_identifier!(0x01, 0x01, 0x01, 0x0e, 0x04, 0x02, 0x01, 0x01, 0x07, 0x00, 0x00, 0x00) =>
        Ul::ReferenceAudioAlignmentLevel,

      smpte_identifier!(0x04, 0x01, 0x01, 0x07, 0x04, 0x01, 0x02, 0x02, 0x03, 0x01, 0x01, 0x00) =>
        Ul::IsoIec154441Jpeg2002,
      smpte_identifier!(0x04, 0x01, 0x01, 0x07, 0x04, 0x01, 0x02, 0x02, 0x03, 0x01, 0x01, 0x01) =>
        Ul::JPEG2000DigitalCinemaProfile,
      smpte_identifier!(0x04, 0x01, 0x01, 0x09, 0x04, 0x01, 0x02, 0x02, 0x03, 0x01, 0x01, 0x02) =>
        Ul::JPEG2000Amd12KDigitalCinemaProfile,
      smpte_identifier!(0x04, 0x01, 0x01, 0x09, 0x04, 0x01, 0x02, 0x02, 0x03, 0x01, 0x01, 0x03) =>
        Ul::JPEG2000Amd14KDigitalCinemaProfile,
      smpte_identifier!(0x04, 0x01, 0x01, 0x0d, 0x04, 0x01, 0x02, 0x02, 0x03, 0x01, 0x01, 0x11) =>
        Ul::JPEG2000BroadcastContributionSingleTileProfileLevel1,
      smpte_identifier!(0x04, 0x01, 0x01, 0x0d, 0x04, 0x01, 0x02, 0x02, 0x03, 0x01, 0x01, 0x12) =>
        Ul::JPEG2000BroadcastContributionSingleTileProfileLevel2,
      smpte_identifier!(0x04, 0x01, 0x01, 0x0d, 0x04, 0x01, 0x02, 0x02, 0x03, 0x01, 0x01, 0x13) =>
        Ul::JPEG2000BroadcastContributionSingleTileProfileLevel3,
      smpte_identifier!(0x04, 0x01, 0x01, 0x0d, 0x04, 0x01, 0x02, 0x02, 0x03, 0x01, 0x01, 0x14) =>
        Ul::JPEG2000BroadcastContributionSingleTileProfileLevel4,
      smpte_identifier!(0x04, 0x01, 0x01, 0x0d, 0x04, 0x01, 0x02, 0x02, 0x03, 0x01, 0x01, 0x15) =>
        Ul::JPEG2000BroadcastContributionSingleTileProfileLevel5,
      smpte_identifier!(0x04, 0x01, 0x01, 0x0d, 0x04, 0x01, 0x02, 0x02, 0x03, 0x01, 0x01, 0x16) =>
        Ul::JPEG2000BroadcastContributionMultiTileReversibleProfileLevel6,
      smpte_identifier!(0x04, 0x01, 0x01, 0x0a, 0x04, 0x01, 0x02, 0x02, 0x03, 0x01, 0x01, 0x7f) =>
        Ul::JPEG2000UndefinedDigitalCinemaProfile,

      smpte_identifier!(0x01, 0x01, 0x01, 0x0e, 0x04, 0x01, 0x06, 0x03, 0x0e, 0x00, 0x00, 0x00) =>
        Ul::Jpeg2000VideoDescriptor_J2CLayout,

      smpte_identifier!(0x01, 0x01, 0x01, 0x05, 0x04, 0x01, 0x06, 0x02, 0x01, designator, 0x00, 0x00) =>
        return build_mpeg_description_key(designator),
      smpte_identifier!(0x01, 0x01, 0x01, 0x0a, 0x04, 0x01, 0x06, 0x03, designator, 0x00, 0x00, 0x00) =>
        return build_jpeg2000_description_key(designator),

      build_identifier!(version_number => 0x01, kind => 0x01) => {
        Ul::Essence_BwfFrameWrapped
      },
      build_identifier!(version_number => 0x01, kind => 0x02) => {
        Ul::Essence_BwfClipWrapped
      },
      build_identifier!(version_number => 0x01, kind => 0x03) => {
        Ul::Essence_AesFrameWrapped
      },
      build_identifier!(version_number => 0x01, kind => 0x04) => {
        Ul::Essence_AesClipWrapped
      },

      build_identifier!(version_number => 0x02, mpeg_es => 0x01) => {
        Ul::Essence_MpegEsWithStreamIdFrameWrapped
      },
      build_identifier!(version_number => 0x02, mpeg_es => 0x02) => {
        Ul::Essence_MpegEsWithStreamIdClipWrapped
      },
      build_identifier!(version_number => 0x02, mpeg_es => 0x03) => {
        Ul::Essence_MpegEsWithStreamIdStripeWrapped
      },
      build_identifier!(version_number => 0x02, mpeg_es => 0x05) => {
        Ul::Essence_MpegEsWithStreamIdFixedAudioSizeWrapped
      },
      build_identifier!(version_number => 0x02, mpeg_es => 0x06) => {
        Ul::Essence_MpegEsWithStreamIdSpliceWrapped
      },
      build_identifier!(version_number => 0x02, mpeg_es => 0x07) => {
        Ul::Essence_MpegEsWithStreamIdClosedGopWrapped
      },
      build_identifier!(version_number => 0x02, mpeg_es => 0x08) => {
        Ul::Essence_MpegEsWithStreamIdSlaveWrapped
      },
      build_identifier!(version_number => 0x02, mpeg_es => 0x7F) => {
        Ul::Essence_MpegEsWithStreamIdNoSpecificWrappingConstraints
      },

      build_identifier!(version_number => 0x0a, avc => 0x01) => {
        Ul::Essence_AVCByteStream_FrameWrapped
      },
      build_identifier!(version_number => 0x0a, avc => 0x02) => {
        Ul::Essence_AVCByteStream_ClipedWrapped
      },
      build_identifier!(version_number => 0x0a, avc => 0x03) => {
        Ul::Essence_AVCByteStream_StripedWrapped
      },
      build_identifier!(version_number => 0x0a, avc => 0x04) => {
        Ul::Essence_AVCByteStream_PesWrapped
      },
      build_identifier!(version_number => 0x0a, avc => 0x05) => {
        Ul::Essence_AVCByteStream_FixedAudioSizeWrapped
      },
      build_identifier!(version_number => 0x0a, avc => 0x06) => {
        Ul::Essence_AVCByteStream_SpliceWrapped
      },
      build_identifier!(version_number => 0x0a, avc => 0x07) => {
        Ul::Essence_AVCByteStream_ClosedGopWrapped
      },
      build_identifier!(version_number => 0x0a, avc => 0x08) => {
        Ul::Essence_AVCByteStream_SlaveWrapped
      },

      build_identifier!(version_number => 0x07, jpeg2000 => 0x01) => {
        Ul::Essence_Jpeg2000_FrameWrapped
      },
      build_identifier!(version_number => 0x07, jpeg2000 => 0x06) => {
        Ul::Essence_Jpeg2000_FrameWrapped
      },
      build_identifier!(version_number => 0x0d, gc => 0x06) => {
        Ul::Essence_MXFGCP1FrameWrappedPictureElement
      },

      smpte_identifier!(0x04, 0x01, 0x01, 0x03, 0x0d, 0x01, 0x03, 0x01, 0x02, 0x7F, 0x01, 0x00) => {
        Ul::Essence_GenericEssenceContainerMultipleWrappings
      },
      smpte_identifier!(0x04, 0x01, 0x01, 0x09, 0x0d, 0x01, 0x03, 0x01, 0x02, 0x0e, 0x00, 0x00) => {
        Ul::Essence_GenericEssenceContainerMultipleWrappings
      },

      smpte_identifier!(0x04, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, 0x01, 0x01, 0x01, 0x00) => {
        Ul::MxfOP1aSingleItemSinglePackageUniTrackStreamInternal
      },
      smpte_identifier!(0x04, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, 0x01, 0x01, 0x03, 0x00) => {
        Ul::MxfOP1aSingleItemSinglePackageUniTrackStreamExternal
      },
      smpte_identifier!(0x04, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, 0x01, 0x01, 0x05, 0x00) => {
        Ul::MxfOP1aSingleItemSinglePackageUniTrackNonStreamInternal
      },
      smpte_identifier!(0x04, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, 0x01, 0x01, 0x07, 0x00) => {
        Ul::MxfOP1aSingleItemSinglePackageUniTrackNonStreamExternal
      },
      smpte_identifier!(0x04, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, 0x01, 0x01, 0x09, 0x00) => {
        Ul::MxfOP1aSingleItemSinglePackageMultiTrackStreamInternal
      },
      smpte_identifier!(0x04, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, 0x01, 0x01, 0x0b, 0x00) => {
        Ul::MxfOP1aSingleItemSinglePackageMultiTrackStreamExternal
      },
      smpte_identifier!(0x04, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, 0x01, 0x01, 0x0d, 0x00) => {
        Ul::MxfOP1aSingleItemSinglePackageMultiTrackNonStreamInternal
      },
      smpte_identifier!(0x04, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, 0x01, 0x01, 0x0f, 0x00) => {
        Ul::MxfOP1aSingleItemSinglePackageMultiTrackNonStreamExternal
      },
      smpte_identifier!(0x04, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, 0x02, 0x01, 0x01, 0x00) => {
        Ul::MxfOP1bSingleItemGangedPackagesUniTrackStreamInternal
      },
      smpte_identifier!(0x04, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, 0x02, 0x01, 0x03, 0x00) => {
        Ul::MxfOP1bSingleItemGangedPackagesUniTrackStreamExternal
      },
      smpte_identifier!(0x04, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, 0x02, 0x01, 0x05, 0x00) => {
        Ul::MxfOP1bSingleItemGangedPackagesUniTrackNonStreamInternal
      },
      smpte_identifier!(0x04, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, 0x02, 0x01, 0x07, 0x00) => {
        Ul::MxfOP1bSingleItemGangedPackagesUniTrackNonStreamExternal
      },
      smpte_identifier!(0x04, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, 0x02, 0x01, 0x09, 0x00) => {
        Ul::MxfOP1bSingleItemGangedPackagesMultiTrackStreamInternal
      },
      smpte_identifier!(0x04, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, 0x02, 0x01, 0x0b, 0x00) => {
        Ul::MxfOP1bSingleItemGangedPackagesMultiTrackStreamExternal
      },
      smpte_identifier!(0x04, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, 0x02, 0x01, 0x0d, 0x00) => {
        Ul::MxfOP1bSingleItemGangedPackagesMultiTrackNonStreamInternal
      },
      smpte_identifier!(0x04, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, 0x02, 0x01, 0x0f, 0x00) => {
        Ul::MxfOP1bSingleItemGangedPackagesMultiTrackNonStreamExternal
      },

      build_identifier!(version_number => 0x01, coding_equations => 0x01) => {
        Ul::CodingEquations_ITU601
      },
      build_identifier!(version_number => 0x01, coding_equations => 0x02) => {
        Ul::CodingEquations_ITU709
      },
      build_identifier!(version_number => 0x01, coding_equations => 0x03) => {
        Ul::CodingEquations_SMPTE240M
      },

      build_identifier!(version_number => 0x06, color_primaries => 0x01) => {
        Ul::ColorPrimaries_SMPTE170M
      },
      build_identifier!(version_number => 0x06, color_primaries => 0x02) => {
        Ul::ColorPrimaries_ITU470_PAL
      },
      build_identifier!(version_number => 0x06, color_primaries => 0x03) => {
        Ul::ColorPrimaries_ITU709
      },
      build_identifier!(version_number => 0x0d, color_primaries => 0x04) => {
        Ul::ColorPrimaries_ITU2020
      },

      build_identifier!(version_number => 0x01, transfer_carateristic => 0x01) => {
        Ul::TransferCharacteristic_ITU470_PAL
      },
      build_identifier!(version_number => 0x01, transfer_carateristic => 0x02) => {
        Ul::TransferCharacteristic_ITU709
      },
      build_identifier!(version_number => 0x01, transfer_carateristic => 0x03) => {
        Ul::TransferCharacteristic_SMPTE240M
      },
      build_identifier!(version_number => 0x01, transfer_carateristic => 0x04) => {
        Ul::TransferCharacteristic_274M_296M
      },
      build_identifier!(version_number => 0x06, transfer_carateristic => 0x05) => {
        Ul::TransferCharacteristic_ITU1361
      },
      build_identifier!(version_number => 0x06, transfer_carateristic => 0x06) => {
        Ul::TransferCharacteristic_linear
      },
      build_identifier!(version_number => 0x08, transfer_carateristic => 0x07) => {
        Ul::TransferCharacteristic_SMPTE_DCDM
      },
      build_identifier!(version_number => 0x0d, transfer_carateristic => 0x08) => {
        Ul::TransferCharacteristic_IEC6196624_xvYCC
      },
      build_identifier!(version_number => 0x0e, transfer_carateristic => 0x09) => {
        Ul::TransferCharacteristic_ITU2020
      },

      build_identifier!(version_number => 0x03, mpeg2_profil => 0x04, mode => 0x02) => {
        Ul::Mpeg2_422_PHL_IFrame
      },
      build_identifier!(version_number => 0x03, mpeg2_profil => 0x04, mode => 0x03) => {
        Ul::Mpeg2_422_PHL_LongGOP
      },
      build_identifier!(version_number => 0x03, mpeg2_profil => 0x04, mode => 0x04) => {
        Ul::Mpeg2_422_PHL_NoIFrames
      },
      build_identifier!(version_number => 0x08, mpeg2_profil => 0x05, mode => 0x02) => {
        Ul::Mpeg2_MPH14_IFrame
      },
      build_identifier!(version_number => 0x08, mpeg2_profil => 0x05, mode => 0x03) => {
        Ul::Mpeg2_MPH14_LongGOP
      },
      build_identifier!(version_number => 0x08, mpeg2_profil => 0x05, mode => 0x04) => {
        Ul::Mpeg2_MPH14_NoIFrames
      },
      build_identifier!(version_number => 0x0a, mpeg4_avc => 0x01) => {
        Ul::H264_Mpeg4_AVC_High422Intra_RP2027ConstrainedClass100_1080_5994iCoding
      },
      build_identifier!(version_number => 0x0a, mpeg4_avc => 0x02) => {
        Ul::H264_Mpeg4_AVC_High422Intra_RP2027ConstrainedClass100_1080_50iCoding
      },
      build_identifier!(version_number => 0x0a, mpeg4_avc => 0x03) => {
        Ul::H264_Mpeg4_AVC_High422Intra_RP2027ConstrainedClass100_1080_2997pCoding
      },
      build_identifier!(version_number => 0x0a, mpeg4_avc => 0x04) => {
        Ul::H264_Mpeg4_AVC_High422Intra_RP2027ConstrainedClass100_1080_25pCoding
      },

      build_identifier!(version_number => 0x0a, uncompressed_sound_coding => 0x01) => {
        Ul::SMPTE382MDefaultUncompressedSoundCoding
      },
      build_identifier!(version_number => 0x07, uncompressed_sound_coding => 0x7e) => {
        Ul::AIFFUncompressedCoding
      },
      build_identifier!(version_number => 0x01, uncompressed_sound_coding => 0x7f) => {
        Ul::UndefinedSoundCoding
      },
      build_identifier!(version_number => 0x0d, audio_channel => 0x01) => {
        Ul::LeftAudioChannel
      },
      build_identifier!(version_number => 0x0d, audio_channel => 0x02) => {
        Ul::RightAudioChannel
      },
      build_identifier!(version_number => 0x0d, audio_channel => 0x03) => {
        Ul::CenterAudioChannel
      },
      build_identifier!(version_number => 0x0d, audio_channel => 0x04) => {
        Ul::LFEAudioChannel
      },
      build_identifier!(version_number => 0x0d, audio_channel => 0x05) => {
        Ul::LeftSurroundAudioChannel
      },
      build_identifier!(version_number => 0x0d, audio_channel => 0x06) => {
        Ul::RightSurroundAudioChannel
      },
      build_identifier!(version_number => 0x0d, audio_channel => 0x07) => {
        Ul::LeftSideSurroundAudioChannel
      },
      build_identifier!(version_number => 0x0d, audio_channel => 0x08) => {
        Ul::RightSideSurroundAudioChannel
      },
      build_identifier!(version_number => 0x0d, audio_channel => 0x09) => {
        Ul::LeftRearSurroundAudioChannel
      },
      build_identifier!(version_number => 0x0d, audio_channel => 0x0a) => {
        Ul::RightRearSurroundAudioChannel
      },
      build_identifier!(version_number => 0x0d, audio_channel => 0x0b) => {
        Ul::LeftCenterAudioChannel
      },
      build_identifier!(version_number => 0x0d, audio_channel => 0x0c) => {
        Ul::RightCenterAudioChannel
      },
      build_identifier!(version_number => 0x0d, audio_channel => 0x0d) => {
        Ul::CenterSurroundAudioChannel
      },
      build_identifier!(version_number => 0x0d, audio_channel => 0x0e) => {
        Ul::HearingImpairedAudioChannel
      },
      build_identifier!(version_number => 0x0d, audio_channel => 0x0f) => {
        Ul::VisuallyImpairedNarrativeAudioChannel
      },
      build_identifier!(version_number => 0x0d, audio_st20678 => 0x01) => {
        Ul::Smpte_ST_20678_MonoOne
      },
      build_identifier!(version_number => 0x0d, audio_st20678 => 0x02) => {
        Ul::Smpte_ST_20678_MonoTwo
      },
      build_identifier!(version_number => 0x0d, audio_st20678 => 0x03) => {
        Ul::Smpte_ST_20678_LeftTotal
      },
      build_identifier!(version_number => 0x0d, audio_st20678 => 0x04) => {
        Ul::Smpte_ST_20678_RightTotal
      },
      build_identifier!(version_number => 0x0d, audio_st20678 => 0x05) => {
        Ul::Smpte_ST_20678_LeftSurroundTotal
      },
      build_identifier!(version_number => 0x0d, audio_st20678 => 0x06) => {
        Ul::Smpte_ST_20678_RightSurroundTotal
      },
      build_identifier!(version_number => 0x0d, audio_st20678 => 0x07) => {
        Ul::Smpte_ST_20678_Surround
      },
      build_identifier!(version_number => 0x0d, audio_st20678 => 0x08, index => 0x01) => {
        Ul::Smpte_ST_20678_NumberedSourceChannel_001
      },
      build_identifier!(version_number => 0x0d, audio_st20678 => 0x08, index => 0x02) => {
        Ul::Smpte_ST_20678_NumberedSourceChannel_002
      },
      build_identifier!(version_number => 0x0d, audio_st20678 => 0x08, index => 0x03) => {
        Ul::Smpte_ST_20678_NumberedSourceChannel_003
      },
      build_identifier!(version_number => 0x0d, audio_st20678 => 0x08, index => 0x04) => {
        Ul::Smpte_ST_20678_NumberedSourceChannel_004
      },
      build_identifier!(version_number => 0x0d, audio_st20678 => 0x08, index => 0x05) => {
        Ul::Smpte_ST_20678_NumberedSourceChannel_005
      },
      build_identifier!(version_number => 0x0d, audio_st20678 => 0x08, index => 0x06) => {
        Ul::Smpte_ST_20678_NumberedSourceChannel_006
      },
      build_identifier!(version_number => 0x0d, audio_st20678 => 0x08, index => 0x07) => {
        Ul::Smpte_ST_20678_NumberedSourceChannel_007
      },
      build_identifier!(version_number => 0x0d, audio_st20678 => 0x08, index => 0x08) => {
        Ul::Smpte_ST_20678_NumberedSourceChannel_008
      },
      build_identifier!(version_number => 0x0d, audio_st20678 => 0x08, index => 0x09) => {
        Ul::Smpte_ST_20678_NumberedSourceChannel_009
      },
      build_identifier!(version_number => 0x0d, audio_st20678 => 0x08, index => 0x0a) => {
        Ul::Smpte_ST_20678_NumberedSourceChannel_00A
      },
      build_identifier!(version_number => 0x0d, audio_st20678 => 0x08, index => 0x0b) => {
        Ul::Smpte_ST_20678_NumberedSourceChannel_00B
      },
      build_identifier!(version_number => 0x0d, audio_st20678 => 0x08, index => 0x0c) => {
        Ul::Smpte_ST_20678_NumberedSourceChannel_00C
      },
      build_identifier!(version_number => 0x0d, audio_st20678 => 0x08, index => 0x0d) => {
        Ul::Smpte_ST_20678_NumberedSourceChannel_00D
      },
      build_identifier!(version_number => 0x0d, audio_st20678 => 0x08, index => 0x0e) => {
        Ul::Smpte_ST_20678_NumberedSourceChannel_00E
      },
      build_identifier!(version_number => 0x0d, audio_st20678 => 0x08, index => 0x0f) => {
        Ul::Smpte_ST_20678_NumberedSourceChannel_00F
      },

      build_identifier!(version_number => 0x0d, soundfield_st20678 => 0x01) => {
        Ul::Smpte_ST_20678_StandardStereo
      },
      build_identifier!(version_number => 0x0d, soundfield_st20678 => 0x02) => {
        Ul::Smpte_ST_20678_DualMono
      },
      build_identifier!(version_number => 0x0d, soundfield_st20678 => 0x03) => {
        Ul::Smpte_ST_20678_DiscreteNumberedSources
      },
      build_identifier!(version_number => 0x0d, soundfield_st20678 => 0x04) => {
        Ul::Smpte_ST_20678_30
      },
      build_identifier!(version_number => 0x0d, soundfield_st20678 => 0x05) => {
        Ul::Smpte_ST_20678_40
      },
      build_identifier!(version_number => 0x0d, soundfield_st20678 => 0x06) => {
        Ul::Smpte_ST_20678_50
      },
      build_identifier!(version_number => 0x0d, soundfield_st20678 => 0x07) => {
        Ul::Smpte_ST_20678_60
      },
      build_identifier!(version_number => 0x0d, soundfield_st20678 => 0x08) => {
        Ul::Smpte_ST_20678_70DS
      },
      build_identifier!(version_number => 0x0d, soundfield_st20678 => 0x09) => {
        Ul::Smpte_ST_20678_LtRt
      },
      build_identifier!(version_number => 0x0d, soundfield_st20678 => 0x0a) => {
        Ul::Smpte_ST_20678_51EX
      },
      build_identifier!(version_number => 0x0d, soundfield_st20678 => 0x0b) => {
        Ul::Smpte_ST_20678_HearingAccessibility
      },
      build_identifier!(version_number => 0x0d, soundfield_st20678 => 0x0c) => {
        Ul::Smpte_ST_20678_VisualAccessibility
      },
      smpte_identifier!(0x04, 0x01, 0x01, 0x0d, 0x04, 0x02, 0x02, 0x10, 0x04, 0x01, 0x00, 0x00) => {
        Ul::Smpte_ST_20672_ApplicationOfTheMXFMultichannelAudioFramework
      },
      build_identifier!(version_number => 0x0d, soundfield => 0x01) => {
        Ul::SoundfieldGroup_51
      },
      build_identifier!(version_number => 0x0d, soundfield => 0x02) => {
        Ul::SoundfieldGroup_71DS
      },
      build_identifier!(version_number => 0x0d, soundfield => 0x03) => {
        Ul::SoundfieldGroup_71SDS
      },
      build_identifier!(version_number => 0x0d, soundfield => 0x04) => {
        Ul::SoundfieldGroup_61
      },
      build_identifier!(version_number => 0x0d, soundfield => 0x05) => {
        Ul::SoundfieldGroup_Monaural
      },
      _ => {
        println!("Unknown key: {}", format_ul(&data));
        return None
      }
    };

  Some(ul)
}

fn build_mpeg_description_key(designator: u8) -> Option<Ul> {
  match designator {
    0x02 => Some(Ul::Mpeg2VideoDescriptorSingleSequence),
    0x03 => Some(Ul::Mpeg2VideoDescriptorConstantBframes),
    0x04 => Some(Ul::Mpeg2VideoDescriptorCodedContentType),
    0x05 => Some(Ul::Mpeg2VideoDescriptorLowDelay),
    0x06 => Some(Ul::Mpeg2VideoDescriptorClosedGOP),
    0x07 => Some(Ul::Mpeg2VideoDescriptorIdenticalGOP),
    0x08 => Some(Ul::Mpeg2VideoDescriptorMaxGOP),
    0x09 => Some(Ul::Mpeg2VideoDescriptorBPictureCount),
    0x0A => Some(Ul::Mpeg2VideoDescriptorProfileAndLevel),
    0x0B => Some(Ul::Mpeg2VideoDescriptorBitRate),
       _ => None,
  }
}

fn build_jpeg2000_description_key(designator: u8) -> Option<Ul> {
  match designator {
    0x01 => Some(Ul::Jpeg2000VideoDescriptor_Rsiz),
    0x02 => Some(Ul::Jpeg2000VideoDescriptor_Xsiz),
    0x03 => Some(Ul::Jpeg2000VideoDescriptor_Ysiz),
    0x04 => Some(Ul::Jpeg2000VideoDescriptor_XOsiz),
    0x05 => Some(Ul::Jpeg2000VideoDescriptor_YOsiz),
    0x06 => Some(Ul::Jpeg2000VideoDescriptor_XTsiz),
    0x07 => Some(Ul::Jpeg2000VideoDescriptor_YTsiz),
    0x08 => Some(Ul::Jpeg2000VideoDescriptor_XTOsiz),
    0x09 => Some(Ul::Jpeg2000VideoDescriptor_YTOsiz),
    0x0A => Some(Ul::Jpeg2000VideoDescriptor_Csiz),
    0x0B => Some(Ul::Jpeg2000VideoDescriptor_PictureComponentSizing),
    0x0C => Some(Ul::Jpeg2000VideoDescriptor_CodingStyleDefault),
    0x0D => Some(Ul::Jpeg2000VideoDescriptor_QuantizationDefault),
       _ => None,
  }
}

pub fn format_ul(key : &Vec<u8>) -> String {
  let (first, next) = key.split_at(1);
  let mut string_key: String = format!("0x{:02x}", first[0]);
  let smpte_string_key: String = format!(
    "{:02x}{:02x}{:02x}{:02x}.{:02x}{:02x}{:02x}{:02x}.{:02x}{:02x}{:02x}{:02x}.{:02x}{:02x}{:02x}{:02x}",
    key[0], key[1], key[2], key[3],
    key[4], key[5], key[6], key[7],
    key[8], key[9], key[10], key[11],
    key[12], key[13], key[14], key[15],
    );
  for v in next {
    string_key = format!("{}, 0x{:02x}", string_key, v);
  }

  // format!("{} ({})", string_key, smpte_string_key)
  smpte_string_key
}
