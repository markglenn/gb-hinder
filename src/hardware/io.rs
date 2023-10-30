use super::Memory;

pub struct IO {}

impl Memory for IO {
    fn read(&self, address: u16) -> u8 {
        match address {
            _ => 0x00,
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF00 => println!("JOYP: 0x{:02X}", value),

            // GPU
            0xFF40..=0xFF4B => {}

            // Audio
            0xFF10..=0xFF26 => {}
            _ => panic!("Invalid address: 0x{:04X}", address),
        }
    }
}
