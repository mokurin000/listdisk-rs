#[cfg(feature = "serde")]
mod typing;
#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub use typing::{Availability, Capability, PowerManagementCapability, StatusInfo};

/// ```rust
/// use wmi::WMIConnection;
/// use listdisk_rs::win32::drive_info::DiskDrive;
///
/// let wmi_conn = WMIConnection::new()?;
/// let disk_drives = wmi_conn.query::<DiskDrive>()?;
/// Ok::<_, wmi::WMIError>(())
/// ```
#[cfg(feature = "serde")]
#[derive(serde::Deserialize, Debug, PartialEq, Eq)]
#[serde(rename = "Win32_DiskDrive")]
#[serde(rename_all = "PascalCase")]
pub struct DiskDrive {
    pub device_id: String,
    /// diskIndex
    pub index: u32,
    pub model: String,
    pub name: String,
    pub serial_number: String,
    pub interface_type: String,
    pub status: String,
    pub pnp_device_id: String,
    pub caption: String,
    pub media_loaded: bool,
    pub media_type: String,
    pub bytes_per_sector: u32,
    pub total_cylinders: u64,
    pub total_heads: u32,
    pub total_sectors: u64,
    pub total_tracks: u64,
    pub tracks_per_cylinder: u32,
    pub system_name: String,
    pub capability_descriptions: Vec<String>,
    pub compression_method: Option<String>,
    pub config_manager_error_code: Option<u32>,
    pub config_manager_user_config: Option<bool>,
    pub default_block_size: Option<u64>,
    pub description: Option<String>,
    pub error_cleared: Option<bool>,
    pub error_description: Option<String>,
    pub error_methodology: Option<String>,
    pub firmware_revision: Option<String>,
    #[cfg(feature = "wmi")]
    pub install_date: Option<wmi::WMIDateTime>,
    pub last_error_code: Option<u32>,
    pub manufacturer: Option<String>,
    pub max_block_size: Option<u64>,
    pub max_media_size: Option<u64>,
    pub min_block_size: Option<u64>,
    pub needs_cleaning: Option<bool>,
    pub number_of_media_supported: Option<u32>,
    pub partitions: Option<u32>,
    pub power_management_supported: Option<bool>,
    pub scsi_bus: Option<u32>,
    pub scsi_logical_unit: Option<u16>,
    pub scsi_port: Option<u16>,
    pub scsi_target_id: Option<u16>,
    pub sectors_per_track: Option<u32>,
    pub signature: Option<u32>,
    pub size: Option<u64>,

    #[serde(default)]
    pub availability: Option<Availability>,
    #[serde(default)]
    pub status_info: Option<StatusInfo>,
    #[serde(default)]
    pub capabilities: Vec<Capability>,
    #[serde(default)]
    pub power_management_capabilities: Vec<PowerManagementCapability>,
}
