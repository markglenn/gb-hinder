use crate::hardware::cpu::CPU;

use super::{Target, Target16};

pub fn inc(cpu: &mut CPU, target: &Target) {
    let value = target.get_value(cpu);
    // Pause by waiting for input
    // let mut input = String::new();
    // std::io::stdin().read_line(&mut input).unwrap();

    let result = value.wrapping_add(1);

    target.set_value(cpu, result);

    cpu.registers.f.set_zero(result == 0);
    cpu.registers.f.set_subtract(false);
    cpu.registers.f.set_half_carry(value & 0xf == 0xf);
}

pub fn dec(cpu: &mut CPU, target: &Target) {
    let value = target.get_value(cpu);

    let result = value.wrapping_sub(1);

    target.set_value(cpu, result);

    cpu.registers.f.set_zero(result == 0);
    cpu.registers.f.set_subtract(true);
    cpu.registers.f.set_half_carry(value & 0xf == 0);
}

pub fn inc16(cpu: &mut CPU, target: &Target16) {
    let value = target.get_value(cpu);
    target.set_value(cpu, value.wrapping_add(1));
}

pub fn dec16(cpu: &mut CPU, target: &Target16) {
    let value = target.get_value(cpu);
    target.set_value(cpu, value.wrapping_sub(1));
}
