use crate::core::cpu::base::CPU;
use crate::core::instructions::definitions::RegisterTarget;

impl CPU{
    // Matching actual instructions /////////////////////
    pub (super) fn add_r(&mut self, target: RegisterTarget) {
        let value = self.get_register_value(target);
        self.add_constant(value);
    }
    pub (super) fn add_hl(&mut self) {
        let address = self.registers.get_hl();
        let value = self.bus.read_byte(address);
        self.add_constant(value);
    }
    pub (super) fn add_n(&mut self) {
        let value = self.read_and_increment_pc();
        self.add_constant(value);
    }
    pub (super) fn adc_r(&mut self, target: RegisterTarget){
        let value = self.get_register_value(target);
        self.add_constant_carry(value);
    }
    pub (super) fn adc_hl(&mut self) {
        let address = self.registers.get_hl();
        let value = self.bus.read_byte(address);
        self.add_constant_carry(value);
    }
    pub (super) fn adc_n(&mut self) {
        let value = self.read_and_increment_pc();
        self.add_constant_carry(value);
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
        self.registers.f.half_carry = ((self.registers.a & 0xF) + (value & 0xF) + (self.registers.f.carry as u8)) > 0xF;
        self.registers.a = new_value;
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
    use crate::core::instructions::definitions::RegisterTarget;
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

    // matching instructions
    #[test]
    fn test_add(){
        let mut cpu = CPU::new();
        cpu.registers.c = 0x10;
        cpu.registers.h = 0x3;

        cpu.add_r(RegisterTarget::C);

        assert_eq!(0x10, cpu.registers.a);

        cpu.add_r(RegisterTarget::H);

        assert_eq!(0x13, cpu.registers.a);

        cpu.add_r(RegisterTarget::A);

        assert_eq!(0x26, cpu.registers.a);
    }

    #[test]
    fn test_adc(){
        let mut cpu = CPU::new();
        cpu.registers.f.carry = true;
        cpu.registers.e = 0x13;

        cpu.adc_r(RegisterTarget::E);

        assert_eq!(0x14, cpu.registers.a);
        assert_eq!(FlagRegister::from(0b0), cpu.registers.f);
    }


    #[test]
    fn test_add_hl(){
        let mut cpu = CPU::new();
        let hl_address = 0x1234;
        let value = 0x11;
        let a_value = 0xF0;
        cpu.registers.set_hl(hl_address);
        cpu.bus.write_byte(hl_address, value);
        cpu.registers.a = a_value;

        cpu.add_hl();

        assert_eq!(0x1, cpu.registers.a);
        assert_eq!(FlagRegister{
            zero: false,
            subtract: false,
            half_carry: false,
            carry: true,
        }, cpu.registers.f);
    }


    #[test]
    fn test_adc_r(){
        let mut cpu = CPU::new();
        cpu.registers.b = 0x2;
        cpu.registers.f.carry = true;

        cpu.adc_r(RegisterTarget::B);

        assert_eq!(0x3, cpu.registers.a);
        assert_eq!(FlagRegister::from(0b0), cpu.registers.f);
    }
    #[test]
    fn test_adc_hl() {
        let mut cpu = CPU::new();
        let hl_address = 0x1234;
        let value = 0x11;
        let a_value = 0xF0;
        cpu.registers.set_hl(hl_address);
        cpu.bus.write_byte(hl_address, value);
        cpu.registers.a = a_value;

        cpu.adc_hl();

        assert_eq!(0x1, cpu.registers.a);
        assert_eq!(FlagRegister{
            zero: false,
            subtract: false,
            half_carry: false,
            carry: true,
        }, cpu.registers.f);
    }

    #[test]
    fn test_adc_n() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x0;
        cpu.bus.write_byte(0x0, 0x10);
        cpu.registers.f.carry = true;

        cpu.adc_n();

        assert_eq!(0x11, cpu.registers.a);
        assert_eq!(FlagRegister::from(0b0), cpu.registers.f);
        assert_eq!(0x1, cpu.program_counter);
    }

    #[test]
    fn test_add_n(){
        let mut cpu = CPU::new();
        cpu.program_counter = 0x0;
        cpu.bus.write_byte(0x0, 0x10);

        cpu.add_n();

        assert_eq!(0x10, cpu.registers.a);
        assert_eq!(FlagRegister::from(0b0), cpu.registers.f);
        assert_eq!(0x1, cpu.program_counter);
    }

}