use std::collections::HashMap;

use wmi::{COMLibrary, WMIConnection, WMIResult};

pub struct DriveInfo {
    wmi_conn: WMIConnection,
}

#[cfg(feature = "serde")]
#[derive(serde::Deserialize, Debug, PartialEq, Eq)]
#[serde(rename = "Win32_DiskDrive")]
#[serde(rename_all = "PascalCase")]
pub struct DiskDrive {
    pub device_id: String,
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
    pub capabilities: Vec<u16>,
    pub total_cylinders: u64,
    pub total_heads: u32,
    pub total_sectors: u64,
    pub total_tracks: u64,
    pub tracks_per_cylinder: u32,
    pub system_name: String,
    pub power_management_capabilities: Vec<u16>,

    pub availability: Option<u16>,
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
    pub status_info: Option<u16>,
}

impl DriveInfo {
    #[cfg(feature = "serde")]
    pub fn query_drive_info(&self) -> WMIResult<Vec<DiskDrive>> {
        self.wmi_conn.query()
    }

    pub fn query_drive_info_raw(&self) -> WMIResult<Vec<HashMap<String, wmi::Variant>>> {
        self.wmi_conn.raw_query("SELECT * FROM Win32_DiskDrive")
    }

    pub fn try_new() -> WMIResult<Self> {
        let com_con = COMLibrary::new()?;
        let wmi_conn = WMIConnection::new(com_con)?;
        Ok(Self { wmi_conn })
    }
}
