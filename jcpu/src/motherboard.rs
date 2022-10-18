use crate::{ram::Ram, helpers, cpu::CPU, peripheral::Peripheral};

pub const KEYBOARD_ADDRESS: u8 = 0;
pub const KEYBOARD_RAM: u8 = 10;
pub const GPU_RAM: u8 = 3;
const RESERVED_RAM: u8 = 2;
const BIN_SIZE: usize = 10 * 12;
const PERIPHERALS: usize = (KEYBOARD_ADDRESS + KEYBOARD_RAM + GPU_RAM + RESERVED_RAM) as usize;
pub const BOOT_ADDR: usize = PERIPHERALS; // ADDRESS Starts after PERIPHERALS
pub const STACK_ADDR: usize = BIN_SIZE + PERIPHERALS; // Stack starts after binary size and peripherals

pub struct Motherboard<'a> {
    cycle_i: usize,
    pub cpu: CPU,
    pub ram: Ram,
    pub peripherals: Vec<&'a mut dyn Peripheral>,
    bootimg: String,
    instructions: String
}

// Motherboard boots from bootfile
// Send cpu instructions to do as cycles
impl<'a> Motherboard<'a> {
    pub fn new(bootfile: &'a str, instructions: &'a str) -> Motherboard<'a> {
        Motherboard {
            cycle_i: 0,
            cpu: CPU::new(),            // new CPU with 3 general purpose registers
            ram: Ram::new(),         // 256 bytes of ram - STYLING!
            peripherals: Vec::new(),
            bootimg: bootfile.to_string(),
            instructions: instructions.to_string()
        }
    }
    pub fn add_peripheral(&mut self, p: &'a mut impl Peripheral) {
        self.peripherals.push(p);
    }

    pub fn process_peripherals(&mut self) {
        for peripheral in self.peripherals.iter_mut() {
            peripheral.process(&mut self.cpu, &mut self.ram);
        }
    }

    pub fn pass_to_peripheral(&mut self, value: u8) {
        for peripheral in self.peripherals.iter_mut() {
            peripheral.update(value);
        }
    }

    pub fn reset_peripherals(&mut self) {
        for peripheral in self.peripherals.iter_mut() {
            peripheral.clear_state();
        }
        self.cpu.clearing = false;
    }

    pub fn ram_info(&self) -> &[u8] {
        &self.ram.memory
    }

    pub fn mb_info(&self) -> Vec<(String,String)> {
        vec![
            ("Cycle".to_string(), format!("{}",self.cycle_i)),
            ("Boot image size".to_string(), format!("{}",self.bootimg.len())),
            ("Relative address".to_string(), format!("{}", (self.cpu.reg_mar as usize) - BOOT_ADDR))
        ]
    }

    pub fn dbg_info(&self) -> String {
        self.cpu.dbg_msg.clone()
    }

    // if false stop cpu
    pub fn cycle(&mut self) -> bool {
        self.cpu.reg_mar = self.cpu.reg_iar;
        self.cpu.reg_ir = self.ram.read(self.cpu.reg_mar);

        if !self.cpu.cycle(&mut self.ram) {
            return false;
        }

        self.cpu.reg_mar += 1;
        self.cycle_i += 1;

        true
    }

    pub fn cpu_state(&self) -> Vec<(String,String)> {
        vec![
            ("CPU_ID".to_string(), self.cpu.name.to_string()),
            ("Arch".to_string(), self.cpu.arch.to_string()),
            ("Bits".to_string(), self.cpu.bits.to_string()),
            ("Registers".to_string(), self.cpu.num_registers.to_string()),
            ("UI Controls:".to_string(), self.cpu.num_registers.to_string()),
            ("Left mouse".to_string(), "Cycle".to_string()),
            ("Right mouse".to_string(), "Reset".to_string()),
            ("Middle mouse".to_string(), "Exit".to_string()),
        ]
    }

    pub fn cpu_info(&self) -> Vec<(String,String)> {
        vec![
            ("Register 1  ".to_string(), format!("{:02x}",self.cpu.reg_1)),
            ("Register 2  ".to_string(), format!("{:02x}",self.cpu.reg_2)),
            ("Register 3  ".to_string(), format!("{:02x}",self.cpu.reg_3)),
            ("Register 4  ".to_string(), format!("{:02x}",self.cpu.reg_4)),
            ("Register IR ".to_string(), format!("{:02x}",self.cpu.reg_ir)),
            ("Register IAR".to_string(), format!("{:02x}",self.cpu.reg_iar)),
            ("Register MAR".to_string(), format!("{:02x}",self.cpu.reg_mar)),
            ("Register OUT ".to_string(), format!("{:02x}",self.cpu.reg_out)),
            ("Register SP ".to_string(), format!("{:02x}",self.cpu.reg_sp)),
            ("Register INT ".to_string(), format!("{:02x}",self.cpu.reg_int)),
            ("Clearing CLI".to_string(), format!("{}",self.cpu.clearing)),
        ]
    }

    pub fn cpu_instructions(&self) -> Vec<String> {
        helpers::read_instructions_to_vec(&self.instructions)
    }

    pub fn alu_info(&self) -> Vec<(String,String)> {
        vec![
            ("A Register  ".to_string(), format!("{:02x}",self.cpu.alu.A)),
            ("B Register  ".to_string(), format!("{:02x}",self.cpu.alu.B)),
            ("OR Flag  ".to_string(), format!("{:02x}",self.cpu.alu.Or)),
            ("AND Flag  ".to_string(), format!("{:02x}",self.cpu.alu.And)),
            ("NOT Flag  ".to_string(), format!("{:02x}",self.cpu.alu.Not)),
            ("SHIFT LEFT Flag  ".to_string(), format!("{:02x}",self.cpu.alu.Shl)),
            ("SHIFT RIGHT Flag  ".to_string(), format!("{:02x}",self.cpu.alu.Shr)),
            ("LESS THAN Flag  ".to_string(), if self.cpu.alu.Lt {"1".to_string()} else {"0".to_string()}),
            ("EQUAL TO Flag  ".to_string(), if self.cpu.alu.Eq {"1".to_string()} else {"0".to_string()}),
            ("ZERO Flag  ".to_string(), if self.cpu.alu.Zero {"1".to_string()} else {"0".to_string()}),
            ("CARRY Flag  ".to_string(), format!("{:?}",self.cpu.alu.C)),
            ("Sign Flag  ".to_string(), format!("{:?}",self.cpu.alu.S)),
        ]
    }

    pub fn boot(&mut self) {
        let boot_content = helpers::read_bin_vec(&self.bootimg);

        self.cpu.dbg_msg = format!("bin size: {:?}", &boot_content.len());
        if boot_content.len() > BIN_SIZE {
            panic!("Compiled binary too large.")
        }
        self.ram.fill(BOOT_ADDR as u8, boot_content);
        self.cpu.reg_mar = BOOT_ADDR as u8;
        self.cpu.reg_iar = self.cpu.reg_mar;
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.ram.reset();
        self.cycle_i = 0;
        self.boot()
    }
}
