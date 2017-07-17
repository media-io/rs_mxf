
#[derive(Debug, PartialEq, Clone)]
pub enum PartitionStatus {
  OpenAndIncomplete,
  ClosedAndIncomplete,
  OpenAndComplete,
  ClosedAndComplete,
}

pub fn partition_status_value(status: PartitionStatus) -> u8 {
  match status {
    PartitionStatus::OpenAndIncomplete => 0x01,
    PartitionStatus::ClosedAndIncomplete => 0x02,
    PartitionStatus::OpenAndComplete => 0x03,
    PartitionStatus::ClosedAndComplete => 0x04
  }
}

macro_rules! partition_status_value {
  (PartitionStatus::OpenAndIncomplete) => (0x01);
  (PartitionStatus::ClosedAndIncomplete) => (0x02);
  (PartitionStatus::OpenAndComplete) => (0x03);
  (PartitionStatus::ClosedAndComplete) => (0x04);
}

pub fn parse_status(s: u8) -> PartitionStatus {
  match s {
    0x01 => PartitionStatus::OpenAndIncomplete,
    0x02 => PartitionStatus::ClosedAndIncomplete,
    0x03 => PartitionStatus::OpenAndComplete,
    0x04 => PartitionStatus::ClosedAndComplete,
       _ => panic!("Unknown partition status"),
  }
}
