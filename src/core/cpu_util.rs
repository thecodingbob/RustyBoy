use crate::core::cpu::CPU;
use crate::core::instructions::{ArithmeticTarget, ArithmeticTarget16};
use crate::util::join_u8;

impl CPU{
    // FUNCTIONS FOR MEMORY/PC/REGISTERS R/W /////////////////////
    pub (super) fn read_and_increment_pc(&mut self) -> u8 {
        let address = self.program_counter;
        self.program_counter = address.wrapping_add(1);
        self.bus.read_byte(address)
    }

    pub (super) fn read_address_and_increment_pc(&mut self) -> u16 {
        let lsb_address = self.read_and_increment_pc();
        let msb_address = self.read_and_increment_pc();
        join_u8(msb_address, lsb_address)
    }

    pub (super) fn get_register_value(&mut self, target: ArithmeticTarget) -> u8 {
        match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l
        }
    }

    pub (super) fn get_register_pointer(&mut self, target: ArithmeticTarget) -> &mut u8 {
        match target {
            ArithmeticTarget::A => &mut self.registers.a,
            ArithmeticTarget::B => &mut self.registers.b,
            ArithmeticTarget::C => &mut self.registers.c,
            ArithmeticTarget::D => &mut self.registers.d,
            ArithmeticTarget::E => &mut self.registers.e,
            ArithmeticTarget::H => &mut self.registers.h,
            ArithmeticTarget::L => &mut self.registers.l
        }
    }

    pub (super) fn get_register_value_16(&mut self, target: ArithmeticTarget16) -> u16 {
        match target {
            ArithmeticTarget16::BC => self.registers.get_bc(),
            ArithmeticTarget16::DE => self.registers.get_de(),
            ArithmeticTarget16::HL => self.registers.get_hl()
        }
    }
    ///////////////////////////////////////////////////

    // HELPER FUNCTIONS FOR INSTRUCTIONS /////////////////////
    pub (super) fn add_constant(&mut self, value:u8) {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = ((self.registers.a & 0xF) + (value & 0xF)) > 0xF;
        self.registers.a = new_value;
    }
    pub (super) fn add_constant_carry(&mut self, value:u8) {
        let (mut new_value, mut did_overflow) = self.registers.a.overflowing_add(value);

        let carry_did_overflow;
        (new_value, carry_did_overflow) = new_value.overflowing_add(self.registers.f.carry as u8);
        did_overflow = did_overflow || carry_did_overflow;

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = ((self.registers.a & 0xF) + (value & 0xF) + ((self.registers.f.carry as u8) & 0xF)) > 0xF;
        self.registers.a = new_value;
    }
    pub (super) fn add_constant_16(&mut self, value: u16) {
        let hl_value = self.registers.get_hl();
        let (new_value, did_overflow) = hl_value.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = ((hl_value & 0xFF) + (value & 0xFF)) > 0xFF;
        self.registers.set_hl(new_value);
    }
    pub (super) fn sub_constant(&mut self, value: u8) {
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = did_overflow;
        let (half_sub, _) = (self.registers.a & 0xF).overflowing_sub(value & 0xF);
        self.registers.f.half_carry = half_sub > 0xF;
        self.registers.a = new_value;
    }
    ///////////////////////////////////////////////////
}

#[cfg(test)]
mod test{
    use strum::IntoEnumIterator;
    use crate::core::cpu::CPU;
    use crate::core::instructions::ArithmeticTarget;
    use crate::util::{join_u8, Randomizable};

    #[test]
    fn test_read_and_increment_pc(){
        let mut cpu = CPU::new();
        let address = 0xFFFF;
        let value = 0x12;
        cpu.program_counter = address;
        cpu.bus.write_byte(address, value);

        let pc_read_value = cpu.read_and_increment_pc();

        assert_eq!(value, pc_read_value);
        assert_eq!(0x0, cpu.program_counter);
    }
    #[test]
    fn test_read_address_and_increment_pc(){
        let mut cpu = CPU::new();
        let lsb_stored_address = 0xFFFF;
        let msb_stored_address = 0x0;
        let lsb_target_address = 0x12;
        let msb_target_address = 0x20;

        cpu.program_counter = lsb_stored_address;

        cpu.bus.write_byte(lsb_stored_address, lsb_target_address);
        cpu.bus.write_byte(msb_stored_address, msb_target_address);

        let result = cpu.read_address_and_increment_pc();

        println!("{:x}", result);
        println!("{:x}", join_u8(msb_target_address, lsb_target_address));
        assert_eq!(join_u8(msb_target_address, lsb_target_address), result);
        assert_eq!(msb_stored_address.wrapping_add(1), cpu.program_counter);
    }

    #[test]
    fn test_get_target_register(){
        let mut cpu = CPU::new();
        for target in ArithmeticTarget::iter(){
            assert_eq!(0x0, *cpu.get_register_pointer(target));
        }
        for target in ArithmeticTarget::iter(){
            let val: u8 = u8::next_random(0xFF);
            *cpu.get_register_pointer(target) = val;

            assert_eq!(val as u8, cpu.get_register_value(target));
        }
    }
}