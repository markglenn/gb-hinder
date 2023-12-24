use crate::hardware::{cpu::CPU, Memory};

use super::Target;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum CBOpcode {
    RLC(Target),
    RRC(Target),
    RL(Target),
    RR(Target),
    SLA(Target),
    SRA(Target),
    SWAP(Target),
    SRL(Target),
    BIT(BitTarget, u8),
    RES(BitTarget, u8),
    SET(BitTarget, u8),

    Undefined,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum BitStatus {
    High,
    Low,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum BitTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    MHL,
}

#[allow(dead_code)]
impl BitTarget {
    pub fn get_value(self, cpu: &mut CPU) -> u8 {
        match self {
            BitTarget::A => cpu.registers.a,
            BitTarget::B => cpu.registers.b,
            BitTarget::C => cpu.registers.c,
            BitTarget::D => cpu.registers.d,
            BitTarget::E => cpu.registers.e,
            BitTarget::H => cpu.registers.h,
            BitTarget::L => cpu.registers.l,
            BitTarget::MHL => cpu.bus.read(cpu.registers.hl()),
        }
    }
    pub fn set_value(self, cpu: &mut CPU, value: u8) {
        match self {
            BitTarget::A => cpu.registers.a = value,
            BitTarget::B => cpu.registers.b = value,
            BitTarget::C => cpu.registers.c = value,
            BitTarget::D => cpu.registers.d = value,
            BitTarget::E => cpu.registers.e = value,
            BitTarget::H => cpu.registers.h = value,
            BitTarget::L => cpu.registers.l = value,
            BitTarget::MHL => cpu.bus.write(cpu.registers.hl(), value),
        };
    }
}

pub fn prefix_cb(cpu: &mut CPU) {
    let op = cpu.next_byte();
    let opcode = &CB_OPCODES[op as usize];

    match opcode {
        CBOpcode::BIT(target, bit) => test_bit(cpu, target, bit),
        CBOpcode::RES(target, bit) => set_bit(cpu, target, bit, BitStatus::Low),
        CBOpcode::SET(target, bit) => set_bit(cpu, target, bit, BitStatus::High),
        CBOpcode::RL(target) => rl(cpu, target, true),
        CBOpcode::SRL(target) => srl(cpu, target),
        CBOpcode::SLA(target) => sla(cpu, target),
        CBOpcode::SRA(target) => sra(cpu, target),
        CBOpcode::RR(target) => rr(cpu, target),
        CBOpcode::RLC(target) => rlc(cpu, target, true),
        CBOpcode::RRC(target) => rrc(cpu, target, true),
        CBOpcode::SWAP(target) => swap(cpu, target),
        _ => panic!("Unimplemented bit opcode: 0x{:02X}", op),
    }
}

fn test_bit(cpu: &mut CPU, target: &BitTarget, bit: &u8) {
    let value = target.get_value(cpu);
    let result = value & (1 << bit);

    cpu.registers.f.set_zero(result == 0);
    cpu.registers.f.set_subtract(false);
    cpu.registers.f.set_half_carry(true);
}

fn set_bit(cpu: &mut CPU, target: &BitTarget, bit: &u8, status: BitStatus) {
    let value = target.get_value(cpu);
    let mask = (1 << bit) as u8;

    let result = match status {
        BitStatus::High => value | mask,
        BitStatus::Low => value & !mask,
    };

    target.set_value(cpu, result);
}

pub fn rra(cpu: &mut CPU) {
    let a = cpu.registers.a;

    let newcarry = (a & 1) != 0;
    let oldcarry = cpu.registers.f.carry() as u8;

    cpu.registers.a = (a >> 1) | (oldcarry << 7);

    cpu.registers.f.set_zero(false);
    cpu.registers.f.set_subtract(false);
    cpu.registers.f.set_half_carry(false);
    cpu.registers.f.set_carry(newcarry);
}

pub fn rl(cpu: &mut CPU, target: &Target, allow_zero: bool) {
    let value = target.get_value(cpu);
    let carry = if cpu.registers.f.carry() { 1 } else { 0 };

    let result = (value << 1) | carry;

    cpu.registers.f.set_zero(allow_zero && result == 0);
    cpu.registers.f.set_subtract(false);
    cpu.registers.f.set_half_carry(false);
    cpu.registers.f.set_carry(value & 0x80 != 0);

    target.set_value(cpu, result);
}

pub fn rlc(cpu: &mut CPU, target: &Target, allow_zero: bool) {
    let value = target.get_value(cpu);
    let result = value.rotate_left(1);

    cpu.registers.f.set_zero(allow_zero && result == 0);
    cpu.registers.f.set_subtract(false);
    cpu.registers.f.set_half_carry(false);
    cpu.registers.f.set_carry(value & 0x80 != 0);

    target.set_value(cpu, result);
}

pub fn rrc(cpu: &mut CPU, target: &Target, allow_zero: bool) {
    let value = target.get_value(cpu);
    let result = value.rotate_right(1);

    cpu.registers.f.set_zero(allow_zero && result == 0);
    cpu.registers.f.set_subtract(false);
    cpu.registers.f.set_half_carry(false);
    cpu.registers.f.set_carry(value & 0x01 == 0x01);

    target.set_value(cpu, result);
}

pub fn rr(cpu: &mut CPU, target: &Target) {
    let value = target.get_value(cpu);
    let carry = if cpu.registers.f.carry() { 0x80 } else { 0 };

    let result = (value >> 1) | carry;

    cpu.registers.f.set_zero(result == 0);
    cpu.registers.f.set_subtract(false);
    cpu.registers.f.set_half_carry(false);
    cpu.registers.f.set_carry(value & 0x01 != 0);

    target.set_value(cpu, result);
}

pub fn srl(cpu: &mut CPU, target: &Target) {
    let value = target.get_value(cpu);

    let result = value >> 1;

    cpu.registers.f.set_zero(result == 0);
    cpu.registers.f.set_subtract(false);
    cpu.registers.f.set_half_carry(false);
    cpu.registers.f.set_carry(value & 0x01 != 0);

    target.set_value(cpu, result);
}

pub fn swap(cpu: &mut CPU, target: &Target) {
    let value = target.get_value(cpu);

    let result = (value << 4) | (value >> 4);

    cpu.registers.f.set_zero(result == 0);
    cpu.registers.f.set_subtract(false);
    cpu.registers.f.set_half_carry(false);
    cpu.registers.f.set_carry(false);

    target.set_value(cpu, result);
}

pub fn sla(cpu: &mut CPU, target: &Target) {
    let value = target.get_value(cpu);
    let result = value << 1;

    cpu.registers.f.set_zero(result == 0);
    cpu.registers.f.set_subtract(false);
    cpu.registers.f.set_half_carry(false);
    cpu.registers.f.set_carry(value & 0x80 != 0);

    target.set_value(cpu, result);
}

pub fn sra(cpu: &mut CPU, target: &Target) {
    let value = target.get_value(cpu);
    let result = value >> 1 | value & 0x80;

    cpu.registers.f.set_zero(result == 0);
    cpu.registers.f.set_subtract(false);
    cpu.registers.f.set_half_carry(false);
    cpu.registers.f.set_carry(value & 0x01 != 0);

    target.set_value(cpu, result);
}

pub static CB_OPCODES: [CBOpcode; 0x100] = [
    // 0x00
    CBOpcode::RLC(Target::B),
    CBOpcode::RLC(Target::C),
    CBOpcode::RLC(Target::D),
    CBOpcode::RLC(Target::E),
    CBOpcode::RLC(Target::H),
    CBOpcode::RLC(Target::L),
    CBOpcode::RLC(Target::MHL),
    CBOpcode::RLC(Target::A),
    CBOpcode::RRC(Target::B),
    CBOpcode::RRC(Target::C),
    CBOpcode::RRC(Target::D),
    CBOpcode::RRC(Target::E),
    CBOpcode::RRC(Target::H),
    CBOpcode::RRC(Target::L),
    CBOpcode::RRC(Target::MHL),
    CBOpcode::RRC(Target::A),
    // 0x10
    CBOpcode::RL(Target::B),
    CBOpcode::RL(Target::C),
    CBOpcode::RL(Target::D),
    CBOpcode::RL(Target::E),
    CBOpcode::RL(Target::H),
    CBOpcode::RL(Target::L),
    CBOpcode::RL(Target::MHL),
    CBOpcode::RL(Target::A),
    CBOpcode::RR(Target::B),
    CBOpcode::RR(Target::C),
    CBOpcode::RR(Target::D),
    CBOpcode::RR(Target::E),
    CBOpcode::RR(Target::H),
    CBOpcode::RR(Target::L),
    CBOpcode::RR(Target::MHL),
    CBOpcode::RR(Target::A),
    // 0x20
    CBOpcode::SLA(Target::B),
    CBOpcode::SLA(Target::C),
    CBOpcode::SLA(Target::D),
    CBOpcode::SLA(Target::E),
    CBOpcode::SLA(Target::H),
    CBOpcode::SLA(Target::L),
    CBOpcode::SLA(Target::MHL),
    CBOpcode::SLA(Target::A),
    CBOpcode::SRA(Target::B),
    CBOpcode::SRA(Target::C),
    CBOpcode::SRA(Target::D),
    CBOpcode::SRA(Target::E),
    CBOpcode::SRA(Target::H),
    CBOpcode::SRA(Target::L),
    CBOpcode::SRA(Target::MHL),
    CBOpcode::SRA(Target::A),
    // 0x30
    CBOpcode::SWAP(Target::B),
    CBOpcode::SWAP(Target::C),
    CBOpcode::SWAP(Target::D),
    CBOpcode::SWAP(Target::E),
    CBOpcode::SWAP(Target::H),
    CBOpcode::SWAP(Target::L),
    CBOpcode::SWAP(Target::MHL),
    CBOpcode::SWAP(Target::A),
    CBOpcode::SRL(Target::B),
    CBOpcode::SRL(Target::C),
    CBOpcode::SRL(Target::D),
    CBOpcode::SRL(Target::E),
    CBOpcode::SRL(Target::H),
    CBOpcode::SRL(Target::L),
    CBOpcode::SRL(Target::MHL),
    CBOpcode::SRL(Target::A),
    // 0x40
    CBOpcode::BIT(BitTarget::B, 0),
    CBOpcode::BIT(BitTarget::C, 0),
    CBOpcode::BIT(BitTarget::D, 0),
    CBOpcode::BIT(BitTarget::E, 0),
    CBOpcode::BIT(BitTarget::H, 0),
    CBOpcode::BIT(BitTarget::L, 0),
    CBOpcode::BIT(BitTarget::MHL, 0),
    CBOpcode::BIT(BitTarget::A, 0),
    CBOpcode::BIT(BitTarget::B, 1),
    CBOpcode::BIT(BitTarget::C, 1),
    CBOpcode::BIT(BitTarget::D, 1),
    CBOpcode::BIT(BitTarget::E, 1),
    CBOpcode::BIT(BitTarget::H, 1),
    CBOpcode::BIT(BitTarget::L, 1),
    CBOpcode::BIT(BitTarget::MHL, 1),
    CBOpcode::BIT(BitTarget::A, 1),
    // 0x50
    CBOpcode::BIT(BitTarget::B, 2),
    CBOpcode::BIT(BitTarget::C, 2),
    CBOpcode::BIT(BitTarget::D, 2),
    CBOpcode::BIT(BitTarget::E, 2),
    CBOpcode::BIT(BitTarget::H, 2),
    CBOpcode::BIT(BitTarget::L, 2),
    CBOpcode::BIT(BitTarget::MHL, 2),
    CBOpcode::BIT(BitTarget::A, 2),
    CBOpcode::BIT(BitTarget::B, 3),
    CBOpcode::BIT(BitTarget::C, 3),
    CBOpcode::BIT(BitTarget::D, 3),
    CBOpcode::BIT(BitTarget::E, 3),
    CBOpcode::BIT(BitTarget::H, 3),
    CBOpcode::BIT(BitTarget::L, 3),
    CBOpcode::BIT(BitTarget::MHL, 3),
    CBOpcode::BIT(BitTarget::A, 3),
    // 0x60
    CBOpcode::BIT(BitTarget::B, 4),
    CBOpcode::BIT(BitTarget::C, 4),
    CBOpcode::BIT(BitTarget::D, 4),
    CBOpcode::BIT(BitTarget::E, 4),
    CBOpcode::BIT(BitTarget::H, 4),
    CBOpcode::BIT(BitTarget::L, 4),
    CBOpcode::BIT(BitTarget::MHL, 4),
    CBOpcode::BIT(BitTarget::A, 4),
    CBOpcode::BIT(BitTarget::B, 5),
    CBOpcode::BIT(BitTarget::C, 5),
    CBOpcode::BIT(BitTarget::D, 5),
    CBOpcode::BIT(BitTarget::E, 5),
    CBOpcode::BIT(BitTarget::H, 5),
    CBOpcode::BIT(BitTarget::L, 5),
    CBOpcode::BIT(BitTarget::MHL, 5),
    CBOpcode::BIT(BitTarget::A, 5),
    // 0x70
    CBOpcode::BIT(BitTarget::B, 6),
    CBOpcode::BIT(BitTarget::C, 6),
    CBOpcode::BIT(BitTarget::D, 6),
    CBOpcode::BIT(BitTarget::E, 6),
    CBOpcode::BIT(BitTarget::H, 6),
    CBOpcode::BIT(BitTarget::L, 6),
    CBOpcode::BIT(BitTarget::MHL, 6),
    CBOpcode::BIT(BitTarget::A, 6),
    CBOpcode::BIT(BitTarget::B, 7),
    CBOpcode::BIT(BitTarget::C, 7),
    CBOpcode::BIT(BitTarget::D, 7),
    CBOpcode::BIT(BitTarget::E, 7),
    CBOpcode::BIT(BitTarget::H, 7),
    CBOpcode::BIT(BitTarget::L, 7),
    CBOpcode::BIT(BitTarget::MHL, 7),
    CBOpcode::BIT(BitTarget::A, 7),
    // 0x80
    CBOpcode::RES(BitTarget::B, 0),
    CBOpcode::RES(BitTarget::C, 0),
    CBOpcode::RES(BitTarget::D, 0),
    CBOpcode::RES(BitTarget::E, 0),
    CBOpcode::RES(BitTarget::H, 0),
    CBOpcode::RES(BitTarget::L, 0),
    CBOpcode::RES(BitTarget::MHL, 0),
    CBOpcode::RES(BitTarget::A, 0),
    CBOpcode::RES(BitTarget::B, 1),
    CBOpcode::RES(BitTarget::C, 1),
    CBOpcode::RES(BitTarget::D, 1),
    CBOpcode::RES(BitTarget::E, 1),
    CBOpcode::RES(BitTarget::H, 1),
    CBOpcode::RES(BitTarget::L, 1),
    CBOpcode::RES(BitTarget::MHL, 1),
    CBOpcode::RES(BitTarget::A, 1),
    // 0x90
    CBOpcode::RES(BitTarget::B, 2),
    CBOpcode::RES(BitTarget::C, 2),
    CBOpcode::RES(BitTarget::D, 2),
    CBOpcode::RES(BitTarget::E, 2),
    CBOpcode::RES(BitTarget::H, 2),
    CBOpcode::RES(BitTarget::L, 2),
    CBOpcode::RES(BitTarget::MHL, 2),
    CBOpcode::RES(BitTarget::A, 2),
    CBOpcode::RES(BitTarget::B, 3),
    CBOpcode::RES(BitTarget::C, 3),
    CBOpcode::RES(BitTarget::D, 3),
    CBOpcode::RES(BitTarget::E, 3),
    CBOpcode::RES(BitTarget::H, 3),
    CBOpcode::RES(BitTarget::L, 3),
    CBOpcode::RES(BitTarget::MHL, 3),
    CBOpcode::RES(BitTarget::A, 3),
    // 0xA0
    CBOpcode::RES(BitTarget::B, 4),
    CBOpcode::RES(BitTarget::C, 4),
    CBOpcode::RES(BitTarget::D, 4),
    CBOpcode::RES(BitTarget::E, 4),
    CBOpcode::RES(BitTarget::H, 4),
    CBOpcode::RES(BitTarget::L, 4),
    CBOpcode::RES(BitTarget::MHL, 4),
    CBOpcode::RES(BitTarget::A, 4),
    CBOpcode::RES(BitTarget::B, 5),
    CBOpcode::RES(BitTarget::C, 5),
    CBOpcode::RES(BitTarget::D, 5),
    CBOpcode::RES(BitTarget::E, 5),
    CBOpcode::RES(BitTarget::H, 5),
    CBOpcode::RES(BitTarget::L, 5),
    CBOpcode::RES(BitTarget::MHL, 5),
    CBOpcode::RES(BitTarget::A, 5),
    // 0xB0
    CBOpcode::RES(BitTarget::B, 6),
    CBOpcode::RES(BitTarget::C, 6),
    CBOpcode::RES(BitTarget::D, 6),
    CBOpcode::RES(BitTarget::E, 6),
    CBOpcode::RES(BitTarget::H, 6),
    CBOpcode::RES(BitTarget::L, 6),
    CBOpcode::RES(BitTarget::MHL, 6),
    CBOpcode::RES(BitTarget::A, 6),
    CBOpcode::RES(BitTarget::B, 7),
    CBOpcode::RES(BitTarget::C, 7),
    CBOpcode::RES(BitTarget::D, 7),
    CBOpcode::RES(BitTarget::E, 7),
    CBOpcode::RES(BitTarget::H, 7),
    CBOpcode::RES(BitTarget::L, 7),
    CBOpcode::RES(BitTarget::MHL, 7),
    CBOpcode::RES(BitTarget::A, 7),
    // 0xC0
    CBOpcode::SET(BitTarget::B, 0),
    CBOpcode::SET(BitTarget::C, 0),
    CBOpcode::SET(BitTarget::D, 0),
    CBOpcode::SET(BitTarget::E, 0),
    CBOpcode::SET(BitTarget::H, 0),
    CBOpcode::SET(BitTarget::L, 0),
    CBOpcode::SET(BitTarget::MHL, 0),
    CBOpcode::SET(BitTarget::A, 0),
    CBOpcode::SET(BitTarget::B, 1),
    CBOpcode::SET(BitTarget::C, 1),
    CBOpcode::SET(BitTarget::D, 1),
    CBOpcode::SET(BitTarget::E, 1),
    CBOpcode::SET(BitTarget::H, 1),
    CBOpcode::SET(BitTarget::L, 1),
    CBOpcode::SET(BitTarget::MHL, 1),
    CBOpcode::SET(BitTarget::A, 1),
    // 0xD0
    CBOpcode::SET(BitTarget::B, 2),
    CBOpcode::SET(BitTarget::C, 2),
    CBOpcode::SET(BitTarget::D, 2),
    CBOpcode::SET(BitTarget::E, 2),
    CBOpcode::SET(BitTarget::H, 2),
    CBOpcode::SET(BitTarget::L, 2),
    CBOpcode::SET(BitTarget::MHL, 2),
    CBOpcode::SET(BitTarget::A, 2),
    CBOpcode::SET(BitTarget::B, 3),
    CBOpcode::SET(BitTarget::C, 3),
    CBOpcode::SET(BitTarget::D, 3),
    CBOpcode::SET(BitTarget::E, 3),
    CBOpcode::SET(BitTarget::H, 3),
    CBOpcode::SET(BitTarget::L, 3),
    CBOpcode::SET(BitTarget::MHL, 3),
    CBOpcode::SET(BitTarget::A, 3),
    // 0xE0
    CBOpcode::SET(BitTarget::B, 4),
    CBOpcode::SET(BitTarget::C, 4),
    CBOpcode::SET(BitTarget::D, 4),
    CBOpcode::SET(BitTarget::E, 4),
    CBOpcode::SET(BitTarget::H, 4),
    CBOpcode::SET(BitTarget::L, 4),
    CBOpcode::SET(BitTarget::MHL, 4),
    CBOpcode::SET(BitTarget::A, 4),
    CBOpcode::SET(BitTarget::B, 5),
    CBOpcode::SET(BitTarget::C, 5),
    CBOpcode::SET(BitTarget::D, 5),
    CBOpcode::SET(BitTarget::E, 5),
    CBOpcode::SET(BitTarget::H, 5),
    CBOpcode::SET(BitTarget::L, 5),
    CBOpcode::SET(BitTarget::MHL, 5),
    CBOpcode::SET(BitTarget::A, 5),
    // 0xF0
    CBOpcode::SET(BitTarget::B, 6),
    CBOpcode::SET(BitTarget::C, 6),
    CBOpcode::SET(BitTarget::D, 6),
    CBOpcode::SET(BitTarget::E, 6),
    CBOpcode::SET(BitTarget::H, 6),
    CBOpcode::SET(BitTarget::L, 6),
    CBOpcode::SET(BitTarget::MHL, 6),
    CBOpcode::SET(BitTarget::A, 6),
    CBOpcode::SET(BitTarget::B, 7),
    CBOpcode::SET(BitTarget::C, 7),
    CBOpcode::SET(BitTarget::D, 7),
    CBOpcode::SET(BitTarget::E, 7),
    CBOpcode::SET(BitTarget::H, 7),
    CBOpcode::SET(BitTarget::L, 7),
    CBOpcode::SET(BitTarget::MHL, 7),
    CBOpcode::SET(BitTarget::A, 7),
];
