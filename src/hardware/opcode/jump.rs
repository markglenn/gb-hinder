use crate::hardware::cpu::CPU;

use super::{Condition, Target16};

pub fn call(cpu: &mut CPU, condition: &Condition) {
    let address = cpu.next_word();

    if condition.test(cpu) {
        cpu.push_word(cpu.pc);
        cpu.pc = address;
    }
}

pub fn ret(cpu: &mut CPU, condition: &Condition) {
    if condition.test(cpu) {
        let address = cpu.pop_word();
        cpu.pc = address;
    }
}

pub fn reti(cpu: &mut CPU) {
    cpu.pc = cpu.pop_word();
    cpu.ime = true;
}

pub fn rst(cpu: &mut CPU, address: u16) {
    cpu.push_word(cpu.pc);
    cpu.pc = address;
}

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
