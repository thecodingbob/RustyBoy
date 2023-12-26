use crate::core::instructions::{RegisterTarget, RegisterTarget16, Instruction, JumpCondition};
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
            // 8 bit arithmetic
            Instruction::ADD(target) => {
                self.add(target);
            },
            Instruction::ADDHL(target) => {
                self.add_hl(target);
            },
            Instruction::ADC(target) => {
                self.adc(target);
            },
            // 8 bit load
            Instruction::LDRR(source, receiver) => {
                self.ld_r_r(source, receiver);
            },
            Instruction::LDRN(receiver) => {
                self.ld_r_n(receiver);
            },
            Instruction::LDRHL(receiver) => {
                self.ld_r_hl(receiver);
            },
            Instruction::LDHLR(source) => {
                self.ld_hl_r(source);
            },
            Instruction::LDHLN => {
                self.ld_hl_n();
            },
            Instruction::LDABC => {
                self.ld_a_bc();
            },
            Instruction::LDADE => {
                self.ld_a_de();
            },
            Instruction::LDBCA => {
                self.ld_bc_a();
            },
            Instruction::LDDEA => {
                self.ld_de_a();
            },
            Instruction::LDANN => {
                self.ld_a_nn();
            },
            Instruction::LDNNA => {
                self.ld_nn_a();
            },
            Instruction::LDHAC => {
                self.ld_h_ac();
            },
            Instruction::LDHCA => {
                self.ld_h_c_a();
            },
            Instruction::LDHAN => {
                self.ld_h_a_n();
            }
            Instruction::LDHNA => {
                self.ld_h_n_a();
            },
            Instruction::LDAHLDEC => {
                self.ld_a_hl_dec();
            },
            Instruction::LDHLDECA => {
                self.ld_hl_dec_a();
            },
            Instruction::LDAHLINC => {
                self.ld_a_hl_inc();
            },
            Instruction::LDHLINCA => {
                self.ld_hl_inc_a();
            },
            // jump instructions
            Instruction::JP(jump_condition) => {
                self.jp(jump_condition);
            }
        }
    }

    // 8 BIT ARITHMETIC /////////////////////
    fn adc(&mut self, target: RegisterTarget){
        let value = self.get_register_value(target);
        self.add_constant_carry(value);
    }
    fn add(&mut self, target: RegisterTarget) {
        let value = self.get_register_value(target);
        self.add_constant(value);
    }
    fn add_hl(&mut self, target: RegisterTarget16) {
        let target_value = self.get_register_value_16(target);
        self.add_constant_16(target_value)
    }
    ///////////////////////////////////////////////////

    // 8 BIT LOAD /////////////////////
    fn ld_r_r(&mut self, source: RegisterTarget, receiver: RegisterTarget) {
        let value = self.get_register_value(source);
        self.set_register_value(receiver, value);
    }
    fn ld_r_n(&mut self, target: RegisterTarget) {
        let n = self.read_and_increment_pc();
        self.set_register_value(target, n);
    }
    fn ld_r_hl(&mut self, target: RegisterTarget) {
        let address = self.registers.get_hl();
        let value = self.bus.read_byte(address);
        self.set_register_value(target, value);
    }
    fn ld_hl_r(&mut self, source: RegisterTarget) {
        let value = self.get_register_value(source);
        let address = self.registers.get_hl();
        self.bus.write_byte(address, value);
    }

    fn ld_hl_n(&mut self) {
        let address = self.registers.get_hl();
        let n = self.read_and_increment_pc();
        self.bus.write_byte(address, n);
    }

    fn ld_a_bc(&mut self) {
        let address = self.registers.get_bc();
        let value = self.bus.read_byte(address);
        self.registers.a = value;
    }

    fn ld_a_de(&mut self) {
        let address = self.registers.get_de();
        let value = self.bus.read_byte(address);
        self.registers.a = value;
    }

    fn ld_bc_a(&mut self) {
        let address = self.registers.get_bc();
        let value = self.registers.a;
        self.bus.write_byte(address, value);
    }

    fn ld_de_a(&mut self){
        let address = self.registers.get_de();
        let value = self.registers.a;
        self.bus.write_byte(address, value);
    }

    fn ld_a_nn(&mut self){
        let lsb_address = self.read_and_increment_pc();
        let msb_address = self.read_and_increment_pc();
        let value = self.bus.read_byte(join_u8(msb_address, lsb_address));
        self.registers.a = value;
    }

    fn ld_nn_a(&mut self){
        let value = self.registers.a;
        let address = self.read_address_and_increment_pc();
        self.bus.write_byte(address, value);
    }

    fn ld_h_a_c(&mut self){
        let address = join_u8(0xFF, self.registers.c);
        let value = self.bus.read_byte(address);
        self.registers.a = value;
    }

    fn ld_h_c_a(&mut self){
        let address = join_u8(0xFF, self.registers.c);
        let value = self.registers.a;
        self.bus.write_byte(address, value);
    }

    fn ld_h_a_n(&mut self){
        let lsb_address = self.read_and_increment_pc();
        let address = join_u8(0xFF, lsb_address);
        let value = self.bus.read_byte(address);
        self.registers.a = value;
    }

    fn ld_h_n_a(&mut self){
        let lsb_address = self.read_and_increment_pc();
        let address = join_u8(0xFF, lsb_address);
        let value = self.registers.a;
        self.bus.write_byte(address, value);
    }

    fn ld_a_hl_dec(&mut self){
        let address = self.registers.get_hl();
        let value = self.bus.read_byte(address);

        self.registers.a = value;
        self.registers.set_hl(address.wrapping_sub(1));
    }

    fn ld_hl_dec_a(&mut self){
        let address = self.registers.get_hl();
        let value = self.registers.a;

        self.bus.write_byte(address, value);
        self.registers.set_hl(address.wrapping_sub(1));
    }

    fn ld_a_hl_inc(&mut self){
        let address = self.registers.get_hl();
        let value = self.bus.read_byte(address);

        self.registers.a = value;
        self.registers.set_hl(address.wrapping_add(1));
    }

    fn ld_hl_in_a(&mut self){
        let address = self.registers.get_hl();
        let value = self.registers.a;

        self.bus.write_byte(address, value);
        self.registers.set_hl(address.wrapping_add(1));
    }
    ///////////////////////////////////////////////////

    // JUMP /////////////////////
    fn jump(&mut self, jump_condition: JumpCondition){
        let should_jump = match jump_condition {
            JumpCondition::NotZero => !self.registers.f.zero,
            JumpCondition::Zero => self.registers.f.zero,
            JumpCondition::NotCarry => !self.registers.f.carry,
            JumpCondition::Carry => self.registers.f.carry,
            JumpCondition::Always => true
        };
        // TODO self.jump(jump_condition)
    }
    ///////////////////////////////////////////////////
}

