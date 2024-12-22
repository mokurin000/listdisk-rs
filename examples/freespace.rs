use anyhow::Result;
use listdisk_rs::win32::freespace::FreeSpace;

fn main() -> Result<()> {
    let system_drive = std::env::var("SystemDrive")?;
    let free_space = FreeSpace::try_from_ascii_path(&system_drive);
    println!("{free_space:?}");

    let path = format!("{system_drive}/");
    let free_space = FreeSpace::try_from_path(path);
    println!("{free_space:?}");

    Ok(())
}
