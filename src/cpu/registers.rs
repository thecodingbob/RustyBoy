struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
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

fn join_u8(value1: u8, value2: u8) -> u16 {
    (value1 as u16) << 8 | value2 as u16
}

fn split_u16(value: u16) -> (u8, u8){
    (
        ((value & 0xFF00) >> 8) as u8,
        (value & 0xFF) as u8
    )
}