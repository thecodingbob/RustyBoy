pub (super) enum Instruction {
    ADD(ArithmeticTarget),
}

pub (super) enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}
