use crate::{ram::Ram, helpers, cpu::CPU};

const VRAM_SIZE: usize = 8 * 8;
const VRAM_ADDR: usize = 0x00;
const BIN_SIZE: usize = 10 * 12;
const PERIPHERALS: usize = 3; // usize for the additon below, excessive I know...
pub const BOOT_ADDR: usize = VRAM_ADDR + VRAM_SIZE; // ADDRESS Starts after VGA BUFFER
pub const STACK_ADDR: usize = BIN_SIZE + PERIPHERALS; // Stack starts after binary size and peripherals

pub trait Peripheral {
    fn create(&mut self);
    fn process(&self, cpu: &mut CPU) {}
    fn postprocess(&mut self);
}

pub struct Keyboard {
    pub pressed_state: bool,
    pub keycode: Option<u8>,
}

impl Peripheral for Keyboard {
    // Return instance of keyboard which
    // can be refered to as a Peripheral
    fn create(&mut self)  {
        // Keyboard { pressed_state: false, keycode: None }
        // @FIXME: we want to establish any buffer defaults etc. here, not actually create the peripheral 
    }
    fn postprocess(&mut self) {
        // do any clears updates to internal buffers here    
        self.keycode = None; // testing mutability
    }

    fn process(&self, cpu: &mut CPU) {
        // get keycode here
        // 1 byte code value  1 byte state
        if cpu.interupt_enabled {
            // send the keycode
           cpu.dbg_msg = String::from("Keyboard key pressed!");
        }

        self.postprocess();
    }
}
// struct<'a> Motherboard<'a> {
//     peripherals: Vec<&dyn Peripheral<'a>>,
// }
pub struct Motherboard<'a> {
    cycle_i: usize,
    pub cpu: CPU,
    pub ram: Ram,
    pub peripherals: Vec<&'a dyn Peripheral>,
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
    pub fn add_peripheral(&mut self, p: &'a impl Peripheral) {
        self.peripherals.push(p);
    }

    pub fn process_peripherals(&mut self) {
        
        for peripheral in &self.peripherals {
            peripheral.process(&mut self.cpu);
        }
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
        if !self.cpu.interupt_enabled {
            self.cpu.reg_mar = self.cpu.reg_iar;
            self.cpu.reg_ir = self.ram.read(self.cpu.reg_mar);

            if !self.cpu.cycle(&mut self.ram) {
                return false;
            }

            self.cpu.reg_mar += 1;
            self.cycle_i += 1;
        }

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
            ("Register SP ".to_string(), format!("{:02x}",self.cpu.reg_sp)),
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
/*

cycle 0:
    PUSH R1
    INT 1


    ..
    POP R1
    // cpu.interrupt_enable = true
cycle 1:
    // if cpu.interrupt_enable {
        self.cpu.reg_1 =
    }

    R1 -> holds the key pressed or released and the state of the key


struct GPU {
    buffer: Vec<u8>
}
impl Peripheral for GPU {
    fn create() {
        // init buffers etc. and set your states here
    }

    fn process(&mut self, &mut ram: Ram );
        if cpu.interrupt_enabled {
            //
            keystate: u8;


            event {
                keys[getKeycode()] = keystate | getKeyCode();
            }

            cpu.reg_1 = key;
        }
    }
}
*/
