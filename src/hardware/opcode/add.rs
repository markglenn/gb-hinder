use super::{Target, Target16};
use crate::hardware::cpu::CPU;

pub fn add(cpu: &mut CPU, target: &Target) {
    let value = target.get_value(cpu);

    let a = cpu.registers.a;
    let (result, carry) = a.overflowing_add(value);

    cpu.registers.a = result;

    cpu.registers.f.set_zero(cpu.registers.a == 0);
    cpu.registers.f.set_subtract(false);
    cpu.registers
        .f
        .set_half_carry((a & 0x0F) + (value & 0x0F) > 0x0F);
    cpu.registers.f.set_carry(carry);
}

pub fn add16(cpu: &mut CPU, target: &Target16) {
    let a = cpu.registers.hl() as u32;
    let b = target.get_value(cpu) as u32;

    let new_value = a + b;

    cpu.registers.f.set_subtract(false);
    cpu.registers
        .f
        .set_half_carry((a ^ b ^ new_value) & 0x1000 != 0);
    cpu.registers.f.set_carry(new_value > 0xFFFF);

    cpu.registers.set_hl(new_value as u16);
}

pub fn adc(cpu: &mut CPU, target: &Target) {
    let a = cpu.registers.a as u16;
    let b = target.get_value(cpu) as u16;

    let carry = if cpu.registers.f.carry() { 1u16 } else { 0 };

    let new_value = a.wrapping_add(b).wrapping_add(carry);

    cpu.registers.f.set_zero(new_value & 0xFF == 0);
    cpu.registers.f.set_subtract(false);
    cpu.registers.f.set_carry(new_value & 0x100 != 0);

    // Half carry is set if adding the lower nibbles of the value and the
    // register together results in a value bigger than 0xF.
    cpu.registers
        .f
        .set_half_carry((a ^ b ^ new_value) & 0x10 != 0);

    cpu.registers.a = new_value as u8;
}

pub fn sub(cpu: &mut CPU, target: &Target) {
    let value = target.get_value(cpu);

    let (result, carry) = cpu.registers.a.overflowing_sub(value);

    cpu.registers.a = result;

    cpu.registers.f.set_zero(cpu.registers.a == 0);
    cpu.registers.f.set_subtract(true);
    cpu.registers
        .f
        .set_half_carry((cpu.registers.a & 0x0F) + (value & 0x0F) > 0x0F);
    cpu.registers.f.set_carry(carry);
}

pub fn daa(cpu: &mut CPU) {
    let mut a = cpu.registers.a;
    let mut adjust = if cpu.registers.f.carry() { 0x60 } else { 0x00 };

    if cpu.registers.f.half_carry() {
        adjust |= 0x06;
    };

    if !cpu.registers.f.subtract() {
        if a & 0x0F > 0x09 {
            adjust |= 0x06;
        };
        if a > 0x99 {
            adjust |= 0x60;
        };
        a = a.wrapping_add(adjust);
    } else {
        a = a.wrapping_sub(adjust);
    }

    cpu.registers.f.set_carry(adjust >= 0x60);
    cpu.registers.f.set_half_carry(false);
    cpu.registers.f.set_zero(a == 0);
    cpu.registers.a = a;
}