#[cfg(test)]
mod test{
    use strum::IntoEnumIterator;
    use crate::core::cpu::CPU;
    use crate::core::instructions::{RegisterTarget, RegisterTarget16, Instruction};
    use crate::core::registers::FlagRegister;
    use crate::util::{join_u8, Randomizable, split_u16};

    // 8 Bit arithmetic
    #[test]
    fn test_add(){
        let mut cpu = CPU::new();
        cpu.registers.c = 0x10;
        cpu.registers.h = 0x3;

        cpu.execute(Instruction::ADD(RegisterTarget::C));

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

        cpu.execute(Instruction::ADC(RegisterTarget::E));

        assert_eq!(0x14, cpu.registers.a);
        assert_eq!(FlagRegister::from(0b0), cpu.registers.f);
    }


    #[test]
    fn test_add_hl(){
        let mut cpu = CPU::new();
        cpu.registers.set_bc(0x1234);

        cpu.execute(Instruction::ADDHL(RegisterTarget16::BC));

        assert_eq!(0x12, cpu.registers.h);
        assert_eq!(0x34, cpu.registers.l);
        assert_eq!(FlagRegister::from(0b0), cpu.registers.f);

        cpu.add_hl(RegisterTarget16::HL);

        assert_eq!(0x24, cpu.registers.h);
        assert_eq!(0x68, cpu.registers.l);
        assert_eq!(FlagRegister::from(0b0), cpu.registers.f);
    }

    // 8 bit load
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
        let mut cpu = CPU::new();
        let n = u8::random();
        let pc_address = u16::random();
        let hl_address = pc_address.wrapping_add(0x5);
        cpu.bus.write_byte(pc_address, n);
        cpu.program_counter = pc_address;
        cpu.registers.set_hl(hl_address);

        cpu.ld_hl_n();

        assert_eq!(pc_address.wrapping_add(1), cpu.program_counter);
        assert_eq!(n, cpu.bus.read_byte(hl_address));
    }

    #[test]
    fn test_ld_r_hl(){
        for receiver in RegisterTarget::iter() {
            let mut cpu = CPU::new();
            let value = u8::random();
            let address = u16::random();
            cpu.bus.write_byte(address, value);
            cpu.registers.set_hl(address);

            cpu.ld_r_hl(receiver);

            assert_eq!(value, cpu.get_register_value(receiver));
        }
    }

    #[test]
    fn test_ld_hl_r(){
        for source in RegisterTarget::iter() {
            let mut cpu = CPU::new();
            let address = u16::random();
            let value = match source {
                RegisterTarget::H => split_u16(address).0 ,
                RegisterTarget::L => split_u16(address).1 ,
                _ => u8::random()
            };
            cpu.set_register_value(source, value);
            cpu.registers.set_hl(address);

            cpu.ld_hl_r(source);

            assert_eq!(value, cpu.bus.read_byte(address));
        }
    }

    #[test]
    fn test_ld_r_n(){
        for receiver in RegisterTarget::iter() {
            let mut cpu = CPU::new();
            let value = u8::random();
            let pc = u16::random();
            cpu.program_counter = pc;
            cpu.bus.write_byte(pc, value);

            cpu.ld_r_n(receiver);

            assert_eq!(value, cpu.get_register_value(receiver));
            assert_eq!(pc.wrapping_add(1), cpu.program_counter);
        }
    }

    #[test]
    fn test_ld_r_r(){
        for source in RegisterTarget::iter(){
            for receiver in RegisterTarget::iter() {
                let mut cpu = CPU::new();
                *cpu.get_register_pointer(source) = 0x1;

                cpu.ld_r_r(source, receiver);

                let source_value = cpu.get_register_value(source);
                let receiver_value = cpu.get_register_value(receiver);

                assert_eq!(source_value, receiver_value);
            }
        }
    }

    // other
    #[test]
    fn test_step(){
        let mut cpu = CPU::new();

        cpu.step();
    }

    #[test]
    fn test_execute(){
        let mut cpu = CPU::new();
        cpu.registers.a = 0x1;

        cpu.execute(Instruction::ADD(RegisterTarget::A));

        assert_eq!(0x2, cpu.registers.a);
    }

}