use crate::hardware::cpu::CPU;

pub fn enable_interrupt(cpu: &mut CPU, enabled: bool) {
    if enabled {
        cpu.interrupt_enable_counter = 2;
    } else {
        cpu.interrupt_disable_counter = 2;
    }
}
