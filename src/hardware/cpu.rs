use crate::hardware::opcode::execute_opcode;

use super::{bus::Bus, opcode::Opcode, registers::Registers, Memory};

pub struct CPU {
    pub registers: Registers,
    pub pc: u16,
    pub sp: u16,

    pub bus: Bus,
}

impl CPU {
    pub fn new(bus: Bus) -> CPU {
        CPU {
            registers: Registers::new(),
            pc: 0,
            sp: 0xFFFE,
            bus,
        }
    }

    pub fn execute_next_instruction(&mut self) -> u8 {
        let original_pc = self.pc;
        let op = self.next_byte();

        let opcode = Opcode::from_byte(op);
        println!("0x{:04X} - 0x{:02X} {:?}", original_pc, op, opcode);
        execute_opcode(self, opcode)
    }

    // Reads the next byte and increments the program counter
    pub fn next_byte(&mut self) -> u8 {
        let byte = self.bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);

        byte
    }

    pub fn next_word(&mut self) -> u16 {
        let low = self.next_byte() as u16;
        let high = self.next_byte() as u16;

        low | (high << 8)
    }

    pub fn push_byte(&mut self, value: u8) {
        let mut sp = self.sp;

        sp -= 1;

        self.sp = sp;
        self.bus.write(sp, value);
    }

    pub fn push_word(&mut self, value: u16) {
        self.push_byte((value >> 8) as u8);
        self.push_byte(value as u8);
    }
}
