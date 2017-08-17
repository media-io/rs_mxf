
#[macro_use] pub mod ul;
pub mod decoder;
pub mod encoder;

use klv::value::partition::*;

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Ul {
  HeaderPartition {status: Option<PartitionStatus>},
  BodyPartition {status: Option<PartitionStatus>},
  FooterPartition {status: Option<PartitionStatus>},
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

  DescribedTrackIDs,

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
  Essence_AncFrameElement,

  PartitionMajor,
  PartitionMinor,
  PartitionKagSize,
  PartitionThisPartition,
  PartitionPreviousPartition,
  PartitionFooterPartition,
  PartitionHeaderByteCount,
  PartitionIndexByteCount,
  PartitionIndexSid,
  PartitionBodyOffset,
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

  Mpeg2_422P_HL_IFrame,
  Mpeg2_422P_HL_LongGOP,
  Mpeg2_422P_HL_NoIFrames,
  Mpeg2_MP_HL_IFrame,
  Mpeg2_MP_HL_LongGOP,
  Mpeg2_MP_HL_NoIFrames,
  Mpeg2_MP_H14_IFrame,
  Mpeg2_MP_H14_LongGOP,
  Mpeg2_MP_H14_NoIFrames,

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