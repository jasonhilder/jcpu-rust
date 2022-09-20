use jcpuinstructions::{Instruction, Register, JumpFlag};

use crate::{alu::ALU, ram::Ram, motherboard::BOOT_ADDR};

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
    pub reg_4: u8,
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
            reg_4: 0,
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
        self.alu.A =  0;
        self.alu.B = 0;

        self.alu.Or = 0;
        self.alu.And = 0;
        self.alu.Not = 0;
        self.alu.Shl = 0;
        self.alu.Shr = 0;
        self.alu.Sum = 0;

        self.alu.Lt = false;
        self.alu.Eq = false;
        self.alu.Zero = false;
        self.alu.C = 0; 
        self.alu.S = 0;   
    }

    pub fn cycle(&mut self, ram: &mut Ram) -> bool {

        let instruction = self.reg_ir;

        // opcode first 4 bits
        let opcode = instruction & 0xF0;

/*
        1. Add a CMP op to the ALU (compiler and sim) - i.e. op_cmp
        2. the cmp will take two registers and compare them for:
            this op basically does: a - b WITHJOUT setting any registers, JUST the flags
            This is so that we can do: cmp rega, regb and then jump if any of the CPU conditions aare set (not mutative)
        3. add labels to the lexer/parser/compiler that:
            3.1 stores the address (offset in the operations index) of that label
            3.2 when, at any point in the code, we reference a [identifier], then we need to lookup the address tabel and return that value

            e.g. if we do:
                ADD R1, R2

                JMP [print] <-- Here this is compiled as: JMP 0x2

                print:
                OUT R2
 */
        // alu
        if (instruction >> 7) == 0b1 {
            let reg_a = (instruction & 0x0C) >> 2;
            let reg_b = instruction & 0x03;

            self.alu.set_a(self.get_register(reg_a));
            self.alu.set_b(self.get_register(reg_b));

            if opcode == Instruction::ADD as u8 {
                let res = self.alu.op_add();
                self.set_register(reg_b, res)
            } else if opcode == Instruction::SUB as u8 {
                let res = self.alu.op_sub();
                self.set_register(reg_b, res)
            } else if opcode == Instruction::CMP as u8 {
                self.alu.A = self.alu.op_sub();
            } else if opcode == Instruction::INC as u8 {
                let res = self.alu.op_inc();
                self.set_register(reg_a, res);
            } else if opcode == Instruction::DEC as u8 {
                let res = self.alu.op_dec();
                self.set_register(reg_a, res);
            } else {
                panic!("[cpu] unknown instruction")
            }

            self.alu.flags();

            self.reg_iar += 1;

        } else {
            let reg_a = (instruction & 0x0C) >> 2;
            let reg_b = instruction & 0x03;
            let flags = instruction & 0b00001111;


            if opcode == Instruction::DATA as u8 {
                self.reg_mar += 1;

                self.set_register(reg_a, ram.read(self.reg_mar));

                self.reg_iar += 1;
            } else if opcode == Instruction::LD as u8 {
                // set prev to current mar,
                let prev = self.reg_mar;

                // set mar to regA value,
                self.reg_mar = self.get_register(reg_a);

                // ld: load memory from ram at regA address into regB
                self.set_register(reg_b, ram.read(self.reg_mar));

                // set mar back to prev
                self.reg_mar = prev;
            } else if opcode == Instruction::ST as u8 {
                // set prev to current mar,
                let prev = self.reg_mar;

                // set mar to regA value,
                self.reg_mar = self.get_register(reg_a);

                // ld: load value* at regB in regA ram location*
                let register_b = self.get_register(reg_b);

                ram.write(self.reg_mar, register_b);

                // set mar back to prev
                self.reg_mar = prev;
            } else if opcode == Instruction::JMP as u8 {
                self.reg_mar += 1;
                let address = (BOOT_ADDR) as u8 + ram.read(self.reg_mar) - 1;
                self.reg_iar = address; // -1 because end of function increments

            } else if opcode == Instruction::JMPR as u8 {
                self.reg_iar = self.get_register(reg_a);

                self.reg_mar = self.reg_iar;    
            } else if opcode == Instruction::JMPIF as u8 {
                if self.alu.match_flags(flags) {
                    self.reg_mar += 1;

                    self.reg_iar = (BOOT_ADDR as u8) + ram.read(self.reg_mar) - 1;
                }  else {
                    
                    self.reg_iar += 1;
                }
            } else if opcode == Instruction::HLT as u8 {
                return false
            } else {
                panic!("[cpu] unknown instruction")
            }
            //

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
        } else if reg == Register::R4 as u8 {
            self.reg_4 = value;
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
        } else if reg == Register::R4 as u8 {
            self.reg_4
        } else {
            panic!("[cpu] unknown register")
        }
    }
}
