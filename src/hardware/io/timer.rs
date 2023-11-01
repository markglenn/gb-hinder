use crate::hardware::Memory;

pub struct Timer {}

impl Memory for Timer {
    fn read(&self, address: u16) -> u8 {
        match address {
            _ => 0x00,
        }
    }

    fn write(&mut self, address: u16, _value: u8) {
        match address {
            0xFF04 => (),
            0xFF05 => (),
            0xFF06 => (),
            0xFF07 => (),
            _ => unreachable!(),
        }
    }
}
