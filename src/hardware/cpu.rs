use super::{bus::Bus, opcode::Opcode, registers::Registers, Memory};
use crate::hardware::opcode::execute_opcode;

pub struct CPU {
    // Standard registers
    pub registers: Registers,

    // Program counter
    pub pc: u16,

    // Stack pointer
    pub sp: u16,

    // Interrupt master enable
    pub ime: bool,

    // Halt flag
    pub halted: bool,

    // Address bus
    pub bus: Bus,

    // These are used to delay the enabling/disabling of interrupts
    pub interrupt_enable_counter: u8,
    pub interrupt_disable_counter: u8,

    // This is used to print out the state of the CPU after each instruction
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
            debug: false,
            interrupt_enable_counter: 0,
            interrupt_disable_counter: 0,
        }
    }

    pub fn execute_next_instruction(&mut self) -> u8 {
        self.update_ime();
        self.handleinterrupt();

        if !self.debug {
            // If we're not in debug mode, just execute the next instruction
            let opcode = Opcode::from_byte(self.next_byte());

            execute_opcode(self, opcode)
        } else {
            let original_pc = self.pc;
            let m0 = self.peek_byte_at_offset(0);
            let m1 = self.peek_byte_at_offset(1);
            let m2 = self.peek_byte_at_offset(2);
            let m3 = self.peek_byte_at_offset(3);

            let op = self.next_byte();

            let opcode = Opcode::from_byte(op);

            println!(
                "{} SP:{:04X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}",
                self.registers, self.sp, original_pc, m0, m1, m2, m3
            );

            execute_opcode(self, opcode)
        }
    }

    fn update_ime(&mut self) {
        self.interrupt_enable_counter = match self.interrupt_enable_counter {
            2 => 1,
            1 => {
                self.ime = true;
                0
            }
            _ => 0,
        };

        self.interrupt_disable_counter = match self.interrupt_disable_counter {
            2 => 1,
            1 => {
                self.ime = false;
                0
            }
            _ => 0,
        };
    }

    fn handleinterrupt(&mut self) -> u32 {
        if self.ime == false && self.halted == false {
            return 0;
        }

        let triggered = self.bus.interrupt_enable & self.bus.interrupt_flags;
        if triggered == 0 {
            return 0;
        }

        self.halted = false;
        if self.ime == false {
            return 0;
        }
        self.ime = false;

        let n = triggered.trailing_zeros() as u16;
        if n >= 5 {
            panic!("Invalid interrupt triggered");
        }

        // Disable the handled interrupt
        self.bus.interrupt_flags &= !(1 << n);

        self.push_word(self.pc);
        self.pc = 0x0040 | (n << 3);

        return 4;
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
