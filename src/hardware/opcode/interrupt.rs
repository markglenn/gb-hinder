use crate::hardware::cpu::CPU;

pub fn enable_interrupt(cpu: &mut CPU, enabled: bool) {
    cpu.ime = enabled;
}
