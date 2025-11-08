#[cfg(feature = "drive-info")]
#[cfg_attr(docsrs, doc(cfg(feature = "drive-info")))]
pub mod drive_info;

#[cfg(feature = "physical-disk")]
#[cfg_attr(docsrs, doc(cfg(feature = "physical-disk")))]
pub mod physical_disk;

#[cfg(feature = "partition")]
#[cfg_attr(docsrs, doc(cfg(feature = "partition")))]
pub mod partition;

#[cfg(feature = "volume_wmi")]
#[cfg_attr(docsrs, doc(cfg(feature = "volume_wmi")))]
pub mod volume_wmi;

#[cfg(feature = "freespace")]
#[cfg_attr(docsrs, doc(cfg(feature = "freespace")))]
pub mod freespace;

#[cfg(feature = "logical-drives")]
#[cfg_attr(docsrs, doc(cfg(feature = "logical-drives")))]
pub mod logical_drives;

#[cfg(feature = "volume")]
#[cfg_attr(docsrs, doc(cfg(feature = "volume")))]
/// list volumes
pub mod volume;

/// powershell-based helper functions
pub mod utils;
