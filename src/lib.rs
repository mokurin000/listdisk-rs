//! Before using listdisk-rs, you could read these resources:
//!
//! - [Namespace root/cimv2](https://powershell.one/wmi/root/cimv2)
//! - [An In-Depth Guide to Windows File Paths](https://chrisdenton.github.io/omnipath/Overview.html)
//! - [Storage Management API Classes](https://learn.microsoft.com/en-us/windows-hardware/drivers/storage/storage-management-api-classes)
#![cfg_attr(docsrs, feature(doc_cfg))]

/// Everything related to Disk/Partition/FileSystem
#[cfg(target_os = "windows")]
pub mod win32;

#[cfg(feature = "serde")]
pub use serde;
#[cfg(feature = "encoding")]
pub use utf16string;
#[cfg(feature = "win32_sys")]
pub use windows_sys;
#[cfg(feature = "wmi")]
pub use wmi;
