use crate::hardware::cpu::CPU;

#[derive(Debug, Clone, Copy)]
pub enum JumpRelativeCondition {
    NotZero,
    Zero,
    NotCarry,
    Carry,
}

pub fn jr(cpu: &mut CPU, condition: &JumpRelativeCondition) {
    let offset = cpu.next_byte() as i8;

    let mut pc = cpu.pc as i16;
    pc = pc.wrapping_add(offset as i16);

    match condition {
        JumpRelativeCondition::NotZero => {
            if !cpu.registers.f.zero() {
                cpu.pc = pc as u16;
            }
        }
        JumpRelativeCondition::Zero => {
            if cpu.registers.f.zero() {
                cpu.pc = pc as u16;
            }
        }
        JumpRelativeCondition::NotCarry => {
            if !cpu.registers.f.carry() {
                cpu.pc = pc as u16;
            }
        }
        JumpRelativeCondition::Carry => {
            if cpu.registers.f.carry() {
                cpu.pc = pc as u16;
            }
        }
    }
}
