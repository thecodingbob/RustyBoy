use crate::core::instructions::{ArithmeticTarget, ArithmeticTarget16, Instruction};
use crate::core::registers::Registers;

#[derive(Debug)]
struct CPU {
    registers: Registers
}
impl CPU {
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => {
                self.add(target);
            },
            Instruction::ADDHL(target) => {
                self.add_hl(target)
            },
            Instruction::ADC(target) => {
                self.adc(target)
            }
        }
    }

    fn get_target_value(&mut self, target: ArithmeticTarget) -> u8 {
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

    fn get_target_value_16(&mut self, target: ArithmeticTarget16) -> u16 {
        match target {
            ArithmeticTarget16::BC => self.registers.get_bc(),
            ArithmeticTarget16::DE => self.registers.get_de(),
            ArithmeticTarget16::HL => self.registers.get_hl()
        }
    }

    fn adc(&mut self, target: ArithmeticTarget){
        let value = self.get_target_value(target);
        self.add_constant_carry(value);
    }

    fn add(&mut self, target: ArithmeticTarget) {
        let value = self.get_target_value(target);
        self.add_constant(value);
    }

    fn add_constant(&mut self, value:u8) {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = ((self.registers.a & 0xF) + (value & 0xF)) > 0xF;
        self.registers.a = new_value;
    }

    fn add_constant_carry(&mut self, value:u8) {
        let carry_value: u8 = if self.registers.f.carry { 1 } else { 0 };

        let (mut new_value, mut did_overflow) = self.registers.a.overflowing_add(value);

        let mut carry_did_overflow;
        (new_value, carry_did_overflow) = new_value.overflowing_add(carry_value);
        did_overflow = did_overflow || carry_did_overflow;

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = ((self.registers.a & 0xF) + ((value + carry_value) & 0xF)) > 0xF;
        self.registers.a = new_value;
    }

    fn add_hl(&mut self, target: ArithmeticTarget16) {
        let target_value = self.get_target_value_16(target);
        self.add_constant_16(target_value)
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
}

#[cfg(test)]
mod test{
    use crate::core::cpu::CPU;
    use crate::core::instructions::{ArithmeticTarget, ArithmeticTarget16};
    use crate::core::registers::{FlagRegister, Registers};

    fn initialize_cpu() -> CPU {
        CPU{
            registers: Registers{
                a: 0,
                b: 0,
                c: 0,
                d: 0,
                e: 0,
                f: FlagRegister{
                    zero: false,
                    subtract: false,
                    half_carry: false,
                    carry: false,
                },
                h: 0,
                l: 0
            }
        }
    }

    #[test]
    fn test_add_constant(){
        let mut cpu = initialize_cpu();
        cpu.add_constant(1);

        assert_eq!(1, cpu.registers.a);
        assert_eq!(FlagRegister{
            zero: false,
            subtract: false,
            half_carry: false,
            carry: false
        }, cpu.registers.f);
    }

    #[test]
    fn test_add_zero_flag(){
        let mut cpu = initialize_cpu();

        cpu.add_constant(0);

        assert_eq!(0, cpu.registers.a);
        assert_eq!(FlagRegister{
            zero: true,
            subtract: false,
            half_carry: false,
            carry: false
        }, cpu.registers.f);
    }

    #[test]
    fn test_add_half_carry_flag(){
        let mut cpu = initialize_cpu();
        cpu.registers.a = 0xF;

        cpu.add_constant(0xF);

        assert_eq!(0x1E, cpu.registers.a);
        assert_eq!(FlagRegister{
            zero: false,
            subtract: false,
            half_carry: true,
            carry: false
        }, cpu.registers.f);
    }

    #[test]
    fn test_add_carry_flag(){
        let mut cpu = initialize_cpu();
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
    fn test_add(){
        let mut cpu = initialize_cpu();
        cpu.registers.c = 0x10;
        cpu.registers.h = 0x3;

        cpu.add(ArithmeticTarget::C);

        assert_eq!(0x10, cpu.registers.a);

        cpu.add(ArithmeticTarget::H);

        assert_eq!(0x13, cpu.registers.a);

        cpu.add(ArithmeticTarget::A);

        assert_eq!(0x26, cpu.registers.a);
    }

    #[test]
    fn test_add_constant_carry(){
        let mut cpu = initialize_cpu();
        cpu.registers.f.carry = true;

        cpu.add_constant_carry(0x0);

        assert_eq!(0x1, cpu.registers.a);
        assert_eq!(FlagRegister{
            zero: false,
            subtract: false,
            half_carry: false,
            carry: false
        }, cpu.registers.f);
    }

    #[test]
    fn test_add_constant_carry_zero(){
        let mut cpu = initialize_cpu();
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
    fn test_adc(){
        let mut cpu = initialize_cpu();
        cpu.registers.e = 0x13;

        cpu.adc(ArithmeticTarget::E);

        assert_eq!(0x13, cpu.registers.a);
        assert_eq!(FlagRegister{
            zero: false,
            subtract: false,
            half_carry: false,
            carry: false
        }, cpu.registers.f);
    }

    #[test]
    fn test_add_constant_16(){
        let mut cpu = initialize_cpu();

        cpu.add_constant_16(0xABCD);
        assert_eq!(0xAB, cpu.registers.h);
        assert_eq!(0xCD, cpu.registers.l);
        assert_eq!(FlagRegister{
            zero: false,
            subtract: false,
            half_carry: false,
            carry: false
        }, cpu.registers.f);
    }

    #[test]
    fn test_add_constant_16_carry(){
        let mut cpu = initialize_cpu();
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
    fn test_add_hl(){
        let mut cpu = initialize_cpu();
        cpu.registers.set_bc(0x1234);

        cpu.add_hl(ArithmeticTarget16::BC);
        assert_eq!(0x12, cpu.registers.h);
        assert_eq!(0x34, cpu.registers.l);
        assert_eq!(FlagRegister{
            zero: false,
            subtract: false,
            half_carry: false,
            carry: false
        }, cpu.registers.f);

        cpu.add_hl(ArithmeticTarget16::HL);
        assert_eq!(0x24, cpu.registers.h);
        assert_eq!(0x68, cpu.registers.l);
        assert_eq!(FlagRegister{
            zero: false,
            subtract: false,
            half_carry: false,
            carry: false
        }, cpu.registers.f);

    }
}