pub mod boot_rom;
pub mod bus;
pub mod cartridge;
pub mod cpu;
pub mod io;
pub mod mbc;
pub mod opcode;
pub mod registers;

pub trait Memory {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);

    fn read_word(&self, address: u16) -> u16 {
        let low = self.read(address) as u16;
        let high = self.read(address + 1) as u16;

        (high << 8) | low
    }

    fn write_word(&mut self, address: u16, value: u16) {
        let low = value as u8;
        let high = (value >> 8) as u8;

        self.write(address, low);
        self.write(address + 1, high);
    }
}

pub struct RAM {
    data: Vec<u8>,
}

impl RAM {
    pub fn new(size: usize) -> RAM {
        RAM {
            data: vec![0; size],
        }
    }
}

impl Memory for RAM {
    fn read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    fn write(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }
}
