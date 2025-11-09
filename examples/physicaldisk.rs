use std::{collections::HashMap, env, error::Error};

use listdisk_rs::win32::{partition::Partition, physical_disk::PhysicalDisk};
use wmi::WMIConnection;

fn main() -> Result<(), Box<dyn Error>> {
    let wmi_storage = WMIConnection::with_namespace_path(r#"ROOT\Microsoft\Windows\Storage"#)?;

    let systemdrive = env::var("SystemDrive")
        .unwrap_or("C:".into())
        .chars()
        .next()
        .unwrap();
    let mut filter_map = HashMap::new();
    filter_map.insert(
        "DriveLetter".into(),
        wmi::FilterValue::String(systemdrive.to_string()),
    );
    let partition = wmi_storage
        .filtered_query::<Partition>(&filter_map)?
        .pop()
        .unwrap();

    let mut filter_map = HashMap::new();
    filter_map.insert(
        "DeviceId".into(),
        wmi::FilterValue::String(partition.disk_number.to_string()),
    );
    let physical_disks = wmi_storage.filtered_query::<PhysicalDisk>(&filter_map)?;

    println!("System drive info:\n{physical_disks:#?}");

    Ok(())
}
