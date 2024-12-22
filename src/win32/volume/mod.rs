use std::ptr;
use std::slice;

use utf16string::WStr;
use windows_sys::Win32::Foundation::{
    ERROR_NO_MORE_FILES, GetLastError, HANDLE, INVALID_HANDLE_VALUE,
};
use windows_sys::Win32::Storage::FileSystem::{FindFirstVolumeW, FindNextVolumeW, FindVolumeClose};

#[derive(Debug, Clone)]
pub struct Volume<const N: usize = 64> {
    handle: HANDLE,
    buf: [u16; N],
}

impl<const N: usize> Default for Volume<{ N }> {
    fn default() -> Self {
        let buffer = [0_u16; N];

        Self {
            handle: ptr::null_mut(),
            buf: buffer,
        }
    }
}

impl<const N: usize> Volume<{ N }> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<const N: usize> Iterator for Volume<{ N }> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let lpszvolumename = self.buf.as_mut().as_mut_ptr();
        let cchbufferlength = self.buf.len() as u32;

        if self.handle.is_null() {
            unsafe {
                let handle = FindFirstVolumeW(lpszvolumename, cchbufferlength);
                if handle == INVALID_HANDLE_VALUE {
                    log::error!("failed to find first volume");
                    return None;
                }
                self.handle = handle;
            }
        } else {
            self.buf = [0; N];
            unsafe {
                let successed = FindNextVolumeW(self.handle, lpszvolumename, cchbufferlength);

                // refer to https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-findnextvolumew#return-value
                if successed == 0 {
                    let error_code = GetLastError();
                    match error_code {
                        ERROR_NO_MORE_FILES => {
                            FindVolumeClose(self.handle);
                            self.handle = ptr::null_mut();
                        }
                        _ => {
                            log::error!("Failed to find next volume: OS {error_code}");
                        }
                    }

                    return None;
                }
            }
        }

        let data = self.buf.as_ptr() as *const u8;
        let len = self.buf.len() as u32 * (u16::BITS / u8::BITS);
        let bytes = unsafe { slice::from_raw_parts(data, len as usize) };

        let wstr = WStr::from_utf16le(bytes).ok()?;
        Some(
            wstr.to_utf8()
                .trim_end_matches(|ch| ch as u32 == 0)
                .to_string(),
        )
    }
}
