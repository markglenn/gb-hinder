use crate::hardware::cpu::CPU;

pub fn call(cpu: &mut CPU) {
    let address = cpu.next_word();
    println!("CALL ${:04x}", address);

    cpu.push_word(cpu.pc);
    cpu.pc = address;
}
