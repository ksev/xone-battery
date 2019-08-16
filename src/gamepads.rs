use std;

use winapi::shared::winerror::SUCCEEDED;
use winapi::um::xinput::{XInputGetBatteryInformation, BATTERY_DEVTYPE_GAMEPAD, BATTERY_LEVEL_FULL,
                         BATTERY_LEVEL_LOW, BATTERY_LEVEL_MEDIUM, BATTERY_TYPE_ALKALINE,
                         BATTERY_TYPE_DISCONNECTED, BATTERY_TYPE_NIMH, BATTERY_TYPE_WIRED,
                         XINPUT_BATTERY_INFORMATION, XUSER_MAX_COUNT};

#[derive(Copy, Debug, Clone, Eq, PartialEq)]
pub enum BatteryType {
    Disconnected,
    Wired,
    Alkaline,
    Nmh,
    Unknown,
}

#[derive(Copy, Debug, Clone, Eq, PartialEq)]
pub enum BatteryLevel {
    Empty,
    Low,
    Medium,
    Full,
}

#[derive(Copy, Debug, Clone)]
pub struct BatteryInfo {
    pub id: u32,
    pub kind: BatteryType,
    pub level: BatteryLevel,
}

impl BatteryInfo {
    pub fn new(id: u32, battery: BatteryType, level: BatteryLevel) -> BatteryInfo {
        BatteryInfo {
            id,
            level,
            kind: battery
        }
    }
}

pub fn battery() -> impl Iterator<Item = BatteryInfo> {
    (0..XUSER_MAX_COUNT).filter_map(|user_index| unsafe {
        let mut xinfo: XINPUT_BATTERY_INFORMATION = std::mem::zeroed();
        let err = XInputGetBatteryInformation(
            user_index,
            BATTERY_DEVTYPE_GAMEPAD,
            &mut xinfo as *mut XINPUT_BATTERY_INFORMATION,
        );

        if SUCCEEDED(err as i32) {
            let typ = match xinfo.BatteryType {
                BATTERY_TYPE_DISCONNECTED => BatteryType::Disconnected,
                BATTERY_TYPE_WIRED => BatteryType::Wired,
                BATTERY_TYPE_NIMH => BatteryType::Nmh,
                BATTERY_TYPE_ALKALINE => BatteryType::Alkaline,
                _ => BatteryType::Unknown,
            };

            let lvl = match xinfo.BatteryLevel {
                BATTERY_LEVEL_LOW => BatteryLevel::Low,
                BATTERY_LEVEL_MEDIUM => BatteryLevel::Medium,
                BATTERY_LEVEL_FULL => BatteryLevel::Full,
                _ => BatteryLevel::Empty,
            };

            Some(BatteryInfo::new(user_index, typ, lvl))
        } else {
            None
        }
    })
}
