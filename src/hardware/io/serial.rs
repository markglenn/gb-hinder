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
            0xFF01 => 0, //self.data,
            0xFF02 => self.control,
            _ => 0x00,
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF01 => {
                self.data = value;
            }
            0xFF02 => {
                if value == 0x81 {
                    print!("{}", self.data as char);
                }
                self.control = value;
            }

            _ => unreachable!(),
        }
    }
}
