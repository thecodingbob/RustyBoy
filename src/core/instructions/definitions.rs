use strum::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub(crate) enum Instruction {
    // Adds to a, value from register R. Sets flags.
    ADDR(RegisterTarget),
    // Adds to a, value from address specified by HL. Sets flags.
    ADDHL,
    // Adds to a, the value N (from pc). Sets flags.
    ADDN,
    // Adds to a, value from register R and carry. Sets flags.
    ADCR(RegisterTarget),
    // Adds to a, value from address specified by HL and carry. Sets flags.
    ADCHL,
    // Adds to a, the value N (from pc) and carry. Sets flags.
    ADCN,
    // SUB(ArithmeticTarget)
    // Loads data from the second register to the first
    LDRR(RegisterTarget, RegisterTarget),
    // Loads value N (from pc) into register R
    LDRN(RegisterTarget),
    // Loads to R, data from the absolute address specified by HL
    LDRHL(RegisterTarget),
    // Loads to the absolute address specified by HL, data from register R
    LDHLR(RegisterTarget),
    // Loads to the absolute address specified by HL, the value N (from pc)
    LDHLN,
    // Loads to the register A, data from the absolute address specified by BC
    LDABC,
    // Loads to the register A, data from the absolute address specified by DE
    LDADE,
    // Loads to the address specified by BC, data from A
    LDBCA,
    // Loads to the address specified by DE, data from A
    LDDEA,
    // Loads to the register A, data from the absolute address NN (from pc)
    LDANN,
    // Loads to the absolute address NN (from pc), data from A
    LDNNA,
    // Loads to A, data from the absolute address 0xFF00 + C
    LDHAC,
    // Loads to the absolute address 0xFF00 + C, data from A
    LDHCA,
    // Loads to A, data from the absolute address 0xFF00 + N (from pc)
    LDHAN,
    // Loads to the absolute address 0xFF00 + N (from pc), data from A
    LDHNA,
    // Loads to A, data from the absolute address HL, then decrements HL by 1
    LDAHLDEC,
    // Loads to the absolute address HL, data from A, then decrements HL
    LDHLDECA,
    // Loads to A, data from the absolute address HL, then increments HL by 1
    LDAHLINC,
    // Loads to the absolute address HL, data from A, then decrements HL
    LDHLINCA,
    // Loads the immediate 16 bit data NN into the 16 bit register RR
    LDRRNN(RegisterTarget16),
    // Unconditional jump to the nn address (indirect pc)
    JPNN,
    // Jumps to the nn address (indirect pc) if the JumpCondition is satisfied
    JPCCNN(JumpCondition)
}

#[derive(Debug, Clone, Copy, EnumIter, PartialEq, Hash)]
pub (crate) enum RegisterTarget {
    A, B, C, D, E, H, L
}

#[derive(Debug, Clone, Copy, EnumIter, PartialEq, Hash)]
pub(crate) enum RegisterTarget16 {
    BC, DE, HL
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub (crate) enum JumpCondition {
    NotZero,
    Zero,
    NotCarry,
    Carry
}

