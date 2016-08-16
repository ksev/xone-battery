#![feature(conservative_impl_trait)]
#[macro_use(DEFINE_GUID)]

extern crate winapi;
extern crate xinput;
extern crate user32;
extern crate kernel32;
extern crate shell32;

mod win;
mod gamepads;

use gamepads::{BatteryType, BatteryLevel};
use std::thread::sleep;
use std::time::Duration;

const IDI_EMPTY: isize = 0x102;
const IDI_LOW: isize = 0x103;
const IDI_MEDIUM: isize = 0x104;
const IDI_FULL: isize = 0x105;
const IDI_NONE: isize = 0x106;

fn main() {
    let hwnd = win::initialize().expect("Could not initialize window");
    win::add_icon(hwnd, IDI_NONE);
    
    loop {
        let pad = gamepads::battery()
            .filter(|bat| bat.kind != BatteryType::Disconnected).nth(0);

        match pad {
            Some(info) => match info.level {
                BatteryLevel::Empty => win::change_icon(hwnd, IDI_EMPTY),
                BatteryLevel::Low => win::change_icon(hwnd, IDI_LOW),
                BatteryLevel::Medium => win::change_icon(hwnd, IDI_MEDIUM),
                BatteryLevel::Full => win::change_icon(hwnd, IDI_FULL),
            },
            None => win::change_icon(hwnd, IDI_NONE),
        } ;

        sleep(Duration::from_secs(10));
    }
}