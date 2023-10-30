use crate::hardware::cpu::CPU;

use super::Target;

pub fn cp(cpu: &mut CPU, target: &Target) {
    // Check for borrow using 32bit arithmetics
    let x = cpu.registers.a as u32;
    let y = target.get_value(cpu) as u32;

    let r = x.wrapping_sub(y);

    let rb = r as u8;

    cpu.registers.f.set_zero(rb == 0);
    cpu.registers.f.set_half_carry((x ^ y ^ r) & 0x10 != 0);
    cpu.registers.f.set_carry(r & 0x100 != 0);
    cpu.registers.f.set_subtract(true);

    // rb
}
