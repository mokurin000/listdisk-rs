use serde::{Deserialize, Serialize};

#[cfg(feature = "serde_repr")]
mod typing;
#[cfg(feature = "serde_repr")]
pub use typing::{BusType, Usage, HealthStatus, MediaType};


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
#[cfg(feature = "serde")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "MSFT_PhysicalDisk")]
#[serde(rename_all = "PascalCase")]
pub struct PhysicalDisk {
    pub device_id: String,
    pub unique_id_format: u16,

    #[cfg(not(feature = "serde_repr"))]
    pub bus_type: u16,
    #[cfg(not(feature = "serde_repr"))]
    pub media_type: u16,
    #[cfg(not(feature = "serde_repr"))]
    pub health_status: u16,
    #[cfg(not(feature = "serde_repr"))]
    pub usage: u16,
    #[cfg(not(feature = "serde_repr"))]
    pub supported_usages: Vec<u16>,

    #[cfg(feature = "serde_repr")]
    pub bus_type: BusType,
    #[cfg(feature = "serde_repr")]
    pub media_type: MediaType,
    #[cfg(feature = "serde_repr")]
    pub health_status: HealthStatus,
    #[cfg(feature = "serde_repr")]
    pub usage: Usage,
    #[cfg(feature = "serde_repr")]
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
    pub cannot_pool_reason: Vec<u16>,
    pub other_cannot_pool_reason_description: Option<String>,
    pub is_partial: bool,
}
