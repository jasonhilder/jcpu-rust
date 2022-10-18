pub struct Ram {
    pub memory: [u8; 255]
}

impl Ram {
    pub fn new() -> Self {
        Self {
            memory: [0; 255]
        }
    }
    pub fn read(&self, address: u8) -> u8 {
        self.memory[address as usize]
    }

    pub fn write(&mut self, address: u8, data: u8) {
        self.memory[address as usize] = data
    }

    pub fn fill(&mut self, address: u8, bytes: Vec<u8>) {

        for bite in address as usize ..address as usize + bytes.len() {
            self.memory[bite] = bytes[bite - address as usize ];
        }
    }

    pub fn reset(&mut self) {
        self.memory = [0; 255]
    }
}
