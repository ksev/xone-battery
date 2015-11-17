extern crate winapi;
extern crate xinput;
extern crate user32;

mod gamepads;

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use winapi::{MB_OK};
use user32::MessageBoxW;

use gamepads::BatteryType;

fn main() {
    let info: Vec<_> = 
        gamepads::battery()
                .into_iter()
                .filter(|bat| bat.kind != BatteryType::Disconnected)
                .collect();

    println!("{:?}", info);

    let mut title: Vec<_> = 
        OsStr::new("Hello").encode_wide()
                           .chain(Some(0).into_iter())
                           .collect();

    title.push(0u16);

    let ptr = title.as_mut_ptr();

    unsafe { MessageBoxW(std::ptr::null_mut(), ptr, ptr, MB_OK); }
}
