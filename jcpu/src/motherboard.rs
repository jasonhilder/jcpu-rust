use crate::{ram::{self, Ram}, helpers, cpu::CPU};

const VRAM_ADDR: usize = 0x00;
pub const BOOT_ADDR: usize = VRAM_ADDR + 0x40; // ADDRESS Starts after VGA BUFFER

pub struct Motherboard {
    cycle_i: usize,
    pub cpu: CPU,
    pub ram: Ram,
    bootimg: String,
}

// Motherboard boots from bootfile
// Send cpu instructions to do as cycles
impl Motherboard {
    pub fn new(bootfile: &str) -> Motherboard {
        Motherboard {
            cycle_i: 0,
            cpu: CPU::new(),            // new CPU with 3 general purpose registers
            ram: Ram::new(),         // 256 bytes of ram - STYLING!
            bootimg: bootfile.to_string(),
        }
    }

    pub fn ram_info(&self) -> &[u8] {
        &self.ram.memory
    }

    pub fn mb_info(&self) -> Vec<(String,String)> {
        vec![
            ("Cycle".to_string(), format!("{}",self.cycle_i)),
            ("Boot image size".to_string(), format!("{}",self.bootimg.len())),
        ]
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
        ]
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
        self.ram.fill(BOOT_ADDR as u8, boot_content);
        self.cpu.reg_mar = BOOT_ADDR as u8 + 1; // +1 to skip boot signature
        self.cpu.reg_iar = self.cpu.reg_mar;
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.ram.reset();
        self.cycle_i = 0;
        self.boot()
    }
}
