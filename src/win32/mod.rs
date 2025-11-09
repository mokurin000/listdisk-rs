/// WMI type for Win32_DiskDrive
#[cfg(feature = "drive-info")]
#[cfg_attr(docsrs, doc(cfg(feature = "drive-info")))]
pub mod drive_info;

/// WMI type for MSFT_PhysicalDisk
#[cfg(feature = "physical-disk")]
#[cfg_attr(docsrs, doc(cfg(feature = "physical-disk")))]
pub mod physical_disk;

/// WMI type for MSFT_Partition
#[cfg(feature = "partition")]
#[cfg_attr(docsrs, doc(cfg(feature = "partition")))]
pub mod partition;

/// WMI type for MSFT_Volume
#[cfg(feature = "volume_wmi")]
#[cfg_attr(docsrs, doc(cfg(feature = "volume_wmi")))]
pub mod volume_wmi;

/// user quota, disk free space, e.g.
#[cfg(feature = "freespace")]
#[cfg_attr(docsrs, doc(cfg(feature = "freespace")))]
pub mod freespace;

/// list used logical drive letters, like C:, D:, ...
#[cfg(feature = "logical-drives")]
#[cfg_attr(docsrs, doc(cfg(feature = "logical-drives")))]
pub mod logical_drives;

/// list volumes
#[cfg(feature = "volume")]
#[cfg_attr(docsrs, doc(cfg(feature = "volume")))]
pub mod volume;

/// powershell-based helper functions
#[cfg(feature = "utils")]
#[cfg_attr(docsrs, doc(cfg(feature = "utils")))]
pub mod utils;
