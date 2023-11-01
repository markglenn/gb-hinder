mod add;
mod bits;
mod cp;
mod inc;
mod interrupt;
mod jump;
mod ld;
mod logic;
mod stack;
mod targets;

use self::targets::{Condition, Target, Target16};
use super::cpu::CPU;

#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    NOP,
    RET(Condition),
    DI,
    EI,
    AND(Target),
    OR(Target),
    LD(Target, Target),
    LD16(Target16, Target16),
    LDD(Target, Target),
    LDH(Target, Target),
    LDI(Target, Target),
    ADD(Target),
    ADD16(Target16),
    SUB(Target),
    ADC(Target),
    XOR(Target),
    INC(Target),
    INC16(Target16),
    DEC(Target),
    DEC16(Target16),
    PUSH(Target16),
    POP(Target16),
    RL(Target),
    RRA,
    CALL(Condition),
    JR(Condition),
    JP(Condition, Target16),
    RST(u16),
    PrefixCB,
    CP(Target),
    HALT,
    INV,
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

        Opcode::ADD(target) => add::add(cpu, target),
        Opcode::ADD16(target) => add::add16(cpu, target),
        Opcode::ADC(target) => add::adc(cpu, target),
        Opcode::SUB(target) => add::sub(cpu, target),

        Opcode::DI => interrupt::enable_interrupt(cpu, false),
        Opcode::EI => interrupt::enable_interrupt(cpu, true),

        Opcode::LD16(target, from) => ld::ld16(cpu, target, from),
        Opcode::LDD(target, from) => ld::ldd(cpu, target, from),
        Opcode::LDH(target, from) => ld::ldh(cpu, target, from),
        Opcode::LDI(target, from) => ld::ldi(cpu, target, from),

        Opcode::AND(target) => logic::and(cpu, target),
        Opcode::OR(target) => logic::or(cpu, target),
        Opcode::XOR(target) => logic::xor(cpu, target),

        Opcode::PrefixCB => bits::prefix_cb(cpu),

        Opcode::JP(condition, target) => jump::jp(cpu, condition, target),
        Opcode::JR(condition) => jump::jr(cpu, condition),
        Opcode::RST(address) => jump::rst(cpu, *address),
        Opcode::LD(target, from) => ld::ld(cpu, target, from),
        Opcode::INC(target) => inc::inc(cpu, target),
        Opcode::INC16(target) => inc::inc16(cpu, target),
        Opcode::DEC(target) => inc::dec(cpu, target),
        Opcode::DEC16(target) => inc::dec16(cpu, target),
        Opcode::CALL(condition) => jump::call(cpu, condition),
        Opcode::RET(condition) => jump::ret(cpu, condition),
        Opcode::PUSH(target) => stack::push(cpu, target),
        Opcode::POP(target) => stack::pop(cpu, target),
        Opcode::RL(target) => bits::rl(cpu, target),
        Opcode::RRA => bits::rra(cpu),
        Opcode::CP(target) => cp::cp(cpu, target),

        Opcode::HALT => cpu.set_halted(true),
        Opcode::INV => panic!("Invalid opcode found"),
        Opcode::Undefined => panic!("Attempted to execute undefined opcode"),
    }

    1
}

