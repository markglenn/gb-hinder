mod bits;
mod call;
mod cp;
mod inc;
mod jr;
mod ld;
mod stack;
mod xor;

use self::{
    bits::{prefix_cb, rl},
    call::{call, ret},
    cp::cp,
    jr::{jp, jr},
    ld::{ld, ld16, ldd, ldh, ldi},
    stack::{pop, push},
    xor::xor,
};

use super::{cpu::CPU, Memory};

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
    MDE,
    MHL,
    Immediate,
    ZeroImmediate,
    MImmediate,
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
    BC,
    DE,
    HL,
    SP,
    Immediate,
}

impl Target16 {
    pub fn get_value(self, cpu: &mut CPU) -> u16 {
        match self {
            Target16::BC => cpu.registers.bc(),
            Target16::DE => cpu.registers.de(),
            Target16::HL => cpu.registers.hl(),
            Target16::SP => cpu.sp,
            Target16::Immediate => cpu.next_word(),
        }
    }

    pub fn set_value(self, cpu: &mut CPU, value: u16) {
        match self {
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
#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    NOP,
    CALL,
    RET,
    LD(Target, Target),
    LD16(Target16, Target16),
    LDD(Target, Target),
    LDH(Target, Target),
    LDI(Target, Target),
    ADC(Target),
    XOR(Target),
    INC(Target),
    INC16(Target16),
    DEC(Target),
    PUSH(Target16),
    POP(Target16),
    RL(Target),
    JR(Condition),
    JP(Condition, Target16),
    PrefixCB,
    CP(Target),
    Undefined,
}

impl Opcode {
    pub fn from_byte(byte: u8) -> &'static Self {
        &OPCODES[byte as usize]
    }
}

pub fn execute_opcode(cpu: &mut CPU, opcode: &Opcode) -> u8 {
    match opcode {
        Opcode::NOP => nop(cpu),

        Opcode::ADC(target) => {
            let value = target.get_value(cpu);
            adc(cpu, value);
        }

        Opcode::LD16(target, from) => ld16(cpu, target, from),
        Opcode::LDD(target, from) => ldd(cpu, target, from),
        Opcode::LDH(target, from) => ldh(cpu, target, from),
        Opcode::LDI(target, from) => ldi(cpu, target, from),

        Opcode::XOR(target) => xor(cpu, target),

        Opcode::PrefixCB => prefix_cb(cpu),

        Opcode::JP(condition, target) => jp(cpu, condition, target),
        Opcode::JR(condition) => jr(cpu, condition),
        Opcode::LD(target, from) => ld(cpu, target, from),
        Opcode::INC(target) => inc::inc(cpu, target),
        Opcode::INC16(target) => inc::inc16(cpu, target),
        Opcode::DEC(target) => inc::dec(cpu, target),
        Opcode::CALL => call(cpu),
        Opcode::RET => ret(cpu),
        Opcode::PUSH(target) => push(cpu, target),
        Opcode::POP(target) => pop(cpu, target),
        Opcode::RL(target) => rl(cpu, target),
        Opcode::CP(target) => cp(cpu, target),
        Opcode::Undefined => panic!("Attempted to execute undefined opcode"),
    }

    1
}

pub static OPCODES: [Opcode; 0x100] = [
    // 0x00 - 0x0F
    Opcode::NOP,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::INC(Target::B),
    Opcode::DEC(Target::B),
    Opcode::LD(Target::B, Target::Immediate),
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::INC(Target::C),
    Opcode::DEC(Target::C),
    Opcode::LD(Target::C, Target::Immediate),
    Opcode::Undefined,
    // 0x10 - 0x1F
    Opcode::Undefined,
    Opcode::LD16(Target16::DE, Target16::Immediate),
    Opcode::Undefined,
    Opcode::INC16(Target16::DE),
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::RL(Target::A),
    Opcode::JR(Condition::None),
    Opcode::Undefined,
    Opcode::LD(Target::A, Target::MDE),
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::LD(Target::E, Target::Immediate),
    Opcode::Undefined,
    // 0x20 - 0x2F
    Opcode::JR(Condition::NotZero),
    Opcode::LD16(Target16::HL, Target16::Immediate),
    Opcode::LDI(Target::MHL, Target::A),
    Opcode::INC(Target::MHL),
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::JR(Condition::Zero),
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::LD(Target::L, Target::Immediate),
    Opcode::Undefined,
    // 0x30 - 0x3F
    Opcode::Undefined,
    Opcode::LD16(Target16::SP, Target16::Immediate),
    Opcode::LDD(Target::MHL, Target::A),
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::LDD(Target::A, Target::MHL),
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::DEC(Target::A),
    Opcode::LD(Target::A, Target::Immediate),
    Opcode::Undefined,
    // 0x40 - 0x4F
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::LD(Target::C, Target::A),
    // 0x50 - 0x5F
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::LD(Target::D, Target::A),
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    // 0x60 - 0x6F
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::LD(Target::H, Target::A),
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    // 0x70 - 0x7F
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::LD(Target::MHL, Target::A),
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::LD(Target::A, Target::E),
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    // 0x80 - 0x8F
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    // 0x90 - 0x9F
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    // 0xA0 - 0xAF
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::XOR(Target::A),
    // 0xB0 - 0xBF
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::CP(Target::B),
    Opcode::CP(Target::C),
    Opcode::CP(Target::D),
    Opcode::CP(Target::E),
    Opcode::CP(Target::H),
    Opcode::CP(Target::L),
    Opcode::CP(Target::MHL),
    Opcode::CP(Target::A),
    // 0xC0 - 0xCF
    Opcode::Undefined,
    Opcode::POP(Target16::BC),
    Opcode::Undefined,
    Opcode::JP(Condition::NotZero, Target16::Immediate),
    Opcode::Undefined,
    Opcode::PUSH(Target16::BC),
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::RET,
    Opcode::Undefined,
    Opcode::PrefixCB,
    Opcode::Undefined,
    Opcode::CALL,
    Opcode::Undefined,
    Opcode::Undefined,
    // 0xD0 - 0xDF
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    // 0xE0 - 0xEF
    Opcode::LDH(Target::ZeroImmediate, Target::A),
    Opcode::Undefined,
    Opcode::LD(Target::MC, Target::A),
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::LD(Target::MImmediate, Target::A),
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    // 0xF0 - 0xFF
    Opcode::LDH(Target::A, Target::ZeroImmediate),
    Opcode::Undefined,
    Opcode::LD(Target::A, Target::MC),
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::CP(Target::Immediate),
    Opcode::Undefined,
];

pub fn nop(_: &mut CPU) {}

pub fn add(cpu: &mut CPU, value: u8) {
    let (new_value, did_overflow) = cpu.registers.a.overflowing_add(value);

    cpu.registers.f.set_zero(new_value == 0);
    cpu.registers.f.set_subtract(false);
    cpu.registers.f.set_carry(did_overflow);

    // Half carry is set if adding the lower nibbles of the value and the
    // register together results in a value bigger than 0xF.
    cpu.registers
        .f
        .set_half_carry((cpu.registers.a & 0xF) + (value & 0xF) > 0xF);

    cpu.registers.a = new_value;
}

pub fn adc(cpu: &mut CPU, value: u8) {
    let a = cpu.registers.a as u16;
    let b = value as u16;
    let carry = if cpu.registers.f.carry() { 1u16 } else { 0 };

    let new_value = a.wrapping_add(b).wrapping_add(carry);

    cpu.registers.f.set_zero(new_value & 0xFF == 0);
    cpu.registers.f.set_subtract(false);
    cpu.registers.f.set_carry(new_value & 0x100 != 0);

    // Half carry is set if adding the lower nibbles of the value and the
    // register together results in a value bigger than 0xF.
    cpu.registers
        .f
        .set_half_carry((a ^ b ^ new_value) & 0x10 != 0);

    cpu.registers.a = new_value as u8;
}

#[cfg(test)]
mod tests {
    use crate::hardware::{bus::Bus, cartridge::Cartridge};

