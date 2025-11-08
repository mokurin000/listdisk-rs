use std::{env, error::Error};

use listdisk_rs::win32::{physical_disk::PhysicalDisk, utils::diskindex_by_driveletter};
use wmi::WMIConnection;

fn main() -> Result<(), Box<dyn Error>> {
    let wmi_storage = WMIConnection::with_namespace_path(r#"ROOT\Microsoft\Windows\Storage"#)?;

    let physical_disks = wmi_storage.query::<PhysicalDisk>()?;

    let systemdrive = env::var("SystemDrive").unwrap_or("C:".into());
    let diskindex = diskindex_by_driveletter(systemdrive.chars().next().unwrap())?.to_string();

    let physical_system_drive = physical_disks
        .into_iter()
        .find(|d| d.device_id == diskindex)
        .expect("cannot find matched device");

    println!("System drive info:\n{physical_system_drive:#?}");

    Ok(())
}
