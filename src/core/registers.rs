use crate::util::{join_u8, split_u16};

pub(super) const AF_BIT_MASK: u16 = 0xFF0;

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

#[derive(Debug)]
pub (super) struct Registers {
    pub (super) a: u8,
    pub (super) b: u8,
    pub (super) c: u8,
    pub (super) d: u8,
    pub (super) e: u8,
    pub (super) f: FlagRegister,
    pub (super) h: u8,
    pub (super) l: u8,
}

#[derive(PartialEq, Debug)]
pub (super) struct FlagRegister {
    pub (super) zero: bool,
    pub (super) subtract: bool,
    pub (super) half_carry: bool,
    pub (super) carry: bool
}



impl Registers {

    pub (super) fn new() -> Registers {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FlagRegister::from(0b0),
            h: 0,
            l: 0
        }
    }

    pub (super) fn get_af(&self) -> u16 {
        join_u8(self.a, u8::from(&self.f))
    }

    pub (super) fn set_af(&mut self, value: u16){
        let f;
        (self.a, f) = split_u16(value);
        self.f = FlagRegister::from(f);
    }

    pub (super) fn get_bc(&self) -> u16 {
        join_u8(self.b, self.c)
    }

    pub (super) fn set_bc(&mut self, value: u16){
        (self.b, self.c) = split_u16(value);
    }

    pub (super) fn get_de(&self) -> u16 {
        join_u8(self.d, self.e)
    }

    pub (super) fn set_de(&mut self, value: u16){
        (self.d, self.e) = split_u16(value);
    }

    pub (super) fn get_hl(&self) -> u16 {
        join_u8(self.h, self.l)
    }

    pub (super) fn set_hl(&mut self, value: u16){
        (self.h, self.l) = split_u16(value);
    }
}

impl From<&FlagRegister> for u8 {
    fn from(flag: &FlagRegister) -> u8 {
        (if flag.zero { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION |
        (if flag.subtract { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION |
        (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION |
        (if flag.carry { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl From<u8> for FlagRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagRegister {
            zero,
            subtract,
            half_carry,
            carry
        }
    }
}
