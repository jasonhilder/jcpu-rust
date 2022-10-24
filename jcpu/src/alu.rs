use jcpuinstructions::JumpFlag;

const INT: u8          = 0b10000000;  // 0x80
pub const REG_A_ISREG: u8  = 0b01000000;  // 0x40
pub const REG_B_ISREG: u8  = 0b00100000;  // 0x20
const FLAG_LT: u8      = 0b00010000;  // 0x10
const FLAG_EQ: u8      = 0b00001000;  // 0x8
const FLAG_Z: u8       = 0b00000100;  // 0x4
const FLAG_SIGN: u8    = 0b00000010;  // 0x2
const FLAG_CARRY: u8   = 0b00000001;  // 0x1

pub struct ALU {
    pub A: u8,
    pub B: u8,

    pub Or: u8,
    pub And: u8,
    pub Not: u8,
    pub Shl: u8,
    pub Shr: u8,
    pub Sum: u8,
    //   0  0  0  0  0 0 0 0
    // INT|R1|R2|LT|EQ|Z|S|C
    pub flags: u8
}

impl ALU {
    pub fn set_a(&mut self, val: u8) {
        self.A = val
    }

    pub fn set_b(&mut self, val: u8) {
        self.B = val
    }

    pub fn op_add(&mut self) -> u8 {
        let res = self.A as isize + self.B as isize;

        self.check_sign_and_carry(res);

        (res % 255) as u8
    }

    pub fn op_sub(&mut self) -> u8 {
        let mut res = self.A as isize - self.B as isize;

        self.check_sign_and_carry(res);

        if res < 0 {
            res = 0;
        }

        (res % 255) as u8
    }

    pub fn op_inc(&mut self) -> u8 {
        let mut res = self.A as isize + 1;

        self.check_sign_and_carry(res);

        if res < 0 {
            res = 0;
        }

        (res % 255) as u8
    }

    pub fn op_dec(&mut self) -> u8 {
        let mut res = self.A as isize - 1;
        //println!("OPDEC: res[{}]", res);

        self.check_sign_and_carry(res);

        if res < 0 {
            res = 0;
        }

        (res % 255) as u8
    }

    pub fn flags(&mut self) {
        self.Or = self.A | self.B;
        self.And = self.A & self.B;
        self.Not = !self.A;
        self.Shl = self.A << self.B;
        self.Shr = self.A >> self.B;
        //self.Sum = self.A + self.B;

        if self.A < self.B {
            self.flags |= FLAG_Z
        } else {
            self.flags ^= FLAG_CARRY;
        };

        if self.A == self.B {
            self.flags |= FLAG_EQ
        } else {
            self.flags ^= FLAG_EQ
        }

        if self.A < self.B {
            self.flags |= FLAG_LT
        } else {
            self.flags ^= FLAG_LT
        }
    }

    fn check_sign_and_carry(&mut self, num: isize) {
        if num > 127  || num < -128 {
            println!("setting carry");
            self.flags |= FLAG_CARRY
        }

        if num < 0 {
            println!("setting sign");
            self.flags |= FLAG_SIGN
        }
    }


    pub fn match_flags(&mut self, flags: u8) -> bool {

        if flags == JumpFlag::CF as u8 {
            if self.flags & FLAG_CARRY > 0 {
                return true;
            } else {
                return false;
            };
        };

        if flags == JumpFlag::Z as u8 {
            if self.flags & FLAG_Z > 0 {
                return true;
            } else {
                return false;
            };
        };

        false
        // if flags == JumpFlag::E as u8 {};
        // if flags == JumpFlag::EZ as u8 {};
        // if flags == JumpFlag::A as u8 {};
        // if flags == JumpFlag::AZ as u8 {};
        // if flags == JumpFlag::AE as u8 {};
        // if flags == JumpFlag::AEZ as u8 {};
        // if flags == JumpFlag::C as u8 {};
        // if flags == JumpFlag::CZ as u8 {};
        // if flags == JumpFlag::CE as u8 {};
        // if flags == JumpFlag::CEZ as u8 {};
        // if flags == JumpFlag::CA as u8 {};
        // if flags == JumpFlag::CAZ as u8 {};
        // if flags == JumpFlag::CAE as u8 {};
        // if flags == JumpFlag::CAEZ as u8 {};

    }
}
