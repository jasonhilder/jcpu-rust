
#[derive(Clone, Debug)]
pub enum JumpFlag {
    // Where CPU flags: C (carry), A (a > than), E (a equal to), Z (a = 0)
    CF  = 0b0000,  // clear flag
    Z   = 0b0001,   // zero
    E   = 0b0010,   // a=b
    EZ  = 0b0011,   // a=b or a=0
    A   = 0b0100,   // a > b
    AZ  = 0b0101,   // a > b or 0
    AE  = 0b0110,   // a = b
    AEZ = 0b0111,   // a = b or 0
    C   = 0b1000,   // jump carry
    CZ  = 0b1001,   //
    CE  = 0b1010,   //
    CEZ = 0b1011,   //
    CA  = 0b1100,   //
    CAZ = 0b1101,   //
    CAE = 0b1110,   //
    CAEZ= 0b1111,   //
}

pub static JUMP_FLAGS: [&str; 16] = [
  "cf", "z", "e", "ez", "a", "az", "ae", "aez", "c", "cz", "ce", "cez", "ca", "caz", "cae", "caez"
];

// These are the machine language codes for the ALU instructions
#[repr(u8)]
#[derive(Clone, Debug)]
pub enum Instruction {
    // ALU instructions are [1][OPC][RA][RB] where opcode is 3 bits, RA and RB are 2 bits
    // OP contents of RA and RB and put into RB
    // OP RA, RB
    ADD  = 0b10000000,
    SUB  = 0b10010000,

    // COMPARISON
    CMP = 0b11000000,

    // INCREMENT AND DECREMENT
    INC = 0b11010000,
    DEC = 0b10110000,

    // Load into RB from RAM address in RA
    // LD RA, RB
    LD   = 0b00000000,
    // Store contents of RB to RAM addresss specified in RA
    // ST RA, RB
    ST   = 0b00010000,
    // DATA RB, xxxxxxxx
    // Load 8 bits from the next RAM address into RB
    // Note: we say next RAM address  because while we specify it here in assembly, we
    // only have 8 bits to perform an op, so it has to perform another fetch cycle to
    // retrieve this data which will be stored in the next RAM address
    DATA = 0b00100000,

    // Jump to address in RB
    // JMPR RB
    JMPR  = 0b00110000,
    // Jump to the next byte in ram
    // JMP ADDR
    JMP   = 0b01000000,
    // Jump if (flag is set)
    // JMP**** ADDR
    // Where the last 4 bits indicate teh C,A,E,Z flag
    // e.g. JMPA 0x01 ; jump to address 0x01 if the A flag is set
    JMPIF = 0b01010000,
    CLF   = 0b01100000,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Register {
    R1 = 0b00,
    R2 = 0b01,
    R3 = 0b10,
}
