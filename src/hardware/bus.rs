use super::{boot_rom::BootROM, cartridge::Cartridge, io::IO, mbc::MBC1, Memory, RAM};

// The gameboy does not necessarily have a bus, but a bus is a close
// representative of what it does have.
pub struct Bus {
    mbc: MBC1,
    internal_ram: RAM,
    vram: RAM,
    zero_page: RAM,
    boot_rom: Option<BootROM>,
    io: IO,
}

impl Bus {
    pub fn new(cartridge: Cartridge) -> Bus {
        let internal_ram = RAM::new(0x2000);
        let mbc = MBC1::new(cartridge);
        let vram = RAM::new(0x2000);
        let io = IO {};
        let zero_page = RAM::new(0x7F);

        Bus {
            mbc,
            internal_ram,
            boot_rom: Some(BootROM::new()),
            vram,
            zero_page,
            io,
        }
    }
}

impl Memory for Bus {
    fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x100 => {
                if let Some(boot_rom) = &self.boot_rom {
                    boot_rom.read(address)
                } else {
                    self.mbc.read(address)
                }
            }
            // Cartridge ROM
            0x0000..=0x7FFF => self.mbc.read(address),

            // Video RAM
            0x8000..=0x9FFF => self.vram.read(address - 0x8000),

            // Internal RAM
            0xC000..=0xDFFF => self.internal_ram.read(address - 0xC000),

            // Echo RAM
            0xE000..=0xFDFF => self.internal_ram.read(address - 0xE000),

            // IO Ports
            0xFF00..=0xFF7F => self.io.read(address),

            // Zero Page
            0xFF80..=0xFFFE => self.zero_page.read(address - 0xFF80),

            _ => panic!("Attempted to read from invalid address: 0x{:04X}", address),
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            // Cartridge ROM
            0x0000..=0x7FFF => self.mbc.write(address, value),

            // Video RAM
            0x8000..=0x9FFF => self.vram.write(address - 0x8000, value),

            // Internal RAM
            0xC000..=0xDFFF => self.internal_ram.write(address - 0xC000, value),

            // Echo RAM
            0xE000..=0xFDFF => self.internal_ram.write(address - 0xE000, value),

            // Disable boot ROM when writing to this I/O address
            0xFF50 => self.boot_rom = None,

            // IO Ports
            0xFF00..=0xFF7F => self.io.write(address, value),

            // Zero Page
            0xFF80..=0xFFFE => self.zero_page.write(address - 0xFF80, value),

            _ => panic!("Attempted to write to invalid address: 0x{:04X}", address),
        }
    }
}
