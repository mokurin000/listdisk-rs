use listdisk_rs::win32::drive_info::DriveInfo;

fn main() -> anyhow::Result<()> {
    let drive_info = DriveInfo::try_new()?;

    let disk_drives = drive_info.query_drive_info()?;
    for drive in disk_drives {
        println!("{drive:?}");
    }

    let disk_drives_raw = drive_info.query_drive_info_raw()?;
    for drive in disk_drives_raw {
        println!("{drive:?}");
    }

    Ok(())
}
