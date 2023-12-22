use crate::core::instructions::{ArithmeticTarget, ArithmeticTarget16, Instruction};
use crate::core::registers::Registers;

#[derive(Debug)]
struct CPU {
    registers: Registers,
    program_counter: u16
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
            },
            Instruction::LDR(source, receiver) => {
                *self.get_register_pointer(receiver) = self.get_register_value(source)
            },
            Instruction::LDRN(receiver, value) => {
                *self.get_register_pointer(receiver) = value;
            },
            Instruction::LDRHL(receiver) => {
                let address = self.registers.get_hl();
                let value = self.read_address(address);
                *self.get_register_pointer(receiver) = value;
            },
            Instruction::LDHLR(source) => {
                let value = self.get_register_value(source);
                let address = self.registers.get_hl();
                self.write_address(address, value);
            },
            Instruction::LDHLN(value) => {
                let address = self.registers.get_hl();
                self.write_address(address, value);
            },
            Instruction::LDABC => {
                let address = self.registers.get_bc();
                let value = self.read_address(address);
                self.registers.a = value;
            },
            Instruction::LDADE => {
                let address = self.registers.get_de();
                let value = self.read_address(address);
                self.registers.a = value;
            },
            Instruction::LDBCA => {
                let address = self.registers.get_bc();
                let value = self.registers.a;
                self.write_address(address, value);
            },
            Instruction::LDDEA => {
                let address = self.registers.get_de();
                let value = self.registers.a;
                self.write_address(address, value);
            },
            Instruction::LDANN(address) => {
                let value = self.read_address(address);
                self.registers.a = value;
            },
            Instruction::LDNNA(address) => {
                let value = self.registers.a;
                self.write_address(address, value);
            }
        }
    }

    fn write_address(&mut self, address: u16, value: u8){
        //TODO
    }

    fn read_address(&mut self, address: u16) -> u8 {
        return address as u8; //TODO
    }

    fn get_register_value(&mut self, target: ArithmeticTarget) -> u8 {
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

    fn get_register_pointer(&mut self, target: ArithmeticTarget) -> &mut u8 {
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

    fn get_register_value_16(&mut self, target: ArithmeticTarget16) -> u16 {
        match target {
            ArithmeticTarget16::BC => self.registers.get_bc(),
            ArithmeticTarget16::DE => self.registers.get_de(),
            ArithmeticTarget16::HL => self.registers.get_hl()
        }
    }

    fn adc(&mut self, target: ArithmeticTarget){
        let value = self.get_register_value(target);
        self.add_constant_carry(value);
    }

    fn add(&mut self, target: ArithmeticTarget) {
        let value = self.get_register_value(target);
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

    fn add_hl(&mut self, target: ArithmeticTarget16) {
        let target_value = self.get_register_value_16(target);
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

    fn sub_constant(&mut self, value: u8) {
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = did_overflow;
        let (half_sub, _) = (self.registers.a & 0xF).overflowing_sub(value & 0xF);
        self.registers.f.half_carry = half_sub > 0xF;
        self.registers.a = new_value;
    }
}

#[cfg(test)]
mod test{
    use strum::IntoEnumIterator;
    use crate::core::cpu::CPU;
    use crate::core::instructions::{ArithmeticTarget, ArithmeticTarget16, Instruction};
    use crate::core::registers::{FlagRegister, Registers};
    use crate::util::rand_8;

    fn initialize_cpu() -> CPU {
        CPU{
            registers: Registers{
                a: 0,
                b: 0,
                c: 0,
                d: 0,
                e: 0,
                f: FlagRegister::from(0b0),
                h: 0,
                l: 0
            },
            program_counter: 0
        }
    }

    #[test]
    fn test_ld_a_nn(){
        let mut cpu = initialize_cpu();
        let address = 0x12;

        cpu.execute(Instruction::LDANN(address));

        let expected_value = cpu.read_address(address);

        assert_eq!(expected_value, cpu.registers.a);
    }

    #[test]
    fn test_ld_nn_a(){
        // TODO after implementing memory operations
    }

    #[test]
    fn test_ld_bc_a(){
        // TODO after implementing memory operations
    }

    #[test]
    fn test_ld_de_a(){
        // TODO after implementing memory operations
    }

    #[test]
    fn test_ld_a_de(){
        let mut cpu = initialize_cpu();

        cpu.execute(Instruction::LDADE);

        let address = cpu.registers.get_de();
        let expected_value = cpu.read_address(address);

        assert_eq!(expected_value, cpu.registers.a);
    }

    #[test]
    fn test_ld_a_bc(){
        let mut cpu = initialize_cpu();

        cpu.execute(Instruction::LDABC);

        let address = cpu.registers.get_bc();
        let expected_value = cpu.read_address(address);

        assert_eq!(expected_value, cpu.registers.a);
    }

    #[test]
    fn test_ld_hl_n(){
        // TODO after implementing memory operations
    }

    #[test]
    fn test_ld_r_hl(){
        for receiver in ArithmeticTarget::iter() {
            let mut cpu = initialize_cpu();

            cpu.execute(Instruction::LDRHL(receiver));

            //TODO: add some better logic after stack pop has been implemented
            assert_eq!(0x1, cpu.get_register_value(receiver));
        }
    }

    #[test]
    fn test_ld_n(){
        for receiver in ArithmeticTarget::iter() {
            let mut cpu = initialize_cpu();
            let value = rand_8(0xFF);

            cpu.execute(Instruction::LDRN(receiver, value));

            assert_eq!(value, cpu.get_register_value(receiver));
        }
    }

    #[test]
    fn test_ld_r(){
        for source in ArithmeticTarget::iter(){
            for receiver in ArithmeticTarget::iter() {
                let mut cpu = initialize_cpu();
                *cpu.get_register_pointer(source) = 0x1;

                cpu.execute(Instruction::LDR(source, receiver));

                let source_value = cpu.get_register_value(source);
                let receiver_value = cpu.get_register_value(receiver);

                assert_eq!(source_value, receiver_value);
            }
        }
    }

    #[test]
    fn test_get_target_register(){
        let mut cpu = initialize_cpu();
        for target in ArithmeticTarget::iter(){
            assert_eq!(0x0, *cpu.get_register_pointer(target));
        }
        for target in ArithmeticTarget::iter(){
            let val: u8 = rand_8(0xFF);
            *cpu.get_register_pointer(target) = val;

            assert_eq!(val as u8, cpu.get_register_value(target));
        }
    }

    #[test]
    fn test_add_constant(){
        let mut cpu = initialize_cpu();
        cpu.add_constant(1);

        assert_eq!(1, cpu.registers.a);
        assert_eq!(FlagRegister::from(0b0), cpu.registers.f);
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
        assert_eq!(FlagRegister::from(0b0), cpu.registers.f);
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
        assert_eq!(FlagRegister::from(0b0), cpu.registers.f);
    }

    #[test]
    fn test_add_constant_16(){
        let mut cpu = initialize_cpu();

        cpu.add_constant_16(0xABCD);
        assert_eq!(0xAB, cpu.registers.h);
        assert_eq!(0xCD, cpu.registers.l);
        assert_eq!(FlagRegister::from(0b0), cpu.registers.f);
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
        assert_eq!(FlagRegister::from(0b0), cpu.registers.f);

        cpu.add_hl(ArithmeticTarget16::HL);

        assert_eq!(0x24, cpu.registers.h);
        assert_eq!(0x68, cpu.registers.l);
        assert_eq!(FlagRegister::from(0b0), cpu.registers.f);
    }

    #[test]
    fn test_sub_constant(){
        let mut cpu = initialize_cpu();

        cpu.sub_constant(0x1);

        assert_eq!(0xFF, cpu.registers.a);
        assert_eq!(FlagRegister{
            zero: false,
            subtract: true,
            half_carry: true,
            carry: true
        }, cpu.registers.f);
    }

    #[test]
    fn test_execute(){
        let mut cpu = initialize_cpu();
        cpu.registers.a = 0x1;

        cpu.execute(Instruction::ADD(ArithmeticTarget::A));

        assert_eq!(0x2, cpu.registers.a);
    }

}