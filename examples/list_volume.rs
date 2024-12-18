use anyhow::Result;
use listdisk_rs::win32::volume;

fn main() -> Result<()> {
    pretty_env_logger::init_timed();

    let volumes = volume::Volume::<64>::new();
    let volumes = volumes.collect::<Vec<_>>();
    println!("Found volumes:");
    for volume in volumes {
        println!(" - {volume}");
    }

    Ok(())
}
