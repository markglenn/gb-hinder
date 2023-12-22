use crate::hardware::cpu;

use super::Target16;

pub fn push(cpu: &mut cpu::CPU, target: &Target16) {
    let value = target.get_value(cpu);

    cpu.push_word(value);
}

pub fn pop(cpu: &mut cpu::CPU, target: &Target16) {
    let value = cpu.pop_word();

    if target == &Target16::AF {
        target.set_value(cpu, value & 0xFFF0);
    } else {
        target.set_value(cpu, value);
    }
}
