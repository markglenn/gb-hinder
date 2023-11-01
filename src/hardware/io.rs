pub mod serial;
pub mod timer;

use super::Memory;

pub struct IO {}

use bitfield_struct::bitfield;

#[bitfield(u8)]
pub struct Flags {
    #[bits(4)]
    _ignore: usize,

    pub carry: bool,
    pub half_carry: bool,
    pub subtract: bool,
    pub zero: bool,
}

#[bitfield(u8)]
pub struct Interrupts {
    #[bits(3)]
    _ignore: usize,

    vblank: bool,
    lcd: bool,
    timer: bool,
    serial: bool,
    joypad: bool,
}

impl Interrupts {
    pub fn status(&self) -> u8 {
        // TODO: Get interrupt statuses
        0
    }

    pub fn set_status(&self, _value: u8) {
        // TODO: Set interrupt statuses
    }
}

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
