use strum::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub(crate) enum Instruction {
    // Adds to a, value from register R. Sets flags.
    AddRegister(RegisterTarget),
    // Adds to a, value from address specified by Hl. Sets flags.
    AddIndirectHl,
    // Adds to a, the value N (from pc). Sets flags.
    AddN,
    // Adds to a, value from register R and carry. Sets flags.
    AddCarryRegister(RegisterTarget),
    // Adds to a, value from address specified by Hl and carry. Sets flags.
    AdcIndirectHl,
    // Adds to a, the value N (from pc) and carry. Sets flags.
    AddCarryN,
    // SUB(ArithmeticTarget)
    // Loads data from the second register to the first
    LoadRegisterRegister(RegisterTarget, RegisterTarget),
    // Loads value N (from pc) into register R
    LoadRegisterN(RegisterTarget),
    // Loads to R, data from the absolute address specified by Hl
    LoadRegisterIndirectHl(RegisterTarget),
    // Loads to the absolute address specified by Hl, data from register R
    LoadIndirectHlRegister(RegisterTarget),
    // Loads to the absolute address specified by Hl, the value N (from pc)
    LoadIndirectHlN,
    // Loads to the register A, data from the absolute address specified by BC
    LoadAIndirectBc,
    // Loads to the register A, data from the absolute address specified by DE
    LoadAIndirectDe,
    // Loads to the address specified by BC, data from A
    LoadIndirectBcA,
    // Loads to the address specified by DE, data from A
    LoadIndirectDeA,
    // Loads to the register A, data from the absolute address NN (from pc)
    LoadANn,
    // Loads to the absolute address NN (from pc), data from A
    LoadNnA,
    // Loads to A, data from the absolute address 0xFF00 + C
    LoadHalfAC,
    // Loads to the absolute address 0xFF00 + C, data from A
    LoadHalfCA,
    // Loads to A, data from the absolute address 0xFF00 + N (from pc)
    LoadHalfAN,
    // Loads to the absolute address 0xFF00 + N (from pc), data from A
    LoadHalfNA,
    // Loads to A, data from the absolute address HL, then decrements HL by 1
    LoadAIndirectHlDecrement,
    // Loads to the absolute address HL, data from A, then decrements HL
    LoadIndirectHlDecrementA,
    // Loads to A, data from the absolute address HL, then increments HL by 1
    LoadAIndirectHlIncrement,
    // Loads to the absolute address HL, data from A, then decrements HL
    LoadIndirectHlIncrementA,
    // Loads the immediate 16 bit data NN into the 16 bit register RR
    LoadRegister16Nn(RegisterTarget16),
    // Unconditional jump to the nn address (indirect pc)
    JumpToNn,
    // Jumps to the nn address (indirect pc) if the JumpCondition is satisfied
    JumpConditionalToNn(JumpCondition)
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

