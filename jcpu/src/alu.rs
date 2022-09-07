pub struct ALU {
    pub A: u8,   
    pub B: u8,

    
    pub Or: u8,
    pub And: u8,
    pub Not: u8,
    pub Shl: u8,
    pub Shr: u8,
    pub Sum: u8,
    

    pub Lt: bool,
    pub Eq: bool,
    pub Zero: bool,
    pub Ci: u8,          // Carry in
    pub Co: u8,          // Carry out
}

impl ALU {
    pub fn set_a(&mut self, val: u8) {
        self.A = val
    }

    pub fn set_b(&mut self, val: u8) {
        self.B = val
    }

    pub fn op_add(&self) -> u8 {
        self.A + self.B 
    }

    pub fn op_sub(&self) -> u8 {
        self.A - self.B 
    }

    pub fn flags(&mut self) {
        self.Or = self.A | self.B;
        self.And = self.A & self.B;
        self.Not = !self.A;
        self.Shl = self.A << self.B;
        self.Shr = self.A >> self.B;
        self.Sum = self.A + self.B;

        self.Lt = if self.A < self.B { true } else { false };
        self.Eq = if self.A == self.B { true } else { false };
        self.Zero = if self.A == 0 { true } else { false };
    }

}