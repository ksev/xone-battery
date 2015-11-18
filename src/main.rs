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

    

    let mut title = encode_wide("Hello");

    let m = format!("{:?}", info);
    let mut content: Vec<_> = encode_wide(&m);

    unsafe { MessageBoxW(std::ptr::null_mut(), content.as_mut_ptr(), title.as_mut_ptr(), MB_OK); }
}

fn encode_wide<'a>(input: &'a str) -> Vec<u16> {
    OsStr::new(input).encode_wide()
                     .chain(std::iter::once(0))
                     .collect()
}
