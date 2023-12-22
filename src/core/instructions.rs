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
    LDNNA(u16)
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub (super) enum ArithmeticTarget {
    A, B, C, D, E, H, L
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub(super) enum ArithmeticTarget16 {
    BC, DE, HL
}
