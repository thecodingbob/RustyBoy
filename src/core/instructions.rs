use strum::EnumIter;
pub (super) enum Instruction {
    ADD(ArithmeticTarget),
    ADDHL(ArithmeticTarget16),
    ADC(ArithmeticTarget),
    //SUB(ArithmeticTarget)
    LDR(ArithmeticTarget, ArithmeticTarget),
    LDN(ArithmeticTarget),
    LDRHL(ArithmeticTarget),
    LDHLR(ArithmeticTarget)
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub (super) enum ArithmeticTarget {
    A, B, C, D, E, H, L
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub(super) enum ArithmeticTarget16 {
    BC, DE, HL
}
