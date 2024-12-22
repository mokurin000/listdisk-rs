use std::collections::HashMap;

use anyhow::Result;
use byte_unit::{AdjustedByte, Byte, Unit};
use listdisk_rs::win32::drive_info::{DiskDrive, DriveInfo, diskindex_by_driveletter};
use listdisk_rs::win32::freespace::FreeSpace;
use listdisk_rs::win32::logical_drives::get_logical_driveletters;

fn main() -> Result<()> {
    let chars = get_logical_driveletters().collect::<Vec<char>>();
    let mut disk_index_map = HashMap::new();
    for letter in chars {
        let disk_index = diskindex_by_driveletter(letter)?;
        disk_index_map.insert(disk_index, letter);
    }

    let drive_info = DriveInfo::try_new()?;
    let drivedisks = drive_info.query_drive_info()?;
    for DiskDrive {
        index,
        model,
        serial_number,
        ..
    } in drivedisks
    {
        if let Some(&letter) = disk_index_map.get(&(index as usize)) {
            let freespace = FreeSpace::try_from_drive(letter).unwrap();
            let bytes_for_caller = human_size(freespace.bytes_for_caller);
            let total_bytes = human_size(freespace.total_bytes);

            println!("Found parition {letter}:");
            println!(" - space: {bytes_for_caller:.02}/{total_bytes:.02}");
            println!("On disk:");
            println!(" - model: {model}");
            println!(" - serial: {serial_number}");
        }
    }

    Ok(())
}

fn human_size(bytes: u64) -> AdjustedByte {
    let units = [
        Unit::B,
        Unit::KiB,
        Unit::MiB,
        Unit::GiB,
        Unit::TiB,
        Unit::PiB,
        Unit::EiB,
    ];

    for (idx, &unit) in units.iter().enumerate() {
        let dived = bytes >> (idx * 10);
        if dived < 1024 {
            let byte = Byte::from_u64(bytes);
            return byte.get_adjusted_unit(unit);
        }
    }

    unreachable!()
}
