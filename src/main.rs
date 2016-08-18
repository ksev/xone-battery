#![feature(conservative_impl_trait)]
extern crate uuid;
extern crate winapi;
extern crate xinput;
extern crate user32;
extern crate kernel32;
extern crate shell32;

mod win;
mod gamepads;

use uuid::Uuid;
use gamepads::{BatteryType, BatteryLevel};
use std::thread::sleep;
use std::time::Duration;
use win::ToWin;

const IDI_EMPTY: isize = 0x102;
const IDI_LOW: isize = 0x103;
const IDI_MEDIUM: isize = 0x104;
const IDI_FULL: isize = 0x105;
const IDI_NONE: isize = 0x106;

fn main() {
    let path = std::env::current_exe().expect("Could not get current path");
    let guid = Uuid::new_v5(&uuid::NAMESPACE_OID, path.to_str().unwrap()).to_win();

    let hwnd = win::initialize().expect("Could not initialize window");
    
    win::add_icon(hwnd, guid, IDI_NONE, "No controller connected");
    
    loop {
        let pad = gamepads::battery()
            .filter(|bat| bat.kind != BatteryType::Disconnected).nth(0);

        match pad {
            Some(info) => match info.level {
                BatteryLevel::Empty => win::change_icon(hwnd, guid, IDI_EMPTY, "Battery is empty"),
                BatteryLevel::Low => win::change_icon(hwnd, guid, IDI_LOW, "Battery is low"),
                BatteryLevel::Medium => win::change_icon(hwnd, guid, IDI_MEDIUM, "Battery is half full"),
                BatteryLevel::Full => win::change_icon(hwnd, guid, IDI_FULL, "Battery is full"),
            },
            None => win::change_icon(hwnd, guid, IDI_NONE, "No controller connected"),
        } ;

        sleep(Duration::from_secs(10));
    }
}