pub static OPCODES: [Opcode; 0x100] = [
    // 0x00 - 0x0F
    Opcode::NOP,
    Opcode::LD16(Target16::BC, Target16::Immediate),
    Opcode::LD(Target::MBC, Target::A),
    Opcode::INC16(Target16::BC),
    Opcode::INC(Target::B),
    Opcode::DEC(Target::B),
    Opcode::LD(Target::B, Target::Immediate),
    Opcode::Undefined,
    Opcode::LD16(Target16::Immediate, Target16::SP),
    Opcode::ADD16(Target16::BC),
    Opcode::LD(Target::A, Target::MBC),
    Opcode::DEC16(Target16::BC),
    Opcode::INC(Target::C),
    Opcode::DEC(Target::C),
    Opcode::LD(Target::C, Target::Immediate),
    Opcode::Undefined,
    // 0x10 - 0x1F
    Opcode::Undefined,
    Opcode::LD16(Target16::DE, Target16::Immediate),
    Opcode::LD(Target::MDE, Target::A),
    Opcode::INC16(Target16::DE),
    Opcode::INC(Target::D),
    Opcode::DEC(Target::D),
    Opcode::LD(Target::D, Target::Immediate),
    Opcode::RL(Target::A),
    Opcode::JR(Condition::None),
    Opcode::ADD16(Target16::DE),
    Opcode::LD(Target::A, Target::MDE),
    Opcode::DEC16(Target16::DE),
    Opcode::INC(Target::E),
    Opcode::DEC(Target::E),
    Opcode::LD(Target::E, Target::Immediate),
    Opcode::RRA,
    // 0x20 - 0x2F
    Opcode::JR(Condition::NotZero),
    Opcode::LD16(Target16::HL, Target16::Immediate),
    Opcode::LDI(Target::MHL, Target::A),
    Opcode::INC16(Target16::HL),
    Opcode::INC(Target::H),
    Opcode::DEC(Target::H),
    Opcode::LD(Target::H, Target::Immediate),
    Opcode::Undefined,
    Opcode::JR(Condition::Zero),
    Opcode::ADD16(Target16::HL),
    Opcode::LDI(Target::A, Target::MHL),
    Opcode::DEC16(Target16::HL),
    Opcode::INC(Target::L),
    Opcode::DEC(Target::L),
    Opcode::LD(Target::L, Target::Immediate),
    Opcode::Undefined,
    // 0x30 - 0x3F
    Opcode::JR(Condition::NotCarry),
    Opcode::LD16(Target16::SP, Target16::Immediate),
    Opcode::LDD(Target::MHL, Target::A),
    Opcode::INC16(Target16::SP),
    Opcode::INC(Target::MHL),
    Opcode::DEC(Target::MHL),
    Opcode::LD(Target::MHL, Target::Immediate),
    Opcode::Undefined,
    Opcode::JR(Condition::Carry),
    Opcode::ADD16(Target16::SP),
    Opcode::LDD(Target::A, Target::MHL),
    Opcode::DEC16(Target16::SP),
    Opcode::INC(Target::A),
    Opcode::DEC(Target::A),
    Opcode::LD(Target::A, Target::Immediate),
    Opcode::Undefined,
    // 0x40 - 0x4F
    Opcode::LD(Target::B, Target::B),
    Opcode::LD(Target::B, Target::C),
    Opcode::LD(Target::B, Target::D),
    Opcode::LD(Target::B, Target::E),
    Opcode::LD(Target::B, Target::H),
    Opcode::LD(Target::B, Target::L),
    Opcode::LD(Target::B, Target::MHL),
    Opcode::LD(Target::B, Target::A),
    Opcode::LD(Target::C, Target::B),
    Opcode::LD(Target::C, Target::C),
    Opcode::LD(Target::C, Target::D),
    Opcode::LD(Target::C, Target::E),
    Opcode::LD(Target::C, Target::H),
    Opcode::LD(Target::C, Target::L),
    Opcode::LD(Target::C, Target::MHL),
    Opcode::LD(Target::C, Target::A),
    // 0x50 - 0x5F
    Opcode::LD(Target::D, Target::B),
    Opcode::LD(Target::D, Target::C),
    Opcode::LD(Target::D, Target::D),
    Opcode::LD(Target::D, Target::E),
    Opcode::LD(Target::D, Target::H),
    Opcode::LD(Target::D, Target::L),
    Opcode::LD(Target::D, Target::MHL),
    Opcode::LD(Target::D, Target::A),
    Opcode::LD(Target::E, Target::B),
    Opcode::LD(Target::E, Target::C),
    Opcode::LD(Target::E, Target::D),
    Opcode::LD(Target::E, Target::E),
    Opcode::LD(Target::E, Target::H),
    Opcode::LD(Target::E, Target::L),
    Opcode::LD(Target::E, Target::MHL),
    Opcode::LD(Target::E, Target::A),
    // 0x60 - 0x6F
    Opcode::LD(Target::H, Target::B),
    Opcode::LD(Target::H, Target::C),
    Opcode::LD(Target::H, Target::D),
    Opcode::LD(Target::H, Target::E),
    Opcode::LD(Target::H, Target::H),
    Opcode::LD(Target::H, Target::L),
    Opcode::LD(Target::H, Target::MHL),
    Opcode::LD(Target::H, Target::A),
    Opcode::LD(Target::L, Target::B),
    Opcode::LD(Target::L, Target::C),
    Opcode::LD(Target::L, Target::D),
    Opcode::LD(Target::L, Target::E),
    Opcode::LD(Target::L, Target::H),
    Opcode::LD(Target::L, Target::L),
    Opcode::LD(Target::L, Target::MHL),
    Opcode::LD(Target::L, Target::A),
    // 0x70 - 0x7F
    Opcode::LD(Target::MHL, Target::B),
    Opcode::LD(Target::MHL, Target::C),
    Opcode::LD(Target::MHL, Target::D),
    Opcode::LD(Target::MHL, Target::E),
    Opcode::LD(Target::MHL, Target::H),
    Opcode::LD(Target::MHL, Target::L),
    Opcode::HALT,
    Opcode::LD(Target::MHL, Target::A),
    Opcode::LD(Target::A, Target::B),
    Opcode::LD(Target::A, Target::C),
    Opcode::LD(Target::A, Target::D),
    Opcode::LD(Target::A, Target::E),
    Opcode::LD(Target::A, Target::H),
    Opcode::LD(Target::A, Target::L),
    Opcode::LD(Target::A, Target::MHL),
    Opcode::LD(Target::A, Target::A),
    // 0x80 - 0x8F
    Opcode::ADD(Target::B),
    Opcode::ADD(Target::C),
    Opcode::ADD(Target::D),
    Opcode::ADD(Target::E),
    Opcode::ADD(Target::H),
    Opcode::ADD(Target::L),
    Opcode::ADD(Target::MHL),
    Opcode::ADD(Target::A),
    Opcode::ADC(Target::B),
    Opcode::ADC(Target::C),
    Opcode::ADC(Target::D),
    Opcode::ADC(Target::E),
    Opcode::ADC(Target::H),
    Opcode::ADC(Target::L),
    Opcode::ADC(Target::MHL),
    Opcode::ADC(Target::A),
    // 0x90 - 0x9F
    Opcode::SUB(Target::B),
    Opcode::SUB(Target::C),
    Opcode::SUB(Target::D),
    Opcode::SUB(Target::E),
    Opcode::SUB(Target::H),
    Opcode::SUB(Target::L),
    Opcode::SUB(Target::MHL),
    Opcode::SUB(Target::A),
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    // 0xA0 - 0xAF
    Opcode::AND(Target::B),
    Opcode::AND(Target::C),
    Opcode::AND(Target::D),
    Opcode::AND(Target::E),
    Opcode::AND(Target::H),
    Opcode::AND(Target::L),
    Opcode::AND(Target::MHL),
    Opcode::AND(Target::A),
    Opcode::XOR(Target::B),
    Opcode::XOR(Target::C),
    Opcode::XOR(Target::D),
    Opcode::XOR(Target::E),
    Opcode::XOR(Target::H),
    Opcode::XOR(Target::L),
    Opcode::XOR(Target::MHL),
    Opcode::XOR(Target::A),
    // 0xB0 - 0xBF
    Opcode::OR(Target::B),
    Opcode::OR(Target::C),
    Opcode::OR(Target::D),
    Opcode::OR(Target::E),
    Opcode::OR(Target::H),
    Opcode::OR(Target::L),
    Opcode::OR(Target::MHL),
    Opcode::OR(Target::A),
    Opcode::CP(Target::B),
    Opcode::CP(Target::C),
    Opcode::CP(Target::D),
    Opcode::CP(Target::E),
    Opcode::CP(Target::H),
    Opcode::CP(Target::L),
    Opcode::CP(Target::MHL),
    Opcode::CP(Target::A),
    // 0xC0 - 0xCF
    Opcode::RET(Condition::NotZero),
    Opcode::POP(Target16::BC),
    Opcode::Undefined,
    Opcode::JP(Condition::NotZero, Target16::Immediate),
    Opcode::CALL(Condition::NotZero),
    Opcode::PUSH(Target16::BC),
    Opcode::ADD(Target::Immediate),
    Opcode::Undefined,
    Opcode::RET(Condition::Zero),
    Opcode::RET(Condition::None),
    Opcode::Undefined,
    Opcode::PrefixCB,
    Opcode::Undefined,
    Opcode::CALL(Condition::None),
    Opcode::ADC(Target::Immediate),
    Opcode::Undefined,
    // 0xD0 - 0xDF
    Opcode::RET(Condition::NotCarry),
    Opcode::POP(Target16::DE),
    Opcode::JP(Condition::NotCarry, Target16::Immediate),
    Opcode::Undefined,
    Opcode::CALL(Condition::NotCarry),
    Opcode::PUSH(Target16::DE),
    Opcode::SUB(Target::Immediate),
    Opcode::Undefined,
    Opcode::RET(Condition::Carry),
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    // 0xE0 - 0xEF
    Opcode::LDH(Target::ZeroImmediate, Target::A),
    Opcode::POP(Target16::HL),
    Opcode::LD(Target::MC, Target::A),
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::PUSH(Target16::HL),
    Opcode::AND(Target::Immediate),
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::LD(Target::MImmediate, Target::A),
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::XOR(Target::Immediate),
    Opcode::Undefined,
    // 0xF0 - 0xFF
    Opcode::LDH(Target::A, Target::ZeroImmediate),
    Opcode::POP(Target16::AF),
    Opcode::LD(Target::A, Target::MC),
    Opcode::DI,
    Opcode::Undefined,
    Opcode::PUSH(Target16::AF),
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::LD(Target::A, Target::MImmediate),
    Opcode::EI,
    Opcode::Undefined,
    Opcode::Undefined,
    Opcode::CP(Target::Immediate),
    Opcode::RST(0x38),
];

pub fn nop(_: &mut CPU) {}
