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
    BIT(BitStatus, BitTarget, u8),
    RES,
    SET,

    Undefined,
}

#[derive(Debug, Clone, Copy)]
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
}

pub fn prefix_cb(cpu: &mut CPU) {
    let op = cpu.next_byte();
    let opcode = &CB_OPCODES[op as usize];

    match opcode {
        CBOpcode::BIT(BitStatus::High, target, bit) => bit_h(cpu, target, bit),
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

fn bit_h(cpu: &mut CPU, target: &BitTarget, bit: &u8) {
    let value = match target {
        BitTarget::A => cpu.registers.a,
        BitTarget::B => cpu.registers.b,
        BitTarget::C => cpu.registers.c,
        BitTarget::D => cpu.registers.d,
        BitTarget::E => cpu.registers.e,
        BitTarget::H => cpu.registers.h,
        BitTarget::L => cpu.registers.l,
        BitTarget::MHL => cpu.bus.read(cpu.registers.hl()),
    };

    let result = value & (1 << bit);

    cpu.registers.f.set_zero(result == 0);
    cpu.registers.f.set_subtract(false);
    cpu.registers.f.set_half_carry(true);
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
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    // 0x50
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    // 0x60
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    // 0x70
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::BIT(BitStatus::High, BitTarget::H, 7),
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    // 0x80
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    // 0x90
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    // 0xA0
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    // 0xB0
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    // 0xC0
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    // 0xD0
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    // 0xE0
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    // 0xF0
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
    CBOpcode::Undefined,
];
