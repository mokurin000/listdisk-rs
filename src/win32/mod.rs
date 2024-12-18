#[cfg(feature = "freespace")]
pub mod freespace;
#[cfg(feature = "logical-drives")]
pub mod logical_drives;
#[cfg(feature = "volume")]
pub mod volume;

#[cfg(feature = "encoding")]
pub use utf16string;
#[cfg(feature = "win32_sys")]
pub use windows_sys;
