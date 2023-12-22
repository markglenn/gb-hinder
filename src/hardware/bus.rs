use super::io::serial::Serial;
use super::io::timer::Timer;
use super::io::Interrupts;
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
    timer: Timer,
    interrupts: Interrupts,
    serial: Serial,
}

impl Bus {
    pub fn new(cartridge: Cartridge) -> Bus {
        let mbc = MBC1::new(cartridge);

        Bus {
            mbc,
            internal_ram: RAM::new(0x2000),
            boot_rom: None, //Some(BootROM::new()),
            vram: RAM::new(0x2000),
            zero_page: RAM::new(0x7F),
            io: IO {},
            timer: Timer {},
            interrupts: Interrupts::new(),
            serial: Serial::new(),
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

            // Object Attribute Memory (OAM)
            0xFE00..=0xFE9F => 0, //self.io.write(address, value),

            // Serial transfer
            0xFF01..=0xFF02 => self.serial.read(address),

            // Timer
            0xFF04..=0xFF07 => self.timer.read(address),

            // IO Ports
            0xFF00..=0xFF7F => self.io.read(address),

            // Zero Page
            0xFF80..=0xFFFE => self.zero_page.read(address - 0xFF80),

            // Interrupt enabled register
            0xFFFF => self.interrupts.into(),

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

            // Object Attribute Memory (OAM)
            0xFE00..=0xFE9F => {} //self.io.write(address, value),

            // Disable boot ROM when writing to this I/O address
            0xFF50 => self.boot_rom = None,

            // Serial transfer
            0xFF01..=0xFF02 => self.serial.write(address, value),

            // Timer
            0xFF04..=0xFF07 => self.timer.write(address, value),

            // Interrupt status
            0xFF0F => self.interrupts.set_status(value),

            // IO Ports
            0xFF00..=0xFF7F => self.io.write(address, value),

            // Zero Page
            0xFF80..=0xFFFE => self.zero_page.write(address - 0xFF80, value),

            // Interrupt enabled register
            0xFFFF => self.interrupts = value.into(),

            _ => panic!("Attempted to write to invalid address: 0x{:04X}", address),
        }
    }
}
