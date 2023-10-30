use crate::hardware::cpu::CPU;

use super::Target;

pub fn inc(cpu: &mut CPU, target: &Target) {
    let value = target.get_value(cpu);

    let result = value.wrapping_add(1);

    target.set_value(cpu, result);

    cpu.registers.f.set_zero(result == 0);
    cpu.registers.f.set_subtract(false);
    cpu.registers.f.set_half_carry(value & 0xf == 0xf);
}
