use serde::{Deserialize, Serialize};

mod typing;
pub use typing::{BusType, HealthStatus, MediaType, Usage};

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
    /// An array of values specifying the reasons why this physical disk cannot be added to a concrete pool. This property is valid only if the CanPool property is FALSE.
    ///
    /// Unknown (0)
    ///
    /// Other (1)
    ///
    /// In a Pool (2)
    ///
    /// Not Healthy (3)
    ///
    /// Removable Media (4)
    ///
    /// In Use by Cluster (5)
    ///
    /// Offline (6)
    ///
    /// Insufficient Capacity (7)
    ///
    /// Spare Disk (8)
    ///
    /// Reserved by subsystem (9)
    ///
    /// Starting (10)
    ///
    /// Microsoft Reserved (..)
    ///
    /// Vendor Reserved (0x8000..)
    pub cannot_pool_reason: Vec<u16>,
    pub other_cannot_pool_reason_description: Option<String>,
    pub is_partial: bool,
}
