use serde::{Deserialize, Serialize};

/// Represents a partition on a disk as exposed by the `MSFT_Partition` WMI class.
/// Namespace: `Root\Microsoft\Windows\Storage`
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(rename = "MSFT_Partition")]
#[serde(rename_all = "PascalCase")]
pub struct Partition {
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

    /// The OS-assigned disk number that contains this partition.
    pub disk_number: u32,

    /// The OS-assigned partition number (based on offset order).
    pub partition_number: u32,

    /// Currently assigned drive letter (e.g., `'C'`), or `None` if unassigned.
    #[serde(default)]
    pub drive_letter: Option<char>,

    /// All access paths: drive letters, mount points, volume GUID paths.
    #[serde(default)]
    pub access_paths: Vec<String>,

    /// Current operational status of the partition.
    pub operational_status: typing::OperationalStatus,

    /// Current transition/resizing state.
    pub transition_state: typing::TransitionState,

    /// Total size of the partition in bytes.
    pub size: u64,

    /// MBR partition type (only valid for MBR disks).
    #[serde(default)]
    pub mbr_type: Option<typing::MbrType>,

    /// GPT partition type GUID string (only valid for GPT disks).
    #[serde(default)]
    pub gpt_type: Option<String>,

    /// GPT GUID of the partition (only valid for GPT disks).
    #[serde(default)]
    pub guid: Option<String>,

    pub is_read_only: Option<bool>,
    pub is_offline: Option<bool>,
    pub is_system: Option<bool>,
    pub is_boot: Option<bool>,
    pub is_active: Option<bool>,
    pub is_hidden: Option<bool>,
    pub is_shadow_copy: Option<bool>,
    pub no_default_drive_letter: Option<bool>,
}

mod typing;
pub use typing::{MbrType, OperationalStatus, TransitionState};
