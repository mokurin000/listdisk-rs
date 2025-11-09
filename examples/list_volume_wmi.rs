use std::collections::HashMap;
use std::error::Error;

use listdisk_rs::win32::partition::{Partition, PartitionToVolume};
use listdisk_rs::win32::physical_disk::PhysicalDisk;
use listdisk_rs::win32::volume_wmi::Volume;
use wmi::WMIConnection;

fn main() -> Result<(), Box<dyn Error>> {
    let wmi_storage = WMIConnection::with_namespace_path(r#"ROOT\Microsoft\Windows\Storage"#)?;

    let volumes = wmi_storage.query::<Volume>()?;

    for volume in volumes {
        println!("#######################################");
        println!("{volume:#?}");
        let partitions =
            wmi_storage.associators::<Partition, PartitionToVolume>(&volume.obj_path)?;

        for partition in partitions {
            println!("---------");
            println!("{:#?}", partition);

            let mut filter_map = HashMap::new();
            filter_map.insert(
                "DeviceId".into(),
                wmi::FilterValue::String(partition.disk_number.to_string()),
            );
            let physical_disks = wmi_storage.filtered_query::<PhysicalDisk>(&filter_map)?;
            let Some(physical_disk) = physical_disks.first() else {
                continue;
            };

            println!("---> {physical_disk:#?}");
        }
    }
    println!("#######################################");

    Ok(())
}
