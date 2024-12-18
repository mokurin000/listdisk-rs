use std::{ffi::CString, str::FromStr};

use windows_sys::Win32::{
    Foundation::GetLastError,
    Storage::FileSystem::{GetDiskFreeSpaceExA, GetDiskFreeSpaceExW},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FreeSpace {
    pub bytes_for_caller: u64,
    pub total_bytes: u64,
    pub total_free_bytes: u64,
}

impl FreeSpace {
    pub fn from_drive(drive_letter: char) -> Option<Self> {
        if !drive_letter.is_ascii_alphabetic() {
            return None;
        }
        let drive_letter = drive_letter.to_ascii_uppercase();

        let path = format!("{drive_letter}:/");
        unsafe { Self::from_ascii_path(path) }
    }

    pub unsafe fn from_ascii_path(path: impl AsRef<str>) -> Option<Self> {
        let dirpath = CString::from_str(path.as_ref()).ok()?;

        unsafe { freespace_from_dirpath(dirpath.as_ptr() as _) }
    }

    #[cfg(feature = "encoding")]
    pub fn from_path(path: impl AsRef<std::path::Path>) -> Option<Self> {
        let wstring =
            utf16string::WString::<utf16string::LE>::from(path.as_ref().to_string_lossy().as_ref());
        let dirpath = wstring.as_ptr() as _;
        unsafe { freespace_from_dirpath_unicode(dirpath) }
    }
}

pub unsafe fn freespace_from_dirpath(dirpath: *const u8) -> Option<FreeSpace> {
    let mut bytes_for_caller = 0;
    let mut total_bytes = 0;
    let mut total_free_bytes = 0;

    let result = unsafe {
        GetDiskFreeSpaceExA(
            dirpath,
            &mut bytes_for_caller,
            &mut total_bytes,
            &mut total_free_bytes,
        )
    };

    if result == 0 {
        let error = unsafe { GetLastError() };
        log::error!("OS Error ({error})");
        return None;
    }

    Some(FreeSpace {
        bytes_for_caller,
        total_bytes,
        total_free_bytes,
    })
}

pub unsafe fn freespace_from_dirpath_unicode(dirpath: *const u16) -> Option<FreeSpace> {
    let mut bytes_for_caller = 0;
    let mut total_bytes = 0;
    let mut total_free_bytes = 0;

    let result = unsafe {
        GetDiskFreeSpaceExW(
            dirpath,
            &mut bytes_for_caller,
            &mut total_bytes,
            &mut total_free_bytes,
        )
    };

    if result == 0 {
        let error = unsafe { GetLastError() };
        log::error!("OS Error ({error})");
        return None;
    }

    Some(FreeSpace {
        bytes_for_caller,
        total_bytes,
        total_free_bytes,
    })
}
