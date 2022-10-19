use crate::{cpu::CPU, ram::Ram, motherboard::{KEYBOARD_ADDRESS, SCREEN_WIDTH, SCREEN_HEIGHT}};

const KEYBOARD_ID: &str = "keyboard";
const SCREEN_ID: &str = "screen";
//@NOTE: PERIPHERAL ORDER, 1 = screen, 2 = keyboard, 3 = not used

pub trait PeripheralTrait {
    fn get_id(&mut self) -> &str;
    fn create(&mut self);
    fn clear_state(&mut self);
    fn process(&mut self, cpu: &mut CPU, ram: &mut Ram) {}
    fn update(&mut self, value: u8) {}
}

pub enum Peripheral {
    Screen(Screen),
    Keyboard(Keyboard),
}


const MAX_BUFFERED_KEYS: u8 = 10;
pub struct Keyboard {
    pub keys_pressed: Vec<u8>,
}

pub fn get_key_code(c: char) -> u8 {
    c as u8 - 32
}

impl PeripheralTrait for Keyboard {
    fn get_id(&mut self) -> &str {
        KEYBOARD_ID
    }

    fn create(&mut self)  {
        // Keyboard { pressed_state: false, keycode: None }
        // @FIXME: we want to establish any buffer defaults etc. here, not actually create the peripheral
    }

    fn process(&mut self, cpu: &mut CPU, ram: &mut Ram) {
        if cpu.reg_int > 0 && cpu.reg_int == 2 {
            if self.keys_pressed.len() > 0 {
                for i in 0..self.keys_pressed.len() {
                    ram.write(KEYBOARD_ADDRESS + i as u8, self.keys_pressed[i]); // write to ram
                }

                cpu.reg_1 = KEYBOARD_ADDRESS;
                cpu.reg_2 = self.keys_pressed.len() as u8;
            }
        }

    }

    fn update(&mut self, value:u8) {
        if self.keys_pressed.len() < MAX_BUFFERED_KEYS.into() {
            self.keys_pressed.push(value);
        }
    }

    fn clear_state(&mut self) {
        // @TODO: rememebr to clear the keyboard ram of keys
        self.keys_pressed = vec![];
    }
}

pub struct Screen {
    pub buffer: [u8; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize],
}

impl Screen {
    pub fn get_buffer(&mut self) -> Vec<u8> {
        self.buffer.clone().into()
    }
}

impl PeripheralTrait for Screen {
    fn get_id(&mut self) -> &str {
        KEYBOARD_ID
    }

    fn create(&mut self) {
        //println!("todo")
    }

    fn process(&mut self, cpu: &mut CPU, ram: &mut Ram) {
        if cpu.reg_int > 0 && cpu.reg_int == 1 {
            // get x
            let x = cpu.reg_1;
            // get y
            let y = cpu.reg_2;
            // get color
            let c = cpu.reg_3;

            let pos = x + (SCREEN_WIDTH * y);
            // add it to the "screen" buffer
            self.buffer[pos as usize] = c;
        }
    }

    fn update(&mut self, value: u8) {
        //
    }

    fn clear_state(&mut self) {
        // print
    }
}
