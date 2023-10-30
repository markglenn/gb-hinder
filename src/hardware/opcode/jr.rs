use crate::hardware::cpu::CPU;

use super::{Condition, Target16};

pub fn jr(cpu: &mut CPU, condition: &Condition) {
    let offset = cpu.next_byte() as i8;

    if condition.test(cpu) {
        let mut pc = cpu.pc as i16;
        pc = pc.wrapping_add(offset as i16);

        cpu.pc = pc as u16;
    }
}

pub fn jp(cpu: &mut CPU, condition: &Condition, target: &Target16) {
    let address = target.get_value(cpu);

    if condition.test(cpu) {
        cpu.pc = address;
    }
}
