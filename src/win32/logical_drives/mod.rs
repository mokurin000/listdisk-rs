use bitvec::{boxed::BitBox, order::Lsb0, view::BitView};
use windows_sys::Win32::Storage::FileSystem::GetLogicalDrives;

pub fn get_logical_driveletters() -> impl Iterator<Item = char> {
    let magic = unsafe { GetLogicalDrives() };
    let bitbox = BitBox::from_bitslice(magic.view_bits::<Lsb0>());
    bitbox
        .into_iter()
        .enumerate()
        .filter(|&(_, is_one)| is_one)
        .map(|(idx, _)| idx as u8 + b'A')
        .map(char::from)
}
