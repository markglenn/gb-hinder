use std::fmt::Display;

use crate::hardware::{cpu::CPU, Memory};

#[derive(Debug, Clone, Copy)]
pub enum Target {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    MC,
    MBC,
    MDE,
    MHL,
    Immediate,
    ZeroImmediate,
    MImmediate,
}

impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Target::A => write!(f, "A"),
            Target::B => write!(f, "B"),
            Target::C => write!(f, "C"),
            Target::D => write!(f, "D"),
            Target::E => write!(f, "E"),
            Target::H => write!(f, "H"),
            Target::L => write!(f, "L"),
            Target::MC => write!(f, "(C)"),
            Target::MBC => write!(f, "(BC)"),
            Target::MDE => write!(f, "(DE)"),
            Target::MHL => write!(f, "(HL)"),
            Target::Immediate => write!(f, "Imm"),
            Target::MImmediate => write!(f, "(Imm)"),
            Target::ZeroImmediate => write!(f, "(0xFF00 + Imm)"),
        }
    }
}

impl Target {
    pub fn get_value(self, cpu: &mut CPU) -> u8 {
        match self {
            Target::A => cpu.registers.a,
            Target::B => cpu.registers.b,
            Target::C => cpu.registers.c,
            Target::D => cpu.registers.d,
            Target::E => cpu.registers.e,
            Target::H => cpu.registers.h,
            Target::L => cpu.registers.l,
            Target::MC => cpu.bus.read(0xFF00 + cpu.registers.c as u16),
            Target::MBC => cpu.bus.read(cpu.registers.bc()),
            Target::MDE => cpu.bus.read(cpu.registers.de()),
            Target::MHL => cpu.bus.read(cpu.registers.hl()),
            Target::Immediate => cpu.next_byte(),
            Target::ZeroImmediate => {
                let address = 0xFF00 + cpu.next_byte() as u16;
                cpu.bus.read(address)
            }
            Target::MImmediate => {
                let address = cpu.next_word();
                cpu.bus.read(address)
            }
        }
    }

    pub fn set_value(self, cpu: &mut CPU, value: u8) {
        match self {
            Target::A => cpu.registers.a = value,
            Target::B => cpu.registers.b = value,
            Target::C => cpu.registers.c = value,
            Target::D => cpu.registers.d = value,
            Target::E => cpu.registers.e = value,
            Target::H => cpu.registers.h = value,
            Target::L => cpu.registers.l = value,
            Target::MC => cpu.bus.write(0xFF00 + cpu.registers.c as u16, value),
            Target::MBC => cpu.bus.write(cpu.registers.bc(), value),
            Target::MDE => cpu.bus.write(cpu.registers.de(), value),
            Target::MHL => cpu.bus.write(cpu.registers.hl(), value),
            Target::ZeroImmediate => {
                let address = 0xFF00 + cpu.next_byte() as u16;
                cpu.bus.write(address, value);
            }
            Target::MImmediate => {
                let address = cpu.next_word();
                cpu.bus.write(address, value);
            }
            Target::Immediate => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Target16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    Immediate,
}

impl Target16 {
    pub fn get_value(self, cpu: &mut CPU) -> u16 {
        match self {
            Target16::AF => cpu.registers.af(),
            Target16::BC => cpu.registers.bc(),
            Target16::DE => cpu.registers.de(),
            Target16::HL => cpu.registers.hl(),
            Target16::SP => cpu.sp,
            Target16::Immediate => cpu.next_word(),
        }
    }

    pub fn set_value(self, cpu: &mut CPU, value: u16) {
        match self {
            Target16::AF => cpu.registers.set_af(value),
            Target16::BC => cpu.registers.set_bc(value),
            Target16::DE => cpu.registers.set_de(value),
            Target16::HL => cpu.registers.set_hl(value),
            Target16::SP => cpu.sp = value,
            Target16::Immediate => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Condition {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    None,
}

impl Condition {
    pub fn test(&self, cpu: &CPU) -> bool {
        match self {
            Condition::NotZero => !cpu.registers.f.zero(),
            Condition::Zero => cpu.registers.f.zero(),
            Condition::NotCarry => !cpu.registers.f.carry(),
            Condition::Carry => cpu.registers.f.carry(),
            Condition::None => true,
        }
    }
}
