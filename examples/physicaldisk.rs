use std::{collections::HashMap, env, error::Error};

use listdisk_rs::win32::{
    partition::{Partition, PartitionToVolume},
    physical_disk::PhysicalDisk,
    storagepool::{StoragePool, StoragePoolToVolume},
    volume_wmi::Volume,
};
use wmi::WMIConnection;

fn main() -> Result<(), Box<dyn Error>> {
    let wmi_storage = WMIConnection::with_namespace_path(r#"ROOT\Microsoft\Windows\Storage"#)?;

    let mut filter_map = HashMap::new();
    if let Some(verbatim_path) = env::args().nth(1) {
        filter_map.insert("Path".into(), wmi::FilterValue::String(verbatim_path));
    } else {
        let systemdrive = env::var("SystemDrive")
            .unwrap_or("C:".into())
            .chars()
            .next()
            .unwrap();
        filter_map.insert(
            "DriveLetter".into(),
            wmi::FilterValue::String(systemdrive.to_string()),
        );
    }

    let volume = wmi_storage
        .filtered_query::<Volume>(&filter_map)?
        .pop()
        .unwrap();
    let object_path = volume.obj_path;
    let partitions = wmi_storage.associators::<Partition, PartitionToVolume>(&object_path)?;

    if !partitions.is_empty() {
        println!("Found {} associated partitions!", partitions.len());

        let mut physical_disks = Vec::new();
        for partition in partitions {
            let mut filter_map = HashMap::new();
            filter_map.insert(
                "DeviceId".into(),
                wmi::FilterValue::String(partition.disk_number.to_string()),
            );
            let mut physical_disk = wmi_storage.filtered_query::<PhysicalDisk>(&filter_map)?;
            physical_disks.append(&mut physical_disk);
        }
        physical_disks.dedup_by_key(|PhysicalDisk { device_id, .. }| device_id.clone());

        println!("System drive info:\n{physical_disks:#?}");
    } else {
        let storage_pool =
            wmi_storage.associators::<StoragePool, StoragePoolToVolume>(&object_path)?;
        println!("System storage pool info:\n{storage_pool:#?}");
    }

    Ok(())
}
