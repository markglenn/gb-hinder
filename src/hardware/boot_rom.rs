use super::Memory;

// The boot ROM is a 256 byte ROM that is mapped to the first 256 bytes of
// memory. It is disabled after the boot ROM has been executed.
// https://gbdev.gg8.se/wiki/articles/Gameboy_Bootstrap_ROM
pub struct BootROM {
    data: Vec<u8>,
}

impl BootROM {
    pub fn new() -> BootROM {
        let data = include_bytes!("../../boot/dmg_boot.bin").to_vec();

        BootROM { data }
    }
}

impl Memory for BootROM {
    fn read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    fn write(&mut self, _: u16, _: u8) {
        // This ROM is read only, so don't write to it
    }
}
