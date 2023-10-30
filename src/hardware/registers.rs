use bitfield_struct::bitfield;

#[bitfield(u8)]
pub struct Flags {
    #[bits(4)]
    _ignore: usize,

    pub carry: bool,
    pub half_carry: bool,
    pub subtract: bool,
    pub zero: bool,
}

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
        Registers {
            a: 0x00,
            b: 0x00,
            c: 0x00,
            d: 0x00,
            e: 0x00,
            f: Flags::new(),
            h: 0x00,
            l: 0x00,
        }
    }

    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
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
