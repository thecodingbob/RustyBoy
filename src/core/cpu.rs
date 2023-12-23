use crate::core::instructions::{ArithmeticTarget, ArithmeticTarget16, Instruction, JumpCondition};
use crate::core::memory::MemoryBus;
use crate::core::registers::Registers;
use crate::util::join_u8;

#[derive(Debug)]
pub(super) struct CPU {
    pub (super) registers: Registers,
    pub (super) program_counter: u16,
    pub (super) bus: MemoryBus
}
impl CPU {
    pub (super) fn new() -> Self {
        CPU{
            registers: Registers::new(),
            program_counter: 0,
            bus: MemoryBus::new()
        }
    }

    fn step(&mut self){
        let mut instruction_byte = self.bus.read_byte(self.program_counter);
        let is_prefixed = instruction_byte == 0xCB;
        if is_prefixed {
            instruction_byte = self.bus.read_byte(self.program_counter.wrapping_add(1));
        }

        if let Some(instruction) = Instruction::from_byte(instruction_byte, is_prefixed){
            self.execute(instruction);
        } else {
            let description = format!("0x{}{:x}", if is_prefixed { "cb" } else { "" }, instruction_byte);
            panic!("Unknown instruction for: 0x{}", description)
        };
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => {
                self.add(target);
            },
            Instruction::ADDHL(target) => {
                self.add_hl(target);
            },
            Instruction::ADC(target) => {
                self.adc(target);
            },
            Instruction::LDR(source, receiver) => {
                *self.get_register_pointer(receiver) = self.get_register_value(source);
            },
            Instruction::LDRN(receiver) => {
                let n = self.read_and_increment_pc();
                *self.get_register_pointer(receiver) = n;
            },
            Instruction::LDRHL(receiver) => {
                let address = self.registers.get_hl();
                let value = self.bus.read_byte(address);
                *self.get_register_pointer(receiver) = value;
            },
            Instruction::LDHLR(source) => {
                let value = self.get_register_value(source);
                let address = self.registers.get_hl();
                self.bus.write_byte(address, value);
            },
            Instruction::LDHLN => {
                let address = self.registers.get_hl();
                let n = self.read_and_increment_pc();
                self.bus.write_byte(address, n);
            },
            Instruction::LDABC => {
                let address = self.registers.get_bc();
                let value = self.bus.read_byte(address);
                self.registers.a = value;
            },
            Instruction::LDADE => {
                let address = self.registers.get_de();
                let value = self.bus.read_byte(address);
                self.registers.a = value;
            },
            Instruction::LDBCA => {
                let address = self.registers.get_bc();
                let value = self.registers.a;
                self.bus.write_byte(address, value);
            },
            Instruction::LDDEA => {
                let address = self.registers.get_de();
                let value = self.registers.a;
                self.bus.write_byte(address, value);
            },
            Instruction::LDANN => {
                let lsb_address = self.read_and_increment_pc();
                let msb_address = self.read_and_increment_pc();
                let value = self.bus.read_byte(join_u8(msb_address, lsb_address));
                self.registers.a = value;
            },
            Instruction::LDNNA => {
                let value = self.registers.a;
                let address = self.read_address_and_increment_pc();
                self.bus.write_byte(address, value);
            },
            Instruction::LDHAC => {
                let address = join_u8(0xFF, self.registers.c);
                let value = self.bus.read_byte(address);
                self.registers.a = value;
            },
            Instruction::LDHCA => {
                let address = join_u8(0xFF, self.registers.c);
                let value = self.registers.a;
                self.bus.write_byte(address, value);
            },
            Instruction::LDHAN => {
                let lsb_address = self.read_and_increment_pc();
                let address = join_u8(0xFF, lsb_address);
                let value = self.bus.read_byte(address);
                self.registers.a = value;
            }
            Instruction::LDHNA => {
                let lsb_address = self.read_and_increment_pc();
                let address = join_u8(0xFF, lsb_address);
                let value = self.registers.a;
                self.bus.write_byte(address, value);
            },
            Instruction::LDAHLDEC => {
                let address = self.registers.get_hl();
                let value = self.bus.read_byte(address);

                self.registers.a = value;
                self.registers.set_hl(address.wrapping_sub(1));
            },
            Instruction::LDHLDECA => {
                let address = self.registers.get_hl();
                let value = self.registers.a;

                self.bus.write_byte(address, value);
                self.registers.set_hl(address.wrapping_sub(1));
            },
            Instruction::LDAHLINC => {
                let address = self.registers.get_hl();
                let value = self.bus.read_byte(address);

                self.registers.a = value;
                self.registers.set_hl(address.wrapping_add(1));
            },
            Instruction::LDHLINCA => {
                let address = self.registers.get_hl();
                let value = self.registers.a;

                self.bus.write_byte(address, value);
                self.registers.set_hl(address.wrapping_add(1));
            },
            Instruction::JP(jump_condition) => {
                let should_jump = match jump_condition {
                    JumpCondition::NotZero => !self.registers.f.zero,
                    JumpCondition::Zero => self.registers.f.zero,
                    JumpCondition::NotCarry => !self.registers.f.carry,
                    JumpCondition::Carry => self.registers.f.carry,
                    JumpCondition::Always => true
                };
                // TODO self.jump(jump_condition)
            }
        }
    }

    // FUNCTIONS DIRECTLY MATCHING INSTRUCTIONS /////////////////////
    fn adc(&mut self, target: ArithmeticTarget){
        let value = self.get_register_value(target);
        self.add_constant_carry(value);
    }
    fn add(&mut self, target: ArithmeticTarget) {
        let value = self.get_register_value(target);
        self.add_constant(value);
    }
    fn add_hl(&mut self, target: ArithmeticTarget16) {
        let target_value = self.get_register_value_16(target);
        self.add_constant_16(target_value)
    }
    ///////////////////////////////////////////////////

}

