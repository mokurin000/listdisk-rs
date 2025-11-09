use serde::{Deserialize, Serialize};

mod typing;
pub use typing::{BusType, CannotPoolReason, HealthStatus, MediaType, Usage};

/// Represents a physical disk in the Windows Storage Management API.
///
/// ```rust
/// use wmi::WMIConnection;
/// use listdisk_rs::win32::physical_disk::PhysicalDisk;
///
/// let wmi_storage = WMIConnection::with_namespace_path(r#"ROOT\Microsoft\Windows\Storage"#)?;
///
/// let phy_drives = wmi_storage.query::<PhysicalDisk>()?;
/// Ok::<_, wmi::WMIError>(())
/// ```
///
/// Also see [MSDN](https://learn.microsoft.com/en-us/windows-hardware/drivers/storage/msft-physicaldisk)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "MSFT_PhysicalDisk")]
#[serde(rename_all = "PascalCase")]
pub struct PhysicalDisk {
    /// Object Path for associated queries, method call.
    #[serde(rename = "__Path")]
    pub obj_path: String,

    /// The identifier for the physical disk that is persistent across reboots.
    ///
    /// This is typically a number (e.g., "0", "1") assigned by the storage subsystem.
    pub device_id: String,

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

    /// Indicates the format of the UniqueId property.
    ///
    /// | Value | Meaning |
    /// |-------|---------|
    /// | 0     | Unknown |
    /// | 1     | Vendor-specific |
    /// | 2     | Serial number |
    /// | 3     | GUID |
    /// | 4     | NAA (IEEE Registered) |
    /// | 5     | EUI-64 |
    /// | 6     | T10 Vendor ID based |
    pub unique_id_format: u16,

    /// The type of bus to which the disk is connected.
    pub bus_type: BusType,

    /// The type of media in the disk.
    pub media_type: MediaType,

    /// Overall health status of the disk.
    pub health_status: HealthStatus,

    /// Current usage of the disk in storage pools.
    pub usage: Usage,

    /// Usages that this disk is capable of supporting.
    ///
    /// A disk may support multiple roles depending on configuration.
    pub supported_usages: Vec<Usage>,

    /// A user-friendly name for the disk (e.g., "Samsung SSD 970 EVO 1TB").
    pub friendly_name: String,

    /// Current operational state of the disk.
    ///
    /// From PowerShell: `Get-PhysicalDisk | Get-Member -Name OperationalStatus`
    ///
    /// | Code   | Meaning                            |
    /// |--------|------------------------------------|
    /// | 1      | Other                              |
    /// | 2      | OK                                 |
    /// | 3      | Degraded                           |
    /// | 4      | Stressed                           |
    /// | 5      | Predictive Failure                 |
    /// | 6      | Error                              |
    /// | 7      | Recoverable Error                  |
    /// | 8      | Starting                           |
    /// | 9      | Stopping                           |
    /// | 10     | Stopped                            |
    /// | 11     | In Service                         |
    /// | 12     | No Contact                         |
    /// | 13     | Lost Communication                 |
    /// | 14     | Aborted                            |
    /// | 15     | Dormant                            |
    /// | 16     | Supporting Entity in Error         |
    /// | 17     | Completed                          |
    /// | 18     | Power Mode                         |
    /// | 19     | Relocating                         |
    /// | 53252  | Failed Media                       |
    /// | 53253  | Split                              |
    /// | 53254  | Stale Metadata                     |
    /// | 53255  | IO Error                           |
    /// | 53256  | Unrecognized Metadata              |
    /// | 53269  | Removing From Pool                 |
    /// | 53270  | In Maintenance Mode                |
    /// | 53271  | Updating Firmware                  |
    /// | 53272  | Device Hardware Error              |
    /// | 53273  | Not Usable                         |
    /// | 53274  | Transient Error                    |
    /// | 53276  | Starting Maintenance Mode          |
    /// | 53277  | Stopping Maintenance Mode          |
    /// | 53285  | Threshold Exceeded                 |
    /// | 53286  | Abnormal Latency                   |
    /// | ..     | Unknown (future extensions)        |
    pub operational_status: Vec<u16>,

    /// Additional human-readable details about the current operational status.
    ///
    /// Corresponds one-to-one with `OperationalStatus` entries.
    pub operational_details: Vec<String>,

    /// Physical location of the disk within an enclosure (e.g., "Slot 3").
    ///
    /// May be null if not applicable or unknown.
    pub physical_location: Option<String>,

    /// Amount of space (in bytes) this disk contributes to virtual disks.
    ///
    /// This is typically the allocated size in storage spaces.
    pub virtual_disk_footprint: u16,

    /// Description of the disk provided by the manufacturer.
    pub description: Option<String>,

    /// Manufacturer's part number for the disk.
    pub part_number: Option<String>,

    /// Current firmware version running on the disk.
    pub firmware_version: Option<String>,

    /// Software version (e.g., driver or management agent), if applicable.
    pub software_version: Option<String>,

    /// Total size of the disk in bytes.
    pub size: u64,

    /// Amount of disk space currently allocated to storage pools or virtual disks.
    pub allocated_size: u64,

    // /// Indicates whether write caching is enabled on the disk.
    // ///
    // /// Note: This property is not exposed in all versions of MSFT_PhysicalDisk.
    // pub is_write_cache_enabled: Option<bool>,

    // /// Indicates whether the disk is protected from power loss (battery-backed).
    // /// Not always available.
    // pub is_power_protected: Option<bool>,
    /// Size of a physical sector (usually 4096 bytes for modern drives).
    ///
    /// Also known as "Advanced Format" or "4Kn" sector size.
    pub physical_sector_size: u64,

    /// Size of a logical sector (typically 512 or 4096 bytes).
    /// This is the emulated sector size seen by the OS.
    pub logical_sector_size: u64,

    /// Rotational speed in RPM for HDDs.
    /// `0xFFFFFFFF` (-1) if not applicable (e.g., SSDs) or unknown.
    pub spindle_speed: u32,

    /// Indicates whether LED indication (e.g., activity/fault lights) is enabled.
    pub is_indication_enabled: Option<bool>,

    /// Number of the enclosure containing the disk (0-based index).
    /// `null` if not in an enclosure or unknown.
    pub enclosure_number: Option<u16>,

    /// Slot number within the enclosure (0-based index).
    /// `null` if not applicable.
    pub slot_number: Option<u16>,

    /// Indicates whether the disk can be added to a storage pool.
    pub can_pool: bool,

    /// Reasons why the disk cannot be pooled.
    /// Use `CannotPoolReason::from(code)` to interpret.
    ///
    /// Common values:
    /// - 2: In a pool
    /// - 3: Unhealthy
    /// - 4: In use by an application
    /// - 7: Insufficient capacity
    /// - 8: Unsupported media type
    /// - 9: Removable media
    /// - 10: Offline
    /// - 11: Not supported by firmware
    /// - 12: In maintenance mode
    /// - etc.
    pub cannot_pool_reason: Vec<u16>,

    /// Additional description for "Other" or unknown `CannotPoolReason`.
    pub other_cannot_pool_reason_description: Option<String>,

    /// Indicates that only part of the physical disk is exposed (e.g., due to partitioning or RAID).
    /// When `true`, not all capacity is available for pooling.
    pub is_partial: Option<bool>,
}
