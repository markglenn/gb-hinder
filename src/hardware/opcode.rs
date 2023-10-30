use super::cpu::CPU;

pub enum Opcode {
    ADD(ArithmeticTarget),
    ADC(ArithmeticTarget),
}

pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    MHL,
    Immediate,
}

impl ArithmeticTarget {
    pub fn get_value(self, cpu: &mut CPU) -> u8 {
        match self {
            ArithmeticTarget::A => cpu.registers.a,
            ArithmeticTarget::B => cpu.registers.b,
            ArithmeticTarget::C => cpu.registers.c,
            ArithmeticTarget::D => cpu.registers.d,
            ArithmeticTarget::E => cpu.registers.e,
            ArithmeticTarget::H => cpu.registers.h,
            ArithmeticTarget::L => cpu.registers.l,
            ArithmeticTarget::MHL => panic!("Not implemented yet"),
            ArithmeticTarget::Immediate => panic!("Not implemented yet"),
        }
    }
}

pub fn add(cpu: &mut CPU, value: u8) {
    let (new_value, did_overflow) = cpu.registers.a.overflowing_add(value);

    cpu.registers.f.set_zero(new_value == 0);
    cpu.registers.f.set_subtract(false);
    cpu.registers.f.set_carry(did_overflow);

    // Half carry is set if adding the lower nibbles of the value and the
    // register together results in a value bigger than 0xF.
    cpu.registers
        .f
        .set_half_carry((cpu.registers.a & 0xF) + (value & 0xF) > 0xF);

    cpu.registers.a = new_value;
}

pub fn adc(cpu: &mut CPU, value: u8) {
    let a = cpu.registers.a as u16;
    let b = value as u16;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x01;
        add(&mut cpu, 0x01);
        assert_eq!(cpu.registers.a, 0x02);
        assert_eq!(cpu.registers.f.zero(), false);
        assert_eq!(cpu.registers.f.subtract(), false);
        assert_eq!(cpu.registers.f.carry(), false);
        assert_eq!(cpu.registers.f.half_carry(), false);

        cpu.registers.a = 0xFF;
        add(&mut cpu, 0x01);
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.registers.f.zero(), true);
        assert_eq!(cpu.registers.f.subtract(), false);
        assert_eq!(cpu.registers.f.carry(), true);
        assert_eq!(cpu.registers.f.half_carry(), true);

        cpu.registers.a = 0x0F;
        add(&mut cpu, 0x01);
        assert_eq!(cpu.registers.a, 0x10);
        assert_eq!(cpu.registers.f.zero(), false);
        assert_eq!(cpu.registers.f.subtract(), false);
        assert_eq!(cpu.registers.f.carry(), false);
        assert_eq!(cpu.registers.f.half_carry(), true);

        cpu.registers.a = 0xF0;
        add(&mut cpu, 0x10);
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.registers.f.zero(), true);
        assert_eq!(cpu.registers.f.subtract(), false);
        assert_eq!(cpu.registers.f.carry(), true);
        assert_eq!(cpu.registers.f.half_carry(), false);
    }

    #[test]
    fn test_adc() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x01;
        cpu.registers.f.set_carry(false);
        adc(&mut cpu, 0x01);
        assert_eq!(cpu.registers.a, 0x02);
        assert_eq!(cpu.registers.f.zero(), false);
        assert_eq!(cpu.registers.f.subtract(), false);
        assert_eq!(cpu.registers.f.carry(), false);
        assert_eq!(cpu.registers.f.half_carry(), false);

        cpu.registers.a = 0xFF;
        cpu.registers.f.set_carry(false);
        adc(&mut cpu, 0x01);
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.registers.f.zero(), true);
        assert_eq!(cpu.registers.f.subtract(), false);
        assert_eq!(cpu.registers.f.carry(), true);
        assert_eq!(cpu.registers.f.half_carry(), true);

        cpu.registers.a = 0x0F;
        cpu.registers.f.set_carry(false);
        adc(&mut cpu, 0x01);
        assert_eq!(cpu.registers.a, 0x10);
        assert_eq!(cpu.registers.f.zero(), false);
        assert_eq!(cpu.registers.f.subtract(), false);
        assert_eq!(cpu.registers.f.carry(), false);
        assert_eq!(cpu.registers.f.half_carry(), true);

        cpu.registers.a = 0xF0;
        cpu.registers.f.set_carry(false);
        adc(&mut cpu, 0x10);
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.registers.f.zero(), true);
        assert_eq!(cpu.registers.f.subtract(), false);
        assert_eq!(cpu.registers.f.carry(), true);
        assert_eq!(cpu.registers.f.half_carry(), false);

        cpu.registers.a = 0x7F;
        cpu.registers.f.set_carry(true);
        adc(&mut cpu, 0x01);
        assert_eq!(cpu.registers.a, 0x81);
        assert_eq!(cpu.registers.f.zero(), false);
        assert_eq!(cpu.registers.f.subtract(), false);
        assert_eq!(cpu.registers.f.carry(), false);
        assert_eq!(cpu.registers.f.half_carry(), true);
    }
}
