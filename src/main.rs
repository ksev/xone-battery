#![cfg_attr(not(test), windows_subsystem = "windows")]

mod win;
mod gamepads;

use std::time::Duration;
use std::error::Error;

use uuid::Uuid;
use gamepads::BatteryType;

const IDI_EMPTY: isize = 0x102;
const IDI_LOW: isize = 0x103;
const IDI_MEDIUM: isize = 0x104;
const IDI_FULL: isize = 0x105;
const IDI_NONE: isize = 0x106;

const NO_CONTROLLER: &[u16] = &[
    78, 111, 32, 99, 111, 110, 116, 114, 
    111, 108, 108, 101, 114, 32, 99, 111, 
    110, 110, 101, 99, 116, 101, 100, 0
];

const BATTERY_EMPTY: &[u16] = &[66, 97, 116, 116, 101, 114, 121, 32, 105, 115, 32, 101, 109, 112, 116, 121, 0];
const BATTERY_LOW: &[u16] = &[66, 97, 116, 116, 101, 114, 121, 32, 105, 115, 32, 108, 111, 119, 0];
const BATTERY_MEDIUM: &[u16] = &[66, 97, 116, 116, 101, 114, 121, 32, 105, 115, 32, 104, 97, 108, 102, 32, 102, 117, 108, 108, 0];
const BATTERY_FULL: &[u16] = &[66, 97, 116, 116, 101, 114, 121, 32, 105, 115, 32, 102, 117, 108, 108, 0];

fn main() -> Result<(), Box<Error>> {
    let path = std::env::current_exe()?;
    let guid = uuid_to_guid(Uuid::new_v5(&Uuid::NAMESPACE_OID, path.to_str().ok_or("Invalid str")?.as_bytes()));
    let hwnd = win::initialize().ok_or("Window initialization failed")?;

    win::add_icon(hwnd, guid, IDI_NONE, NO_CONTROLLER);

    loop {
        let pad = gamepads::battery()
            .filter(|bat| bat.kind != BatteryType::Disconnected)
            .nth(0);

        match pad {
            Some(info) => {
                use gamepads::BatteryLevel::*;
                
                match info.level {
                    Empty => win::change_icon(hwnd, guid, IDI_EMPTY, BATTERY_EMPTY),
                    Low => win::change_icon(hwnd, guid, IDI_LOW, BATTERY_LOW),
                    Medium => win::change_icon(hwnd, guid, IDI_MEDIUM, BATTERY_MEDIUM),
                    Full => win::change_icon(hwnd, guid, IDI_FULL, BATTERY_FULL),
                }
            }
            None => win::change_icon(hwnd, guid, IDI_NONE, NO_CONTROLLER),
        };

        std::thread::sleep(Duration::from_secs(5));
    }
}

fn uuid_to_guid(uuid: Uuid) -> winapi::shared::guiddef::GUID {
    use winapi::shared::guiddef::GUID;

    let bytes = uuid.as_bytes();
    let end = [
        bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]
    ];

    GUID {
        Data1: ((bytes[0] as u32) << 24 | (bytes[1] as u32) << 16 | (bytes[2] as u32) << 8
            | (bytes[3] as u32)),
        Data2: ((bytes[4] as u16) << 8 | (bytes[5] as u16)),
        Data3: ((bytes[6] as u16) << 8 | (bytes[7] as u16)),
        Data4: end,
    }
}