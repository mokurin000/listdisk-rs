use std::collections::HashMap;

use anyhow::Result;
use byte_unit::{AdjustedByte, Byte, Unit};
use listdisk_rs::win32::drive_info::{
    DiskDrive, DriveInfo, diskindex_by_driveletter, diskindex_by_win32_path,
};
use listdisk_rs::win32::freespace::FreeSpace;
use listdisk_rs::win32::logical_drives::get_logical_driveletters;
use listdisk_rs::win32::volume::Volume;

fn main() -> Result<()> {
    pretty_env_logger::init_timed();

    let chars = get_logical_driveletters().collect::<Vec<char>>();
    let mut disk_index_map = HashMap::new();
    for letter in chars {
        eprintln!("finding for {letter}:");
        let disk_index = diskindex_by_driveletter(letter)?;
        disk_index_map.insert(letter, disk_index);
    }

    let mut volume_index_map = HashMap::new();
    for volume in Volume::<64>::new() {
        eprintln!("finding for {volume}");
        match diskindex_by_win32_path(&volume) {
            Ok(disk_index) => {
                volume_index_map.insert(volume, disk_index);
            }
            Err(e) => {
                eprintln!("failed to read DiskIndex of {volume}: {e}");
                continue;
            }
        }
    }

    eprintln!("disk_index prepare done!");

    let drive_info = DriveInfo::try_new()?;
    let drivedisks = drive_info.query_drive_info()?;
    for DiskDrive {
        index,
        model,
        serial_number,
        ..
    } in drivedisks
    {
        println!("disk:");
        println!("    index: {index}");
        println!("    model: {model}");
        println!("    serial: {serial_number}");
        println!("    drive:");
        for (&letter, _) in disk_index_map
            .iter()
            .filter(|(_, idx)| **idx == index as usize)
        {
            println!("    - type: drive");
            println!("      name: {letter}");
            match FreeSpace::try_from_drive(letter) {
                Some(freespace) => {
                    let bytes_for_caller = human_size(freespace.bytes_for_caller);
                    let total_bytes = human_size(freespace.total_bytes);
                    println!("      space: {bytes_for_caller:.02}/{total_bytes:.02}");
                }
                None => (),
            };

            println!();
        }

        for (volume, _) in volume_index_map
            .iter()
            .filter(|(_, idx)| **idx == index as usize)
        {
            println!("    - type: volume");
            println!("      name: {volume}");

            match FreeSpace::try_from_ascii_path(&volume) {
                Some(freespace) => {
                    let bytes_for_caller = human_size(freespace.bytes_for_caller);
                    let total_bytes = human_size(freespace.total_bytes);
                    println!("      space: {bytes_for_caller:.02}/{total_bytes:.02}");
                }
                None => (),
            };

            println!();
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
