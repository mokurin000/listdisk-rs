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
