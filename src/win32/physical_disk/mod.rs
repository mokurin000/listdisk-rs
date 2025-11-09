use serde::{Deserialize, Serialize};

mod typing;
pub use typing::{BusType, CannotPoolReason, HealthStatus, MediaType, Usage};

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
    pub device_id: String,
    pub unique_id_format: u16,

    pub bus_type: BusType,
    pub media_type: MediaType,
    pub health_status: HealthStatus,
    pub usage: Usage,
    pub supported_usages: Vec<Usage>,

    pub friendly_name: String,
    /// From powershell cmdlet:
    ///
    /// ```powershell
    /// Get-PhysicalDisk | Get-Member -Name OperationalStatus | fl
    /// ```
    ///
    /// | Opcode | Reason                     |
    /// | ------ | -------------------------- |
    /// | 1      | Other                      |
    /// | 2      | OK                         |
    /// | 3      | Degraded                   |
    /// | 4      | Stressed                   |
    /// | 5      | Predictive Failure         |
    /// | 6      | Error                      |
    /// | 7      | Recoverable Error          |
    /// | 8      | Starting                   |
    /// | 9      | Stopping                   |
    /// | 10     | Stopped                    |
    /// | 11     | In Service                 |
    /// | 12     | No Contact                 |
    /// | 13     | Lost Communication         |
    /// | 14     | Aborted                    |
    /// | 15     | Dormant                    |
    /// | 16     | Supporting Entity in Error |
    /// | 17     | Completed                  |
    /// | 18     | Power Mode                 |
    /// | 19     | Relocating                 |
    /// | 53252  | Failed Media               |
    /// | 53253  | Split                      |
    /// | 53254  | Stale Metadata             |
    /// | 53255  | IO Error                   |
    /// | 53256  | Unrecognized Metadata      |
    /// | 53269  | Removing From Pool         |
    /// | 53270  | In Maintenance Mode        |
    /// | 53271  | Updating Firmware          |
    /// | 53272  | Device Hardware Error      |
    /// | 53273  | Not Usable                 |
    /// | 53274  | Transient Error            |
    /// | 53276  | Starting Maintenance Mode  |
    /// | 53277  | Stopping Maintenance Mode  |
    /// | 53285  | Threshold Exceeded         |
    /// | 53286  | Abnormal Latency           |
    /// | ..     | Unknown                    |
    pub operational_status: Vec<u16>,
    pub operational_details: Vec<String>,
    pub physical_location: Option<String>,
    pub virtual_disk_footprint: u16,
    pub description: Option<String>,
    pub part_number: Option<String>,
    pub firmware_version: Option<String>,
    pub software_version: Option<String>,
    pub size: u64,
    pub allocated_size: u64,
    // pub is_write_cache_enabled: Option<bool>,
    // pub is_power_protected: Option<bool>,
    pub physical_sector_size: u64,
    pub logical_sector_size: u64,
    pub spindle_speed: u32,
    pub is_indication_enabled: bool,
    pub enclosure_number: Option<u16>,
    pub slot_number: Option<u16>,
    pub can_pool: bool,
    /// For typed reason, use CannotPoolReason::from.
    pub cannot_pool_reason: Vec<u16>,
    pub other_cannot_pool_reason_description: Option<String>,
    pub is_partial: bool,
}
