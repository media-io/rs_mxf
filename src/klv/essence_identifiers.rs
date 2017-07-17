
#[derive(Debug, Clone, PartialEq)]
pub enum EssenceIdentifier {
  BwfFrameWrapped,
  BwfClipWrapped,
  AesFrameWrapped,
  AesClipWrapped,
  MpegEsWithStreamIdFrameWrapped,
  MpegEsWithStreamIdClipWrapped,
  MpegEsWithStreamIdStripeWrapped,
  MpegEsWithStreamIdFixedAudioSizeWrapped,
  MpegEsWithStreamIdSpliceWrapped,
  MpegEsWithStreamIdClosedGopWrapped,
  MpegEsWithStreamIdSlaveWrapped,
  MpegEsWithStreamIdNoSpecificWrappingConstraints,
  GenericEssenceContainerMultipleWrappings,
  Unknown
}

macro_rules! build_identifier {
  (version_number => $vn:expr, kind => $k:expr) => (
    smpte_identifier!(0x04, 0x01, 0x01, $vn, 0x0d, 0x01, 0x03, 0x01, 0x02, 0x06, $k, 0x00)
  );
  (version_number => $vn:expr, mpeg_es => $me:expr) => (
    smpte_identifier!(0x04, 0x01, 0x01, $vn, 0x0d, 0x01, 0x03, 0x01, 0x02, 0x04, _, $me)
  );
}

pub fn parse_essence_ul(ul: Vec<u8>) -> EssenceIdentifier {
  match (ul[0], ul[1], ul[2], ul[3], ul[4], ul[5], ul[6], ul[7], ul[8], ul[9], ul[10], ul[11], ul[12], ul[13], ul[14], ul[15]) {
    build_identifier!(version_number => 0x01, kind => 0x01) => {
      EssenceIdentifier::BwfFrameWrapped
    },
    build_identifier!(version_number => 0x01, kind => 0x02) => {
      EssenceIdentifier::BwfClipWrapped
    },
    build_identifier!(version_number => 0x01, kind => 0x03) => {
      EssenceIdentifier::AesFrameWrapped
    },
    build_identifier!(version_number => 0x01, kind => 0x04) => {
      EssenceIdentifier::AesClipWrapped
    },

    build_identifier!(version_number => 0x02, mpeg_es => 0x01) => {
      EssenceIdentifier::MpegEsWithStreamIdFrameWrapped
    },
    build_identifier!(version_number => 0x02, mpeg_es => 0x02) => {
      EssenceIdentifier::MpegEsWithStreamIdClipWrapped
    },
    build_identifier!(version_number => 0x02, mpeg_es => 0x03) => {
      EssenceIdentifier::MpegEsWithStreamIdStripeWrapped
    },
    build_identifier!(version_number => 0x02, mpeg_es => 0x05) => {
      EssenceIdentifier::MpegEsWithStreamIdFixedAudioSizeWrapped
    },
    build_identifier!(version_number => 0x02, mpeg_es => 0x06) => {
      EssenceIdentifier::MpegEsWithStreamIdSpliceWrapped
    },
    build_identifier!(version_number => 0x02, mpeg_es => 0x07) => {
      EssenceIdentifier::MpegEsWithStreamIdClosedGopWrapped
    },
    build_identifier!(version_number => 0x02, mpeg_es => 0x08) => {
      EssenceIdentifier::MpegEsWithStreamIdSlaveWrapped
    },
    build_identifier!(version_number => 0x02, mpeg_es => 0x7F) => {
      EssenceIdentifier::MpegEsWithStreamIdNoSpecificWrappingConstraints
    },

    (0x06, 0x0e, 0x2b, 0x34, 0x04, 0x01, 0x01, 0x03, 0x0d, 0x01, 0x03, 0x01, 0x02, 0x7F, 0x01,  0x00) => {
      EssenceIdentifier::GenericEssenceContainerMultipleWrappings
    },
    _ => {
      EssenceIdentifier::Unknown
    }
  }
}
