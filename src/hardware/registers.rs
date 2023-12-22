use std::fmt::Display;

use bitfield_struct::bitfield;

#[bitfield(u8)]
pub struct Flags {
    #[bits(4)]
    pub ignore: usize,

    pub carry: bool,
    pub half_carry: bool,
    pub subtract: bool,
    pub zero: bool,
}

impl Display for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            if self.zero() { "Z" } else { "-" },
            if self.subtract() { "N" } else { "-" },
            if self.half_carry() { "H" } else { "-" },
            if self.carry() { "C" } else { "-" },
        )
    }
}

#[derive(Debug)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: Flags,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    pub fn new() -> Registers {
        let mut f = Flags::new();
        f.set_ignore(0);
        f.set_zero(true);
        f.set_half_carry(true);
        f.set_carry(true);

        Registers {
            a: 0x01,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            f,
            h: 0x01,
            l: 0x4D,
        }
    }

    pub fn af(&self) -> u16 {
        let f: u8 = self.f.into();
        ((self.a as u16) << 8) | (f as u16)
    }

    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = ((value & 0x00FF) as u8).into();
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }
}

impl Display for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X}",
            self.a,
            u8::from(self.f),
            self.b,
            self.c,
            self.d,
            self.e,
            self.h,
            self.l,
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flags() {
        let mut flags = Flags::new();
        assert_eq!(flags.carry(), false);
        assert_eq!(flags.half_carry(), false);
        assert_eq!(flags.subtract(), false);
        assert_eq!(flags.zero(), false);

        assert_eq!(0b0000_0000u8, flags.into());

        flags.set_carry(true);
        flags.set_half_carry(true);
        flags.set_subtract(true);
        flags.set_zero(true);
        assert_eq!(flags.carry(), true);
        assert_eq!(flags.half_carry(), true);
        assert_eq!(flags.subtract(), true);
        assert_eq!(flags.zero(), true);
        assert_eq!(0b1111_0000u8, flags.into());

        flags.set_carry(false);
        flags.set_half_carry(true);
        flags.set_subtract(false);
        flags.set_zero(true);

        assert_eq!(0b1010_0000u8, flags.into());
    }
}
