use strum::EnumIter;
use Instruction::*;
use RegisterTarget::*;
use crate::core::instructions::Instruction::LDRN;

pub (super) enum Instruction {
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
    // Jumps if the JumpCondition is satisfied
    JP(JumpCondition)
}

impl Instruction {
    pub (super) fn from_byte(byte: u8, is_prefixed: bool) -> Option<Instruction>{
        if is_prefixed {
            Instruction::from_byte_not_prefixed(byte)
        } else {
            Instruction::from_byte_prefixed(byte)
        }
    }

    fn from_byte_prefixed(byte: u8) -> Option<Instruction>{
        match byte {
            0x02 => Some(LDBCA),
            0x06 => Some(LDRN(B)),
            0x0A => Some(LDABC),
            0x0E => Some(LDRN(C)),
            0x12 => Some(LDDEA),
            0x16 => Some(LDRN(D)),
            0x1A => Some(LDADE),
            0x1E => Some(LDRN(E)),
            0x22 => Some(LDHLINCA),
            0x26 => Some(LDRN(H)),
            0x2A => Some(LDAHLINC),
            0x2E => Some(LDRN(L)),
            0x32 => Some(LDHLDECA),
            0x36 => Some(LDHLN),
            0x3A => Some(LDAHLDEC),
            0x3E => Some(LDRN(A)),
            0x40 => Some(LDRR(B,B)),
            0x41 => Some(LDRR(B,C)),
            0x42 => Some(LDRR(B,D)),
            0x43 => Some(LDRR(B,E)),
            0x44 => Some(LDRR(B,H)),
            0x45 => Some(LDRR(B,L)),
            0x46 => Some(LDRHL(B)),
            0x47 => Some(LDRR(B,A)),
            _ => None
        }
    }

    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction>{
        match byte {
            _ => None
        }
    }
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub (super) enum RegisterTarget {
    A, B, C, D, E, H, L
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub(super) enum RegisterTarget16 {
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
