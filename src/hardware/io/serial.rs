use crate::hardware::Memory;

pub struct Serial {
    pub data: u8,
    pub control: u8,
}

impl Serial {
    pub fn new() -> Serial {
        Serial {
            data: 0x00,
            control: 0x00,
        }
    }
}

impl Memory for Serial {
    fn read(&self, address: u16) -> u8 {
        match address {
            0xFF01 => self.data,
            0xFF02 => self.control,
            _ => 0x00,
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF01 => {
                println!("SB: 0x{:02X}", value);
                self.data = value;
            }
            0xFF02 => {
                println!("SC: 0x{:02X}", self.data);
                self.control = value;
            }

            _ => unreachable!(),
        }
    }
}