    use super::*;

    #[test]
    fn test_add() {
        let bus = Bus::new(Cartridge::from_path("priv/cpu_instrs.gb").unwrap());
        let mut cpu = CPU::new(bus);
        cpu.registers.a = 0x01;
        add(&mut cpu, 0x01);
        assert_eq!(cpu.registers.a, 0x02);
        assert_eq!(cpu.registers.f.zero(), false);
        assert_eq!(cpu.registers.f.subtract(), false);
        assert_eq!(cpu.registers.f.carry(), false);
        assert_eq!(cpu.registers.f.half_carry(), false);

        cpu.registers.a = 0xFF;
        add(&mut cpu, 0x01);
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.registers.f.zero(), true);
        assert_eq!(cpu.registers.f.subtract(), false);
        assert_eq!(cpu.registers.f.carry(), true);
        assert_eq!(cpu.registers.f.half_carry(), true);

        cpu.registers.a = 0x0F;
        add(&mut cpu, 0x01);
        assert_eq!(cpu.registers.a, 0x10);
        assert_eq!(cpu.registers.f.zero(), false);
        assert_eq!(cpu.registers.f.subtract(), false);
        assert_eq!(cpu.registers.f.carry(), false);
        assert_eq!(cpu.registers.f.half_carry(), true);