#[cfg(test)]
mod test{
    use strum::IntoEnumIterator;
    use crate::core::cpu::CPU;
    use crate::core::instructions::{ArithmeticTarget, ArithmeticTarget16, Instruction};
    use crate::core::registers::FlagRegister;
    use crate::util::{join_u8, Randomizable};

    #[test]
    fn test_step(){
        let mut cpu = CPU::new();

        cpu.step();
    }

    #[test]
    fn test_ld_hl_inc_a(){
        let mut cpu = CPU::new();
        let hl_address = u16::MAX;
        cpu.registers.set_hl(hl_address);

        cpu.execute(Instruction::LDAHLINC);

        let expected_value = cpu.bus.read_byte(hl_address);

        // TODO after implementing memory operations
        assert_eq!(hl_address.wrapping_add(1), cpu.registers.get_hl())
    }

    #[test]
    fn test_ld_a_hl_inc(){
        //TODO: fix address increment/decrement logic
        let mut cpu = CPU::new();
        let hl_address = u16::MAX;
        cpu.registers.set_hl(hl_address);

        cpu.execute(Instruction::LDAHLINC);

        let expected_value = cpu.bus.read_byte(hl_address);

        assert_eq!(expected_value, cpu.registers.a);
        assert_eq!(hl_address.wrapping_add(1), cpu.registers.get_hl())
    }

    #[test]
    fn test_ld_hl_dec_a(){
        let mut cpu = CPU::new();
        let hl_address = 0;
        cpu.registers.set_hl(hl_address);

        cpu.execute(Instruction::LDAHLDEC);

        let expected_value = cpu.bus.read_byte(hl_address);

        // TODO after implementing memory operations
        assert_eq!(hl_address.wrapping_sub(1), cpu.registers.get_hl())
    }

    #[test]
    fn test_ld_a_hl_dec(){
        let mut cpu = CPU::new();
        let hl_address = 0;
        cpu.registers.set_hl(hl_address);

        cpu.execute(Instruction::LDAHLDEC);

        let expected_value = cpu.bus.read_byte(hl_address);

        assert_eq!(expected_value, cpu.registers.a);
        assert_eq!(hl_address.wrapping_sub(1), cpu.registers.get_hl())
    }

    #[test]
    fn test_ld_h_n_a(){
        // TODO after implementing memory operations
    }

