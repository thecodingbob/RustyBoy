use strum::EnumIter;
pub (super) enum Instruction {
    ADD(ArithmeticTarget),
    ADDHL(ArithmeticTarget16),
    ADC(ArithmeticTarget),
    //SUB(ArithmeticTarget)
    // Loads data from the second register to the first
    LDR(ArithmeticTarget, ArithmeticTarget),
    // Loads value N into register R
    LDRN(ArithmeticTarget, u8),
    // Loads to R, data from the absolute address specified by HL
    LDRHL(ArithmeticTarget),
    // Loads to the absolute address specified by HL, data from register R
    LDHLR(ArithmeticTarget),
    // Loads to the absolute address specified by HL, the value N
    LDHLN(u8),
    // Loads to the register A, data from the absolute address specified by BC
    LDABC,
    // Loads to the register A, data from the absolute address specified by DE
    LDADE,
    // Loads to the address specified by BC, data from A
    LDBCA,
    // Loads to the address specified by DE, data from A
    LDDEA,
    // Loads to the register A, data from the absolute address NN
    LDANN(u16),
    // Loads to the absolute address NN, data from A
    LDNNA(u16),
    // Loads to A, data from the absolute address 0xFF00 + C
    LDHAC,
    // Loads to the absolute address 0xFF00 + C, data from A
    LDHCA,
    // Loads to A, data from the absolute address 0xFF00 + N
    LDHAN(u8),
    // Loads to the absolute address 0xFF00 + N, data from A
    LDHNA(u8),
    // Loads to A, data from the absolute address HL, then decrements HL by 1
    LDAHLDEC,
    // Loads to the absolute address HL, data from A, then decrements HL
    LDHLDECA,
    // Loads to A, data from the absolute address HL, then increments HL by 1
    LDAHLINC,
    // Loads to the absolute address HL, data from A, then decrements HL
    LDHLINCA,
    // Jumps if the JumpCondition is satisfied
    JP(JumpCondition)
}

impl Instruction {
    pub (super) fn from_byte(byte: u8, is_prefixed: bool) -> Option<Instruction>{
        match byte {
            _ => None
        }
    }

    pub (super) fn from_byte_prefixed(byte: u8) -> Option<Instruction>{
        match byte {
            _ => None
        }
    }

    pub (super) fn from_byte_not_prefixed(byte: u8) -> Option<Instruction>{
        match byte {
            _ => None
        }
    }
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub (super) enum ArithmeticTarget {
    A, B, C, D, E, H, L
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub(super) enum ArithmeticTarget16 {
    BC, DE, HL
}

#[derive(Debug)]
pub (super) enum JumpCondition {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always
}
