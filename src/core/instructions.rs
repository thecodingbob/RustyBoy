pub (super) enum Instruction {
    ADD(ArithmeticTarget),
    ADDHL(ArithmeticTarget16),
    ADC(ArithmeticTarget)
}

pub (super) enum ArithmeticTarget {
    A, B, C, D, E, H, L
}

pub(super) enum ArithmeticTarget16 {
    BC, DE, HL
}
