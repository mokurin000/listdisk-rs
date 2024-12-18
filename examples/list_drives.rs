use listdisk_rs::win32::logical_drives::get_logical_driveletters;

fn main() {
    let chars = get_logical_driveletters().collect::<String>();
    println!("{chars}");
}
