mod typing;
pub use typing::{Availability, Capability, PowerManagementCapability, StatusInfo};

/// Represents a physical disk drive as seen by Windows (WMI class: `Win32_DiskDrive`).
///
/// This class is in the `root\cimv2` namespace and is derived from `CIM_DiskDrive`.
///
/// ```rust
/// use wmi::WMIConnection;
/// use listdisk_rs::win32::drive_info::DiskDrive;
///
/// let wmi_conn = WMIConnection::new()?;
/// let disk_drives = wmi_conn.query::<DiskDrive>()?;
/// Ok::<_, wmi::WMIError>(())
/// ```
///
/// See: [Win32_DiskDrive - MSDN](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-diskdrive)
#[derive(serde::Deserialize, Debug, PartialEq, Eq)]
#[serde(rename = "Win32_DiskDrive")]
#[serde(rename_all = "PascalCase")]
pub struct DiskDrive {
    /// Object Path for associated queries, method call.
    #[serde(rename = "__Path")]
    pub obj_path: String,

    /// Unique identifier of the disk drive with other devices on the system.
    /// Example: `\\.\PHYSICALDRIVE0`
    pub device_id: String,

    /// Physical drive number (0-based index) assigned by the system.
    ///
    /// Obtained from `IOCTL_STORAGE_GET_DEVICE_NUMBER`.  
    /// `0xFFFFFFFF` means the drive does **not map** to a physical disk (e.g., virtual or RAM disk).
    ///
    /// Example: `0`, `1`
    pub index: u32,

    /// Manufacturer's model number of the disk drive.
    ///
    /// Example: `"ST1000LM035-1RK172"`
    pub model: String,

    /// Label by which the object is known (often same as `DeviceID` or model).
    ///
    /// Example: `"\\.\PHYSICALDRIVE0"`
    pub name: String,

    /// Serial number allocated by the manufacturer to identify the physical media.
    ///
    /// Retrieved via `IOCTL_STORAGE_QUERY_PROPERTY` with `StorageDeviceProperty`.
    ///
    /// Example: `"WD-WMC6N0K3E9F8"`
    pub serial_number: String,

    /// Interface type of the physical disk drive.
    ///
    /// Common values:
    /// - `"SCSI"`
    /// - `"IDE"`
    /// - `"USB"`
    /// - `"1394"` (FireWire)
    /// - `"NVMe"` (may appear as `"SCSI"` on some systems)
    pub interface_type: String,

    /// Current status of the object.
    ///
    /// Possible values:
    /// - `"OK"`
    /// - `"Error"`
    /// - `"Degraded"`
    /// - `"Unknown"`
    /// - `"Pred Fail"` (SMART prediction)
    /// - `"Starting"`, `"Stopping"`, `"Service"`, `"Stressed"`, `"NonRecover"`, `"No Contact"`, `"Lost Comm"`
    pub status: String,

    /// Windows Plug and Play device identifier.
    ///
    /// Example: `"PCI\VEN_8086&DEV_2829..."` or `"USBSTOR\DISK&VEN_WD..."`.
    pub pnp_device_id: String,

    /// Short description of the object (usually same as `Model`).
    ///
    /// Max length: 64 characters.
    pub caption: String,

    /// If `true`, media is loaded and accessible (has a readable file system).
    ///
    /// Always `true` for fixed disk drives. May be `false` for removable media.
    pub media_loaded: bool,

    /// Type of media used or accessed by this device.
    ///
    /// Examples:
    /// - `"Fixed hard disk media"`
    /// - `"Removable media other than floppy"`
    /// - `"External hard disk media"`
    /// - `"Format is unknown"`
    pub media_type: String,

    /// Number of bytes in each sector.
    ///
    /// Typically `512` (legacy) or `4096` (Advanced Format).
    pub bytes_per_sector: u32,

    /// Total number of cylinders on the disk.
    ///
    /// > **Note**: Value comes from BIOS INT 13h extended functions.  
    /// > May be **inaccurate** if drive uses translation (LBA). Use `Size` instead for accurate capacity.
    pub total_cylinders: u64,

    /// Total number of heads (read/write heads).
    ///
    /// > **Note**: May be inaccurate due to translation schemes.
    pub total_heads: u32,

    /// Total number of sectors on the disk.
    ///
    /// > **Note**: May be inaccurate due to translation. Prefer `Size`.
    pub total_sectors: u64,

    /// Total number of tracks on the disk.
    ///
    /// > **Note**: May be inaccurate.
    pub total_tracks: u64,

    /// Number of tracks per cylinder.
    ///
    /// > **Note**: May be inaccurate due to translation.
    pub tracks_per_cylinder: u32,

    /// Name of the scoping system (computer name).
    pub system_name: String,

