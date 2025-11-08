use std::error::Error;

use listdisk_rs::win32::partition::Partition;
use wmi::WMIConnection;

fn main() -> Result<(), Box<dyn Error>> {
    let wmi_storage = WMIConnection::with_namespace_path(r#"ROOT\Microsoft\Windows\Storage"#)?;

    let partitions = wmi_storage.query::<Partition>()?;
    println!("{partitions:#?}");
    Ok(())
}
