use jcpuinstructions::{Instruction, Register};

use crate::{alu::ALU, ram::Ram};

pub struct CPU {
    // just some descriptors because we're fancy like that
    pub name: &'static str,
    pub arch: &'static str,
    pub bits: usize,
    pub num_registers: usize,
    // the registers (3 general purpose, IR,IAR,MAR,OUT)
    pub reg_1: u8,
    pub reg_2: u8,
    pub reg_3: u8,
    pub reg_iar: u8,    // address of the next instruction to load into IR
    pub reg_mar: u8,    // memory address register (we should have an MDR but for expediency we won't)
    pub reg_ir: u8,     // instruction register, contains the instruction being executed 
    pub reg_out: u8,    // a bogus output register
    pub alu: ALU
}

impl CPU {
    pub fn new() -> Self {
        Self {
            name: "jCPU",
            arch: "jx8",
            bits: 8,
            num_registers: 3,
            reg_1: 0,
            reg_2: 0,
            reg_3: 0,
            reg_iar: 0,
            reg_ir: 0,
            reg_mar: 0,
            reg_out: 0,
            alu: ALU {
                A: 0,   
                B: 0,  
                
                Or: 0,
                And: 0,
                Not: 0,
                Shl: 0,
                Shr: 0,
                Sum: 0,
                
                Lt: false,
                Eq: false,
                Zero: false,
                C: 0,          // Carry flag
                S: 0,          // Sign flag
            
            }
        }
    }

    pub fn reset(&mut self) {
        self.reg_1 = 0;
        self.reg_2 = 0;
        self.reg_3 = 0;
        self.reg_iar = 0;
        self.reg_mar = 0;
        self.reg_ir = 0;
        self.reg_out = 0;      
    }

    pub fn cycle(&mut self, ram: &mut Ram) -> bool {

        let instruction = self.reg_ir;

        // opcode first 4 bits 
        let opcode = instruction & 0xF0;

        
        // alu
        if (instruction >> 7) == 0b1 {
            let reg_a = (instruction & 0x0C) >> 2;
            let reg_b = (instruction & 0x03);
            

            self.alu.set_a(self.get_register(reg_a));
            self.alu.set_b(self.get_register(reg_b));
            
            if opcode == Instruction::ADD as u8 {
                self.set_register(reg_b, self.alu.op_add())
            } else if opcode == Instruction::SUB as u8 {
                self.set_register(reg_b, self.alu.op_sub())
            } else if opcode == Instruction::PRNT as u8 {
                self.reg_out = self.get_register(reg_a)
            } else {
                panic!("[cpu] unknown instruction")
            }
            
            self.alu.flags();

            self.reg_iar += 1;

        } else {
            let reg_a = (instruction & 0x0C) >> 2;
            let reg_b = (instruction & 0x03);

            if opcode == Instruction::DATA as u8 {
                self.reg_mar += 1;

                self.set_register(reg_a, ram.read(self.reg_mar));

                self.reg_iar += 1;

            } else {
                panic!("[cpu] unknown instruction")
            }

            self.reg_iar += 1;
        }

        true
    }

    fn set_register(&mut self, reg: u8, value: u8) {
        if reg == Register::R1 as u8 {
            self.reg_1 = value;
        } else if reg == Register::R2 as u8 {
            self.reg_2 = value;
        } else if reg == Register::R3 as u8 {
            self.reg_3 = value;
        } else {
            panic!("[cpu] unknown register")
        }
    }

    fn get_register(&self, reg: u8) -> u8 {
        if reg == Register::R1 as u8 {
            self.reg_1
        } else if reg == Register::R2 as u8 {
            self.reg_2
        } else if reg == Register::R3 as u8 {
            self.reg_3
        } else {
            panic!("[cpu] unknown register")
        }
    }
}