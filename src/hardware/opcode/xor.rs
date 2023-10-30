use crate::hardware::cpu::CPU;

use super::Target;

pub fn xor(cpu: &mut CPU, target: &Target) {
    let value = target.get_value(cpu);

    cpu.registers.a ^= value;

    cpu.registers.f.set_zero(cpu.registers.a == 0);
    cpu.registers.f.set_subtract(false);
    cpu.registers.f.set_half_carry(false);
    cpu.registers.f.set_carry(false);
}