    #[test]
    fn test_ld_h_a_n(){
        let mut cpu = CPU::new();
        let n_address = 0x12;
        let full_address = join_u8(0xFF, n_address);

        cpu.execute(Instruction::LDHAN);

        let expected_value = cpu.bus.read_byte(full_address);

        assert_eq!(expected_value, cpu.registers.a);
    }

    #[test]
    fn test_ld_h_c_a(){
        // TODO after implementing memory operations
    }

    #[test]
    fn test_ld_h_a_c(){
        let mut cpu = CPU::new();
        let c_address = 0x12;
        cpu.registers.c = c_address;
        let full_address = join_u8(0xFF, c_address);

        cpu.execute(Instruction::LDHAC);

        let expected_value = cpu.bus.read_byte(full_address);

        assert_eq!(expected_value, cpu.registers.a);
    }

    #[test]
    fn test_ld_a_nn(){
        let mut cpu = CPU::new();
        let address = 0x12;

        cpu.execute(Instruction::LDANN);

        let expected_value = cpu.bus.read_byte(address);

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
        let mut cpu = CPU::new();

        cpu.execute(Instruction::LDADE);

        let address = cpu.registers.get_de();
        let expected_value = cpu.bus.read_byte(address);

        assert_eq!(expected_value, cpu.registers.a);
    }

    #[test]
    fn test_ld_a_bc(){
        let mut cpu = CPU::new();

        cpu.execute(Instruction::LDABC);

        let address = cpu.registers.get_bc();
        let expected_value = cpu.bus.read_byte(address);

        assert_eq!(expected_value, cpu.registers.a);
    }

    #[test]
    fn test_ld_hl_n(){
        // TODO after implementing memory operations
    }

    #[test]
    fn test_ld_r_hl(){
        for receiver in ArithmeticTarget::iter() {
            let mut cpu = CPU::new();

            cpu.execute(Instruction::LDRHL(receiver));

            //TODO: add some better logic after stack pop has been implemented
            assert_eq!(0x1, cpu.get_register_value(receiver));
        }
    }

    #[test]
    fn test_ld_n(){
        for receiver in ArithmeticTarget::iter() {
            let mut cpu = CPU::new();
            let value = u8::next_random(0xFF);

            cpu.execute(Instruction::LDRN(receiver));

            assert_eq!(value, cpu.get_register_value(receiver));
        }
    }

    #[test]
    fn test_ld_r(){
        for source in ArithmeticTarget::iter(){
            for receiver in ArithmeticTarget::iter() {
                let mut cpu = CPU::new();
                *cpu.get_register_pointer(source) = 0x1;

                cpu.execute(Instruction::LDR(source, receiver));

                let source_value = cpu.get_register_value(source);
                let receiver_value = cpu.get_register_value(receiver);

                assert_eq!(source_value, receiver_value);
            }
        }
    }

    #[test]
    fn test_add_constant(){
        let mut cpu = CPU::new();
        cpu.add_constant(1);

        assert_eq!(1, cpu.registers.a);
        assert_eq!(FlagRegister::from(0b0), cpu.registers.f);
    }

    #[test]
    fn test_add_zero_flag(){
        let mut cpu = CPU::new();

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
        let mut cpu = CPU::new();
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
    fn test_add(){
        let mut cpu = CPU::new();
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
        let mut cpu = CPU::new();
        cpu.registers.f.carry = true;

        cpu.add_constant_carry(0x0);

        assert_eq!(0x1, cpu.registers.a);
        assert_eq!(FlagRegister::from(0b0), cpu.registers.f);
    }

    #[test]
    fn test_add_constant_carry_zero(){
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
    fn test_adc(){
        let mut cpu = CPU::new();
        cpu.registers.e = 0x13;

        cpu.adc(ArithmeticTarget::E);

        assert_eq!(0x13, cpu.registers.a);
        assert_eq!(FlagRegister::from(0b0), cpu.registers.f);
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
    fn test_add_constant_16_carry(){
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
    fn test_add_hl(){
        let mut cpu = CPU::new();
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

    #[test]
    fn test_execute(){
        let mut cpu = CPU::new();
        cpu.registers.a = 0x1;

        cpu.execute(Instruction::ADD(ArithmeticTarget::A));

        assert_eq!(0x2, cpu.registers.a);
    }

}