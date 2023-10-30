use super::{
    opcode::{self, Opcode},
    registers::Registers,
};

pub struct CPU {
    pub registers: Registers,
    pub pc: u16,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            pc: 0,
        }
    }

    pub fn execute(&mut self, instruction: Opcode) {
        match instruction {
            Opcode::ADD(target) => {
                let value = target.get_value(self);
                opcode::add(self, value);
            }

            Opcode::ADC(target) => {
                let value = target.get_value(self);
                opcode::adc(self, value);
            }
        }
    }
}
