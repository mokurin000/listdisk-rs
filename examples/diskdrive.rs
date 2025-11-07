use std::collections::HashMap;

use listdisk_rs::win32::drive_info::DiskDrive;
use wmi::WMIConnection;

fn main() -> anyhow::Result<()> {
    let wmi_conn = WMIConnection::new()?;

    let disk_drives = wmi_conn.query::<DiskDrive>()?;
    for drive in disk_drives {
        println!("{drive:?}");
    }

    let disk_drives_raw:  Vec<HashMap<String, wmi::Variant>> = wmi_conn.raw_query("SELECT * FROM Win32_DiskDrive")?;
    for drive in disk_drives_raw {
        println!("{drive:?}");
    }

    Ok(())
}