    /// Human-readable descriptions of capabilities listed in `capabilities`.
    ///
    /// Each entry corresponds to the same index in `capabilities`.
    pub capability_descriptions: Vec<String>,

    /// Compression algorithm used by the device.
    ///
    /// Values:
    /// - `null` → Unknown
    /// - `"Compressed"` → Supports compression (scheme unknown)
    /// - `"Not Compressed"` → No compression
    pub compression_method: Option<String>,

    /// Windows Configuration Manager error code.
    ///
    /// `0` = Device is working properly.  
    /// See full list in docs for codes 1–31.
    pub config_manager_error_code: Option<u32>,

    /// If `true`, the device uses a user-defined configuration.
    pub config_manager_user_config: Option<bool>,

    /// Default block size in bytes for this device.
    pub default_block_size: Option<u64>,

    /// Description of the object (often same as `Caption`).
    pub description: Option<String>,

    /// If `true`, the error in `last_error_code` has been cleared.
    pub error_cleared: Option<bool>,

    /// More information about the error in `last_error_code`.
    pub error_description: Option<String>,

    /// Type of error detection and correction supported (e.g., ECC, CRC).
    pub error_methodology: Option<String>,

    /// Firmware revision assigned by the manufacturer.
    ///
    /// Retrieved via `STORAGE_DEVICE_DESCRIPTOR.ProductRevisionOffset`.
    pub firmware_revision: Option<String>,

    /// Date and time the object was installed.
    ///
    /// Does **not** require a value to indicate installation.
    pub install_date: Option<wmi::WMIDateTime>,

    /// Last error code reported by the logical device.
    pub last_error_code: Option<u32>,

    /// Name of the disk drive manufacturer.
    ///
    /// Example: `"Western Digital"`, `"Seagate"`, `"Samsung"`
    pub manufacturer: Option<String>,

    /// Maximum block size in bytes for media accessed by this device.
    pub max_block_size: Option<u64>,

    /// Maximum media size in **kilobytes** supported by this device.
    pub max_media_size: Option<u64>,

    /// Minimum block size in bytes for media accessed by this device.
    pub min_block_size: Option<u64>,

    /// If `true`, the media access device needs cleaning.
    ///
    /// Cleaning capability indicated in `capabilities`.
    pub needs_cleaning: Option<bool>,

    /// Maximum number of individual media supported (e.g., for multi-disc changers).
    ///
    /// Usually `1` for hard drives.
    pub number_of_media_supported: Option<u32>,

    /// Number of partitions recognized by the OS on this drive.
    pub partitions: Option<u32>,

    /// If `true`, the device **can** be power-managed (e.g., suspend).
    ///
    /// Does **not** mean power management is currently enabled.
    pub power_management_supported: Option<bool>,

    /// SCSI bus number of the disk drive.
    pub scsi_bus: Option<u32>,

    /// SCSI Logical Unit Number (LUN).
    pub scsi_logical_unit: Option<u16>,

    /// SCSI port number.
    pub scsi_port: Option<u16>,

    /// SCSI target ID.
    pub scsi_target_id: Option<u16>,

    /// Number of sectors per track.
    ///
    /// > **Note**: May be inaccurate due to translation.
    pub sectors_per_track: Option<u32>,

    /// Disk identification tag (NTFS MBR signature or GPT identifier).
    ///
    /// Used to identify shared resources.
    pub signature: Option<u32>,

    /// Total size of the disk drive in **bytes**.
    ///
    /// Calculated as:  
    /// `TotalCylinders × TracksPerCylinder × SectorsPerTrack × BytesPerSector`  
    ///
    /// **This is the most reliable size field.**
    pub size: Option<u64>,

    /// Availability and status of the device.
    ///
    /// See `Availability` enum:
    /// - `RunningFullPower(3)`
    /// - `PowerSaveLowPowerMode(14)`
    /// - etc.
    pub availability: Option<Availability>,

    /// State of the logical device.
    ///
    /// See `StatusInfo`:
    /// - `Enabled(3)`
    /// - `Disabled(4)`
    /// - `NotApplicable(5)`
    pub status_info: Option<StatusInfo>,

    /// Array of device capabilities.
    ///
    /// See `Capability` enum:
    /// - `RandomAccess(3)`
    /// - `SupportsWriting(4)`
    /// - `SupportsRemovableMedia(7)`
    /// - `SmartNotification(10)`
    /// - etc.
    pub capabilities: Vec<Capability>,

    /// Specific power-related capabilities.
    ///
    /// See `PowerManagementCapability`:
    /// - `NotSupported(1)`
    /// - `Enabled(3)`
    /// - `PowerSavingModesEnteredAutomatically(4)`
    /// - `PowerStateSettable(5)`
    /// - etc.
    pub power_management_capabilities: Vec<PowerManagementCapability>,
}
