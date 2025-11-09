use std::collections::HashMap;

use wmi::{FilterValue, WMIConnection, WMIError, WMIResult};

use crate::win32::partition::Partition;

/// - wmi_conn: WMI connection to `ROOT\Microsoft\Windows\Storage` namespace
/// - verbatim_path: \\?\Volume{...}
pub fn diskindex_by_volume_path(
    wmi_storage: &WMIConnection,
    verbatim_path: &String,
) -> WMIResult<u32> {
    wmi_storage
        .query::<Partition>()?
        .into_iter()
        .filter(|p| p.access_paths.contains(verbatim_path))
        .next()
        .map(|p| p.disk_number)
        .ok_or_else(|| WMIError::ResultEmpty)
}

/// - wmi_conn: WMI connection to `ROOT\Microsoft\Windows\Storage` namespace
pub fn diskindex_by_driveletter(wmi_storage: &WMIConnection, drive_letter: char) -> WMIResult<u32> {
    Ok(wmi_storage
        .filtered_query::<Partition>(&HashMap::from([(
            "DriveLetter".to_string(),
            FilterValue::String(drive_letter.to_string()),
        )]))?
        .first()
        .ok_or_else(|| WMIError::ResultEmpty)?
        .disk_number)
}
