#![cfg_attr(not(test), windows_subsystem = "windows")]

mod win;
mod gamepads;

use std::time::Duration;
use std::error::Error;

use uuid::Uuid;
use gamepads::BatteryType;

use wchar::wch_c;

const IDI_EMPTY: isize = 0x102;
const IDI_LOW: isize = 0x103;
const IDI_MEDIUM: isize = 0x104;
const IDI_FULL: isize = 0x105;
const IDI_NONE: isize = 0x106;

const NO_CONTROLLER: &[u16] = wch_c!("No controller connected");

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
                    Empty => win::change_icon(hwnd, guid, IDI_EMPTY, wch_c!("Battery is empty")),
                    Low => win::change_icon(hwnd, guid, IDI_LOW, wch_c!("Battery is low")),
                    Medium => win::change_icon(hwnd, guid, IDI_MEDIUM, wch_c!("Battery is half-full")),
                    Full => win::change_icon(hwnd, guid, IDI_FULL, wch_c!("Battery is full")),
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
        Data1: (u32::from(bytes[0]) << 24 | u32::from(bytes[1]) << 16 | u32::from(bytes[2]) << 8
            | u32::from(bytes[3])),
        Data2: (u16::from(bytes[4]) << 8 | u16::from(bytes[5])),
        Data3: (u16::from(bytes[6]) << 8 | u16::from(bytes[7])),
        Data4: end,
    }
}