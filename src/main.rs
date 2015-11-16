extern crate winapi;
extern crate xinput;

mod gamepads;

use gamepads::BatteryType;

fn main() {
    let info: Vec<_> = 
        gamepads::battery()
                .into_iter()
                .filter(|bat| bat.kind != BatteryType::Disconnected)
                .collect();

    println!("{:?}", info);
}