        cpu.registers.a = 0xF0;
        add(&mut cpu, 0x10);
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.registers.f.zero(), true);
        assert_eq!(cpu.registers.f.subtract(), false);
        assert_eq!(cpu.registers.f.carry(), true);
        assert_eq!(cpu.registers.f.half_carry(), false);
    }

    #[test]
    fn test_adc() {
        let bus = Bus::new(Cartridge::from_path("priv/cpu_instrs.gb").unwrap());
        let mut cpu = CPU::new(bus);
        cpu.registers.a = 0x01;
        cpu.registers.f.set_carry(false);
        adc(&mut cpu, 0x01);
        assert_eq!(cpu.registers.a, 0x02);
        assert_eq!(cpu.registers.f.zero(), false);
        assert_eq!(cpu.registers.f.subtract(), false);
        assert_eq!(cpu.registers.f.carry(), false);
        assert_eq!(cpu.registers.f.half_carry(), false);

        cpu.registers.a = 0xFF;
        cpu.registers.f.set_carry(false);
        adc(&mut cpu, 0x01);
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.registers.f.zero(), true);
        assert_eq!(cpu.registers.f.subtract(), false);
        assert_eq!(cpu.registers.f.carry(), true);
        assert_eq!(cpu.registers.f.half_carry(), true);

        cpu.registers.a = 0x0F;
        cpu.registers.f.set_carry(false);
        adc(&mut cpu, 0x01);
        assert_eq!(cpu.registers.a, 0x10);
        assert_eq!(cpu.registers.f.zero(), false);
        assert_eq!(cpu.registers.f.subtract(), false);
        assert_eq!(cpu.registers.f.carry(), false);
        assert_eq!(cpu.registers.f.half_carry(), true);

        cpu.registers.a = 0xF0;
        cpu.registers.f.set_carry(false);
        adc(&mut cpu, 0x10);
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.registers.f.zero(), true);
        assert_eq!(cpu.registers.f.subtract(), false);
        assert_eq!(cpu.registers.f.carry(), true);
        assert_eq!(cpu.registers.f.half_carry(), false);

        cpu.registers.a = 0x7F;
        cpu.registers.f.set_carry(true);
        adc(&mut cpu, 0x01);
        assert_eq!(cpu.registers.a, 0x81);
        assert_eq!(cpu.registers.f.zero(), false);
        assert_eq!(cpu.registers.f.subtract(), false);
        assert_eq!(cpu.registers.f.carry(), false);
        assert_eq!(cpu.registers.f.half_carry(), true);
    }
}
