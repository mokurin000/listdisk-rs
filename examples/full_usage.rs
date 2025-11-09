use std::collections::HashMap;

use anyhow::Result;
use byte_unit::{AdjustedByte, Byte, Unit};
use listdisk_rs::win32::freespace::FreeSpace;
use listdisk_rs::win32::logical_drives::get_logical_driveletters;
use listdisk_rs::win32::volume::Volume;
use listdisk_rs::win32::{drive_info::DiskDrive, partition::Partition};
use wmi::{FilterValue, WMIConnection, WMIError, WMIResult};

fn main() -> Result<()> {
    pretty_env_logger::init_timed();
    let wmi_storage = WMIConnection::with_namespace_path("ROOT\\Microsoft\\Windows\\Storage")?;

    let chars = get_logical_driveletters().collect::<Vec<char>>();
    let disk_index_map: HashMap<_, _> = chars
        .iter()
        .filter_map(|&letter| {
            eprintln!("finding for {letter}:");
            diskindex_by_driveletter(&wmi_storage, letter)
                .ok()
                .map(|index| (letter, index))
        })
        .collect();

    let mut volume_index_map = HashMap::new();
    for volume in Volume::<64>::new() {
        eprintln!("finding for {volume}");
        match diskindex_by_volume_path(&wmi_storage, &volume) {
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

    let wmi_conn = WMIConnection::new()?;
    let drivedisks = wmi_conn.query::<DiskDrive>()?;
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
        for (&letter, _) in disk_index_map.iter().filter(|&(_, idx)| idx == &index) {
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

        for (volume, _) in volume_index_map.iter().filter(|&(_, idx)| idx == &index) {
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

fn diskindex_by_volume_path(wmi_conn: &WMIConnection, volume_path: &String) -> WMIResult<u32> {
    wmi_conn
        .query::<Partition>()?
        .into_iter()
        .filter(|p| p.access_paths.contains(volume_path))
        .next()
        .map(|p| p.disk_number)
        .ok_or_else(|| WMIError::ResultEmpty)
}

fn diskindex_by_driveletter(wmi_conn: &WMIConnection, letter: char) -> WMIResult<u32> {
    Ok(wmi_conn
        .filtered_query::<Partition>(&HashMap::from([(
            "DriveLetter".to_string(),
            FilterValue::String(letter.to_string()),
        )]))?
        .first()
        .ok_or_else(|| WMIError::ResultEmpty)?
        .disk_number)
}
