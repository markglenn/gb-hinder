use super::{cartridge::Cartridge, Memory};

enum BankMode {
    ROM,
    RAM,
}

impl From<u8> for BankMode {
    fn from(value: u8) -> Self {
        match value & 1 {
            0x00 => BankMode::ROM,
            0x01 => BankMode::RAM,
            _ => unreachable!(),
        }
    }
}

pub struct MBC1 {
    cartridge: Cartridge,
    ram_enabled: bool,
    rom_bank: u8,
    ram_bank: u8,
    bank_mode: BankMode,
}

impl MBC1 {
    pub fn new(cartridge: Cartridge) -> MBC1 {
        MBC1 {
            cartridge,
            ram_enabled: false,
            rom_bank: 1,
            ram_bank: 1,
            bank_mode: BankMode::ROM,
        }
    }

    pub fn set_rom_bank(&mut self, bank: u8) {
        self.rom_bank = bank;
    }
}

impl Memory for MBC1 {
    fn read(&self, address: u16) -> u8 {
        match address {
            // Cartridge ROM
            0x0000..=0x3FFF => self.cartridge.rom[address as usize],

            // Cartridge ROM
            0x4000..=0x7FFF => {
                let bank = self.rom_bank as usize;
                let offset = (address - 0x4000) as usize;
                self.cartridge.rom[(bank * 0x4000) + offset]
            }

            // Cartridge RAM
            0xA000..=0xBFFF => unimplemented!(),
            _ => panic!("Invalid address: 0x{:04X}", address),
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            // Cartridge RAM Enable
            0x0000..=0x1FFF => self.ram_enabled = (value & 0x0F) == 0x0A,

            // ROM Bank Number
            0x2000..=0x3FFF => {
                // Set bits 0-4 of the ROM bank number
                let current_bank = self.rom_bank & !0x1F;

                // Writing a 0 to this register will actually set the bank to 1
                let value = if value == 0x00 { 0x01 } else { value };

                println!("ROM BANK: {}", current_bank | (value & 0x1F));
                self.set_rom_bank(current_bank | (value & 0x1F));
            }

            // RAM Bank Number - or - Upper Bits of ROM Bank Number
            0x4000..=0x5FFF => {
                match self.bank_mode {
                    BankMode::ROM => {
                        // Get the current bank not including bits 5 and 6
                        let current_bank = self.rom_bank & !0x60;
                        self.set_rom_bank(current_bank | ((value << 5) & 0x60));
                    }
                    BankMode::RAM => {
                        self.ram_bank = value & 0x03;
                    }
                }
            }

            // ROM/RAM Mode Select
            0x6000..=0x7FFF => self.bank_mode = value.into(),

            // Cartridge RAM
            0xA000..=0xBFFF => unimplemented!(),

            _ => panic!("Invalid address: 0x{:04X}", address),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_byte_ram_enabled() {
        let cartridge = Cartridge::from_path("priv/cpu_instrs.gb").unwrap();
        let mut mbc = MBC1::new(cartridge);
        mbc.write(0x0000, 0x0A);
        assert_eq!(mbc.ram_enabled, true);

        mbc.write(0x0000, 0x00);
        assert_eq!(mbc.ram_enabled, false);
    }

    #[test]
    fn test_write_byte_rom_bank_number() {
        let cartridge = Cartridge::from_path("priv/cpu_instrs.gb").unwrap();
        let mut mbc = MBC1::new(cartridge);
        mbc.write(0x2000, 0x01);
        assert_eq!(mbc.rom_bank, 1);

        mbc.write(0x2000, 0x1F);
        assert_eq!(mbc.rom_bank as usize, 3);

        mbc.write(0x2000, 0x20);
        assert_eq!(mbc.rom_bank as usize, 1);

        mbc.write(0x2000, 0x00);
        assert_eq!(mbc.rom_bank as usize, 1);
    }
}
