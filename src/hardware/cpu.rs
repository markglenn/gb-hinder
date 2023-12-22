use crate::hardware::opcode::execute_opcode;

use super::{bus::Bus, opcode::Opcode, registers::Registers, Memory};

pub struct CPU {
    pub registers: Registers,
    pub pc: u16,
    pub sp: u16,
    pub ime: bool,

    pub halted: bool,

    pub bus: Bus,
    pub instr_count: usize,
    debug: bool,
}

impl CPU {
    pub fn new(bus: Bus) -> CPU {
        CPU {
            registers: Registers::new(),
            pc: 0x100,
            sp: 0xFFFE,
            bus,
            ime: false,
            halted: false,
            instr_count: 0,
            debug: true,
        }
    }

    pub fn execute_next_instruction(&mut self) -> u8 {
        let original_pc = self.pc;
        let m0 = self.peek_byte_at_offset(0);
        let m1 = self.peek_byte_at_offset(1);
        let m2 = self.peek_byte_at_offset(2);
        let m3 = self.peek_byte_at_offset(3);

        let op = self.next_byte();

        let opcode = Opcode::from_byte(op);

        if self.debug {
            println!(
                "{} SP:{:04X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}",
                self.registers, self.sp, original_pc, m0, m1, m2, m3
            );
        }

        self.instr_count += 1;

        if self.instr_count == 100000 {
            // panic!("100 instructions executed")
        }

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

    pub fn peek_byte(&self) -> u8 {
        self.bus.read(self.pc)
    }

    pub fn peek_byte_at_offset(&self, offset: u16) -> u8 {
        self.bus.read(self.pc + offset)
    }

    pub fn peek_word(&self) -> u16 {
        let low = self.bus.read(self.pc) as u16;
        let high = self.bus.read(self.pc + 1) as u16;

        low | (high << 8)
    }

    pub fn peek_double(&self) -> u32 {
        let low = self.bus.read(self.pc) as u32;
        let high = self.bus.read(self.pc + 1) as u32;
        let low2 = self.bus.read(self.pc + 2) as u32;
        let high2 = self.bus.read(self.pc + 3) as u32;

        low | (high << 8) | (low2 << 16) | (high2 << 24)
    }

    pub fn push_byte(&mut self, value: u8) {
        let mut sp = self.sp;

        sp = sp.wrapping_sub(1);

        self.sp = sp;
        self.bus.write(sp, value);
    }

    pub fn pop_byte(&mut self) -> u8 {
        let sp = self.sp;
        let value = self.bus.read(sp);

        self.sp = sp.wrapping_add(1);

        value
    }

    pub fn push_word(&mut self, value: u16) {
        self.push_byte((value >> 8) as u8);
        self.push_byte(value as u8);
    }

    pub fn pop_word(&mut self) -> u16 {
        let low = self.pop_byte() as u16;
        let high = self.pop_byte() as u16;

        low | (high << 8)
    }

    pub fn set_halted(&mut self, halted: bool) {
        self.halted = halted;
    }

    pub fn stop(&mut self) {
        panic!("STOP instruction executed");
    }
}
