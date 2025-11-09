use serde::{Deserialize, Serialize};

/// Represents a volume on a computer as exposed by the `MSFT_Volume` WMI class.
/// Namespace: `Root\Microsoft\Windows\Storage`
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(rename = "MSFT_Volume")]
#[serde(rename_all = "PascalCase")]
pub struct Volume {
    /// Object Path for associated queries, method call.
    #[serde(rename = "__Path")]
    pub obj_path: String,

    /// ObjectId is a mandatory property that is used to opaquely and uniquely identify
    /// an instance of a class. ObjectId values are required to be globally unique.
    ///
    /// That is, no two objects should ever have the same ObjectId,
    /// even if they are managed by separate storage management providers,
    /// or are on different storage subsystems.
    pub object_id: String,

    /// UniqueId is a mandatory property that is used to uniquely identify a logical
    /// instance of a storage subsystem's object.
    ///
    /// This value must be the same for an object viewed by two or more provider instances,
    /// even if they are running on separate management servers.
    pub unique_id: String,

    /// Assigned drive letter (e.g., `'C'`), or `None` if unassigned.
    #[serde(default)]
    pub drive_letter: Option<char>,

    /// Full path to the volume (e.g., `\\?\Volume{...}\`).
    pub path: String,

    /// Health status of the volume.
    pub health_status: HealthStatus,

    /// File system name: "NTFS", "ReFS", "FAT32", "CSVFS", etc.
    #[serde(default)]
    pub file_system: Option<String>,

    /// User-defined volume label.
    #[serde(default)]
    pub file_system_label: Option<String>,

    /// Underlying file system type (numeric code).
    #[serde(default)]
    pub file_system_type: Option<FileSystemType>,

    /// Total size of the volume in bytes.
    pub size: u64,

    /// Free space remaining on the volume in bytes.
    pub size_remaining: u64,

    /// Type of drive (removable, fixed, etc.).
    pub drive_type: DriveType,

    /// Deduplication mode (Windows 10+).
    #[serde(default)]
    pub dedup_mode: Option<DedupMode>,
}

mod typing;
pub use typing::{DedupMode, DriveType, FileSystemType, HealthStatus};
