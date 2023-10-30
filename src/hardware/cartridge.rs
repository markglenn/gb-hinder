use anyhow::Result;
use std::fs;

// https://gbdev.gg8.se/wiki/articles/The_Cartridge_Header

pub struct Cartridge {
    /// Cartridge ROM
    pub rom: Vec<u8>,
}

#[derive(Debug)]
pub enum CartridgeType {
    ROMOnly,
    MBC1,
}

impl From<u8> for CartridgeType {
    fn from(value: u8) -> Self {
        match value {
            0x00 => CartridgeType::ROMOnly,
            0x01 => CartridgeType::MBC1,
            _ => panic!("Unknown cartridge type: {}", value),
        }
    }
}

#[derive(Debug)]
pub enum Destination {
    Japanese,
    NonJapanese,
}

impl From<u8> for Destination {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Destination::Japanese,
            0x01 => Destination::NonJapanese,
            _ => panic!("Unknown destination: {}", value),
        }
    }
}

impl Cartridge {
    pub fn from_path(path: &str) -> Result<Cartridge> {
        let rom = fs::read(path)?;

        Ok(Cartridge { rom })
    }

    /// Returns the name of the cartridge
    pub fn name(&self) -> String {
        let mut name = String::new();

        for byte in &self.rom[0x0134..=0x0143] {
            // The name can be terminated by a 0x00 byte
            if *byte == 0x00 {
                break;
            }

            name.push(*byte as char);
        }

        name
    }

    pub fn cartridge_type(&self) -> CartridgeType {
        self.rom[0x0147].into()
    }

    pub fn rom_banks(&self) -> usize {
        match self.rom[0x0148] {
            0x00 => 2,
            0x01 => 4,
            0x02 => 8,
            0x03 => 16,
            0x04 => 32,
            0x05 => 64,
            0x06 => 128,
            0x07 => 256,
            0x08 => 512,
            0x52 => 72,
            0x53 => 80,
            0x54 => 96,
            _ => panic!("Unknown ROM size: {}", self.rom[0x0148]),
        }
    }

    pub fn ram_banks(&self) -> (usize, usize) {
        match self.rom[0x0149] {
            0x00 => (0, 0),
            0x01 => (1, 2 * 1024),
            0x02 => (1, 8 * 1024),
            0x03 => (4, 8 * 1024),
            0x04 => (16, 8 * 1024),
            0x05 => (8, 8 * 1024),
            _ => panic!("Unknown RAM size: {}", self.rom[0x0149]),
        }
    }

    pub fn destination(&self) -> Destination {
        self.rom[0x014A].into()
    }

    pub fn validate_header(&self) -> bool {
        let mut x: u8 = 0;

        for byte in &self.rom[0x0134..=0x014C] {
            x = x.wrapping_sub(*byte).wrapping_sub(1);
        }

        x == self.rom[0x014D]
    }
}
