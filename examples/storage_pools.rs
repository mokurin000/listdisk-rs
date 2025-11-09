use std::error::Error;

use listdisk_rs::win32::storagepool::StoragePool;
use wmi::WMIConnection;

fn main() -> Result<(), Box<dyn Error>> {
    let wmi_storage = WMIConnection::with_namespace_path(r#"ROOT\Microsoft\Windows\Storage"#)?;

    let storage_pools = wmi_storage.query::<StoragePool>()?;
    println!("{storage_pools:#?}");
    Ok(())
}
