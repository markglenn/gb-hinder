use crate::hardware::cpu::CPU;

use super::{Target, Target16};

pub fn ld(cpu: &mut CPU, target: &Target, from_target: &Target) {
    let value = from_target.get_value(cpu);
    target.set_value(cpu, value);
}

pub fn ld16(cpu: &mut CPU, target: &Target16, from_target: &Target16) {
    let value = from_target.get_value(cpu);
    target.set_value(cpu, value);
}

pub fn ldd(cpu: &mut CPU, target: &Target, from: &Target) {
    let value = from.get_value(cpu);
    target.set_value(cpu, value);

    cpu.registers.set_hl(cpu.registers.hl().wrapping_sub(1));
}

pub fn ldh(cpu: &mut CPU, target: &Target, from: &Target) {
    let value = from.get_value(cpu);
    target.set_value(cpu, value);
}
