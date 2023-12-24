pub mod hardware;

use hardware::cartridge::Cartridge;

use crate::hardware::{bus::Bus, cpu::CPU};

fn main() {
    let bus = Bus::new(Cartridge::from_path("priv/10-bit ops.gb").unwrap());
    let mut cpu = CPU::new(bus);

    loop {
        cpu.execute_next_instruction();
    }
}
