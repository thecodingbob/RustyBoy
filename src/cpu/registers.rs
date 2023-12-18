const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: FlagRegister,
    h: u8,
    l: u8,
}

struct FlagRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool
}

impl Registers {
    fn get_af(&self) -> u16 {
        join_u8(self.a, self.f)
    }

    fn set_af(&mut self, value: u16){
        (self.a, self.f) = split_u16(value);
    }

    fn get_bc(&self) -> u16 {
        join_u8(self.b, self.c)
    }

    fn set_bc(&mut self, value: u16){
        (self.b, self.c) = split_u16(value);
    }

    fn get_de(&self) -> u16 {
        join_u8(self.d, self.e)
    }

    fn set_de(&mut self, value: u16){
        (self.d, self.e) = split_u16(value);
    }

    fn get_hl(&self) -> u16 {
        join_u8(self.h, self.l)
    }

    fn set_hl(&mut self, value: u16){
        (self.h, self.l) = split_u16(value);
    }
}

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION |
        (if flag.subtract { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION |
        (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION |
        (if flag.carry { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry
        }
    }
}

fn join_u8(value1: u8, value2: u8) -> u16 {
    (value1 as u16) << 8 | value2 as u16
}

fn split_u16(value: u16) -> (u8, u8){
    (
        ((value & 0xFF00) >> 8) as u8,
        (value & 0xFF) as u8
    )
}