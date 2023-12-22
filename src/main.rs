pub mod hardware;

use hardware::cartridge::Cartridge;

use crate::hardware::{bus::Bus, cpu::CPU};

fn main() {
    //let bus = Bus::new(Cartridge::from_path("priv/06-ld r,r.gb").unwrap());
    let bus = Bus::new(Cartridge::from_path("priv/01-special.gb").unwrap());
    let mut cpu = CPU::new(bus);

    loop {
        cpu.execute_next_instruction();
    }
}
