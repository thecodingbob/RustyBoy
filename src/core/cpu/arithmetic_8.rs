use crate::core::cpu::base::CPU;
use crate::core::instructions::{RegisterTarget, RegisterTarget16};

impl CPU{
    // Matching actual instructions /////////////////////
    pub (super) fn adc(&mut self, target: RegisterTarget){
        let value = self.get_register_value(target);
        self.add_constant_carry(value);
    }
    pub (super) fn add(&mut self, target: RegisterTarget) {
        let value = self.get_register_value(target);
        self.add_constant(value);
    }
    pub (super) fn add_hl(&mut self, target: RegisterTarget16) {
        let target_value = self.get_register_value_16(target);
        self.add_constant_16(target_value)
    }

    //////////////////////////////////////////////////////

    // HELPER FUNCTIONS FOR INSTRUCTIONS /////////////////////
    fn add_constant(&mut self, value:u8) {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = ((self.registers.a & 0xF) + (value & 0xF)) > 0xF;
        self.registers.a = new_value;
    }
    fn add_constant_carry(&mut self, value:u8) {
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
    fn add_constant_16(&mut self, value: u16) {
        let hl_value = self.registers.get_hl();
        let (new_value, did_overflow) = hl_value.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = ((hl_value & 0xFF) + (value & 0xFF)) > 0xFF;
        self.registers.set_hl(new_value);
    }
    fn sub_constant(&mut self, value: u8) {
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
    use crate::core::cpu::base::CPU;
    use crate::core::instructions::{RegisterTarget, RegisterTarget16};
    use crate::core::registers::FlagRegister;

    #[test]
    fn test_add_constant(){
        let mut cpu = CPU::new();
        cpu.add_constant(1);

        assert_eq!(1, cpu.registers.a);
        assert_eq!(FlagRegister::from(0b0), cpu.registers.f);
    }

    #[test]
    fn test_add_flags(){
        let mut cpu = CPU::new();
        cpu.registers.a = 0xFF;

        cpu.add_constant(1);

        assert_eq!(0, cpu.registers.a);
        assert_eq!(FlagRegister{
            zero: true,
            subtract: false,
            half_carry: true,
            carry: true
        }, cpu.registers.f);
    }

    #[test]
    fn test_add_constant_carry_no_carry(){
        let mut cpu = CPU::new();
        cpu.registers.a = 1;
        cpu.registers.f.carry = false;

        cpu.add_constant_carry(0x0);

        assert_eq!(0x1, cpu.registers.a);
        assert_eq!(FlagRegister::from(0b0), cpu.registers.f);
    }

    #[test]
    fn test_add_constant_carry(){
        let mut cpu = CPU::new();
        cpu.registers.f.carry = true;
        cpu.registers.a = 0xFF;

        cpu.add_constant_carry(0x0);

        assert_eq!(0x0, cpu.registers.a);
        assert_eq!(FlagRegister{
            zero: true,
            subtract: false,
            half_carry: true,
            carry: true
        }, cpu.registers.f);
    }

    #[test]
    fn test_add_constant_16(){
        let mut cpu = CPU::new();

        cpu.add_constant_16(0xABCD);
        assert_eq!(0xAB, cpu.registers.h);
        assert_eq!(0xCD, cpu.registers.l);
        assert_eq!(FlagRegister::from(0b0), cpu.registers.f);
    }

    #[test]
    fn test_add_constant_16_flags(){
        let mut cpu = CPU::new();
        cpu.registers.set_hl(0xFFFF);

        cpu.add_constant_16(1);
        assert_eq!(0, cpu.registers.h);
        assert_eq!(0, cpu.registers.l);
        assert_eq!(FlagRegister{
            zero: true,
            subtract: false,
            half_carry: true,
            carry: true
        }, cpu.registers.f);
    }

    #[test]
    fn test_sub_constant(){
        let mut cpu = CPU::new();

        cpu.sub_constant(0x1);

        assert_eq!(0xFF, cpu.registers.a);
        assert_eq!(FlagRegister{
            zero: false,
            subtract: true,
            half_carry: true,
            carry: true
        }, cpu.registers.f);
    }

    // 8 Bit arithmetic
    #[test]
    fn test_add(){
        let mut cpu = CPU::new();
        cpu.registers.c = 0x10;
        cpu.registers.h = 0x3;

        cpu.add(RegisterTarget::C);

        assert_eq!(0x10, cpu.registers.a);

        cpu.add(RegisterTarget::H);

        assert_eq!(0x13, cpu.registers.a);

        cpu.add(RegisterTarget::A);

        assert_eq!(0x26, cpu.registers.a);
    }

    #[test]
    fn test_adc(){
        let mut cpu = CPU::new();
        cpu.registers.f.carry = true;
        cpu.registers.e = 0x13;

        cpu.adc(RegisterTarget::E);

        assert_eq!(0x14, cpu.registers.a);
        assert_eq!(FlagRegister::from(0b0), cpu.registers.f);
    }


    #[test]
    fn test_add_hl(){
        let mut cpu = CPU::new();
        cpu.registers.set_bc(0x1234);

        cpu.add_hl(RegisterTarget16::BC);

        assert_eq!(0x12, cpu.registers.h);
        assert_eq!(0x34, cpu.registers.l);
        assert_eq!(FlagRegister::from(0b0), cpu.registers.f);

        cpu.add_hl(RegisterTarget16::HL);

        assert_eq!(0x24, cpu.registers.h);
        assert_eq!(0x68, cpu.registers.l);
        assert_eq!(FlagRegister::from(0b0), cpu.registers.f);
    }

}