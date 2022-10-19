use jcpuinstructions::{Instruction, Register};

use crate::{alu::ALU, ram::Ram, motherboard::{BOOT_ADDR, STACK_ADDR}};

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
    pub reg_sp: u8,
    pub reg_int: u8,
    pub alu: ALU,
    pub dbg_msg: String,
    pub clearing: bool,
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
            reg_int: 0,
            reg_sp: STACK_ADDR as u8,
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
            },
            dbg_msg: String::from("CPU started"),
            clearing: false
        }
    }

    pub fn reset(&mut self) {
        self.reg_1 = 0;
        self.reg_2 = 0;
        self.reg_3 = 0;
        self.reg_4 = 0;
        self.reg_iar = 0;
        self.reg_mar = 0;
        self.reg_ir = 0;
        self.reg_out = 0;
        self.reg_int = 0;
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
        self.dbg_msg = String::from("CPU Reset");
        self.clearing = false;
    }

    pub fn cycle(&mut self, ram: &mut Ram) -> bool {
        let instruction = self.reg_ir;

        // non alu instructions
        if (instruction >> 7) == 0b0 {
            let reg_a = (instruction & 0x0C) >> 2;
            let reg_b = instruction & 0x03;
            let flags = instruction & 0b00001111;

            // check for non packed instructions
            if instruction == Instruction::INT as u8 {
                self.dbg_msg = String::from("interrupt found");

                self.reg_mar += 1;

                self.reg_int = ram.read(self.reg_mar);

                self.reg_iar += 1
            } else if instruction == Instruction::CLI as u8 {
                self.dbg_msg = String::from("CLI TIME");

                self.clearing = true;

                self.reg_int = 0
            } else if instruction == Instruction::HLT as u8 {
                self.dbg_msg = String::from("halting");
                return false
            } else {
                self.dbg_msg = format!("instruction found! {}", instruction);
            }

            // opcode first 4 bits
            let opcode = instruction & 0xF0;
            if opcode == Instruction::DATA as u8 {
                self.reg_mar += 1;

                self.set_register(reg_a, ram.read(self.reg_mar));

                self.dbg_msg = format!("Setting reg {} to value {}", (reg_a + 1), ram.read(self.reg_mar));
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
                self.dbg_msg = format!("Jumping to address {}", address);

            } else if opcode == Instruction::JMPR as u8 {
                self.reg_iar = self.get_register(reg_a);

                self.reg_mar = self.reg_iar;
            } else if opcode == Instruction::JMPIF as u8 {
                if self.alu.match_flags(flags) {
                    self.dbg_msg = String::from("Jump if check passed");
                    self.reg_mar += 1;

                    self.dbg_msg = format!("Retrieving address from {}, read({})", self.reg_mar, ram.read(self.reg_mar));

                    self.reg_iar = (BOOT_ADDR as u8) + ram.read(self.reg_mar) - 1;
                }  else {
                    self.dbg_msg = String::from("Jump if check failed");
                    self.reg_iar += 1;
                }
            } else {
                panic!("[cpu] unknown instruction")
            }

            self.reg_iar += 1;
        }

        // alu instructions
        if (instruction >> 7) == 0b1 {
            // opcode first 4 bits
            let opcode = instruction & 0xF0;

            let reg_a = (instruction & 0x0C) >> 2;
            let reg_b = instruction & 0x03;

            self.alu.set_a(self.get_register(reg_a));
            self.alu.set_b(self.get_register(reg_b));

            if opcode == Instruction::ADD as u8 {
                let res = self.alu.op_add();
                self.dbg_msg = format!("Adding reg A and reg B, setting result {} to reg B", {res});
                self.set_register(reg_b, res)
            } else if opcode == Instruction::SUB as u8 {
                let res = self.alu.op_sub();
                self.set_register(reg_b, res)
            } else if opcode == Instruction::CMP as u8 {
                self.dbg_msg = String::from("Comparing reg A and reg B");
                self.alu.A = self.alu.op_sub();
            } else if opcode == Instruction::INC as u8 {
                let res = self.alu.op_inc();
                self.set_register(reg_a, res);
            } else if opcode == Instruction::DEC as u8 {
                let res = self.alu.op_dec();
                self.dbg_msg = format!("Decrementing reg {} to value {}", (reg_a + 1), res);
                self.set_register(reg_a, res);
            } else if opcode == Instruction::PUSH as u8 {
                // SP is after the BIN_SIZE so on boot we plus 1
                // @TODO check for register value or literal value
                if self.reg_sp == 255 {
                   panic!("Stack limit reached!")
                } else {
                    self.reg_sp += 1;
                    let val = self.get_register(reg_a);
                    self.dbg_msg = format!("Setting value {} in reg {} to stack", val, (reg_a + 1));
                    ram.write(self.reg_sp, val)
                }
            } else if opcode == Instruction::POP as u8 {
                let val = ram.read(self.reg_sp);
                self.dbg_msg = format!("Popping value {} in stack to register {}", val, (reg_a + 1));
                self.set_register(reg_a, val);
                self.reg_sp -= 1
            } else {
                panic!("[cpu] unknown instruction")
            }

            self.alu.flags();

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
