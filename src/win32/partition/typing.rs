
use serde_repr::{Deserialize_repr, Serialize_repr};

/// `OperationalStatus` values
#[derive(Deserialize_repr, Serialize_repr, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u16)]
pub enum OperationalStatus {
    Unknown = 0,
    Online = 1,
    NoMedia = 3,
    Offline = 4,
    Failed = 5,
}

/// `TransitionState` values
#[derive(Deserialize_repr, Serialize_repr, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u16)]
pub enum TransitionState {
    Reserved = 0,
    Stable = 1,
    Extending = 2,
    Shrinking = 3,
    Reconfiguring = 4,
    Restriping = 8,
}

/// `MbrType` values (only for MBR disks)
#[derive(Deserialize_repr, Serialize_repr, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u16)]
pub enum MbrType {
    FAT12 = 1,
    FAT16 = 4,
    Extended = 5,
    Huge = 6,
    IFS = 7,
    FAT32 = 12,
}
