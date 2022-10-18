use crate::{cpu::CPU, ram::Ram, motherboard::KEYBOARD_RAM};

pub trait Peripheral {
    fn create(&mut self);
    fn clear_state(&mut self);
    fn process(&mut self, cpu: &mut CPU, ram: &mut Ram) {}
    fn update(&mut self, value: u8) {}
}

const MAX_BUFFERED_KEYS: u8 = 10;
pub struct Keyboard {
    pub pressed_state: bool,
    pub keys_pressed: Vec<u8>,
    pub keycode: Option<u8>,
    pub active: bool
}

pub fn get_key_code(c: char) -> u8 {
    match c {
        'a' => 65,
        'b' => 66,
        'c' => 67,
        'd' => 68,
        'e' => 69,
        'f' => 70,
        'g' => 71,
        'h' => 72,
        'i' => 73,
        'j' => 74,
        'k' => 75,
        'l' => 76,
        'm' => 77,
        'n' => 78,
        'o' => 79,
        'p' => 80,
        'q' => 81,
        'r' => 82,
        's' => 83,
        't' => 84,
        'u' => 85,
        'v' => 86,
        'w' => 87,
        'x' => 88,
        'y' => 89,
        'z' => 90,
        _ => 0
    }
}

impl Peripheral for Keyboard {
    // Return instance of keyboard which
    // can be refered to as a Peripheral
    fn create(&mut self)  {
        // Keyboard { pressed_state: false, keycode: None }
        // @FIXME: we want to establish any buffer defaults etc. here, not actually create the peripheral
    }

    fn process(&mut self, cpu: &mut CPU, ram: &mut Ram) {
        // do any clears updates to internal buffers here
        // get keycode here
        // 1 byte code value  1 byte state

        if cpu.interupt_enabled {
            if self.keys_pressed.len() > 0 {
                // R1 <- store the address that we're dumping the key pressed into in RAM
                // R2 <- store the number of key presses that we're dumping into ram
                let mut keyboard_add:u8 = 0;

                for i in 0..self.keys_pressed.len() {
                    ram.write(keyboard_add, self.keys_pressed[i]); // write to ram
                    keyboard_add += 1
                }

                cpu.reg_1 = keyboard_add;
                cpu.reg_2 = self.keys_pressed.len() as u8;
            }

            // still not too sure about how to proceed ops
            cpu.interupt_enabled = false;
        }

    }

    fn update(&mut self, value:u8) {
        // if backspace remove
        if value == 19 {
            self.keys_pressed.pop();
        } else if self.keys_pressed.len() < MAX_BUFFERED_KEYS.into() {
            self.keys_pressed.push(value);
        }
    }

    fn clear_state(&mut self) {
        // Clear the state to the default
        // WHEN CPU is CLI (clear interrrupts)
        // @TODO: rememebr to clear the keyboard ram of keys
    }
}

pub struct Screen {
    pub buffer: Vec<u8>,
    pub active: bool
    // buffer?
}

impl Peripheral for Screen {
    fn create(&mut self) {
        //println!("todo")
    }

    fn process(&mut self, cpu: &mut CPU, ram: &mut Ram) {
        if cpu.interupt_enabled {
            // get x
            let x = cpu.reg_1;
            // get y
            let y = cpu.reg_2;
            // get color
            let c = cpu.reg_3;
        }
    }

    fn update(&mut self, value: u8) {
        //
    }

    fn clear_state(&mut self) {
        // print
    }
}
