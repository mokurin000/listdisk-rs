use serde_repr::{Deserialize_repr, Serialize_repr};

/// `HealthStatus` values
#[derive(Deserialize_repr, Serialize_repr, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u16)]
pub enum HealthStatus {
    Healthy = 0,
    ScanNeeded = 1,
    SpotFixNeeded = 2,
    FullRepairNeeded = 3,
}

/// `FileSystemType` values (Windows 10+)
#[derive(Deserialize_repr, Serialize_repr, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u16)]
#[allow(non_camel_case_types)]
pub enum FileSystemType {
    Unknown = 0,
    UFS = 2,
    HFS = 3,
    FAT = 4,
    FAT16 = 5,
    FAT32 = 6,
    NTFS4 = 7,
    NTFS5 = 8,
    XFS = 9,
    AFS = 10,
    EXT2 = 11,
    EXT3 = 12,
    ReiserFS = 13,
    NTFS = 14,
    ReFS = 15,
    CSVFS_NTFS = 0x8000,
    CSVFS_ReFS = 0x8001,
}

/// `DriveType` values
#[derive(Deserialize_repr, Serialize_repr, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum DriveType {
    Unknown = 0,
    InvalidRootPath = 1,
    Removable = 2,
    Fixed = 3,
    Remote = 4,
    CDROM = 5,
    RAMDisk = 6,
}

/// `DedupMode` values (Windows 10+)
#[derive(Deserialize_repr, Serialize_repr, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum DedupMode {
    Disabled = 0,
    GeneralPurpose = 1,
    HyperV = 2,
    Backup = 3,
    NotAvailable = 4,
}
