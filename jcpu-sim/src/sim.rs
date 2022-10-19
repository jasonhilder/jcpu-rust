use jcpu::{motherboard::Motherboard};
/*

The basic process here is that we have a motherboard that will power up, reserve some memory for itself (for
whatever reason), then it will load up the boot record and jump the CPU to begin executing instructions.
The kernel.img file must be a binary file that contains machine code of our instructions.

For each cycle of the motherboard, it executes a cycle on the CPU.

*/

pub struct Sim {
    pub mb: Motherboard,
}

impl Sim {
    pub fn new() -> Self {
        Self {
            // our board and CPU are 8 bits and we want to reserve 10 bytes of ram for ourselves
            mb: Motherboard::new("./boot.img", "./instructions.d")
        }
    }
    // The next four functions are to display the data from the motherboard and CPU
    pub fn get_cpu_info(&mut self) -> Vec<(String,String)> {
        self.mb.cpu_state()
    }
    pub fn get_mb_info(&mut self) -> Vec<(String,String)> {
        self.mb.mb_info()
    }
    pub fn get_screen_info(&mut self) {
        for p in self.mb.peripherals.iter() {
            
        }
    }
    pub fn get_cpu_details(&mut self) -> Vec<(String,String)> {
        self.mb.cpu_info()
    }
    pub fn get_cpu_instructions_text(&mut self) -> Vec<String> {
        self.mb.cpu_instructions()
    }
    pub fn get_alu_details(&mut self) -> Vec<(String,String)> {
        self.mb.alu_info()
    }
    pub fn get_ram_info(&mut self) -> Vec<u8> {
        self.mb.ram_info().to_vec()
    }
    pub fn get_dbg_info(&mut self) -> String {
        self.mb.dbg_info()
    }
    pub fn start(&mut self) {
        self.mb.boot();
    }
    pub fn cycle(&mut self) -> bool {
        self.mb.cycle()
    }
    pub fn reset(&mut self) {
        self.mb.reset();
    }
}
