use crate::core::cpu::base::CPU;
use crate::core::instructions::definitions::JumpCondition;
use crate::util::join_u8;

impl CPU{
    pub (super) fn jump_conditional_to_nn(&mut self, jump_condition: JumpCondition){
        let should_jump = match jump_condition {
            JumpCondition::NotZero => !self.registers.f.zero,
            JumpCondition::Zero => self.registers.f.zero,
            JumpCondition::NotCarry => !self.registers.f.carry,
            JumpCondition::Carry => self.registers.f.carry
        };
        self.jump(should_jump)
    }

    pub (super) fn jump_to_nn(&mut self) {
        self.jump(true);
    }

    fn jump(&mut self, should_jump: bool) {
        if should_jump {
            self.jump_to_pc_pointed_address();
        } else {
            let new_pc = self.program_counter.wrapping_add(2);
            self.program_counter = new_pc;
        }
    }

    fn jump_to_pc_pointed_address(&mut self) {
        let lsb_address = self.bus.read_byte(self.program_counter);
        let msb_address = self.bus.read_byte(self.program_counter.wrapping_add(1));
        let address = join_u8(msb_address, lsb_address);
        self.program_counter = address;
    }
}


#[cfg(test)]
mod test{
    use crate::core::cpu::base::CPU;
    use crate::core::instructions::definitions::JumpCondition::{Carry, NotCarry, NotZero, Zero};

    #[test]
    fn test_jump_not_zero(){
        let mut cpu = CPU::new();
        cpu.program_counter = 0x1234;
        cpu.bus.write_byte(0x1234, 0x56);
        cpu.bus.write_byte(0x1235, 0x78);
        cpu.registers.f.zero = false;

        cpu.jump_conditional_to_nn(NotZero);

        assert_eq!(0x7856, cpu.program_counter);


        cpu.registers.f.zero = true;

        cpu.jump_conditional_to_nn(NotZero);

        assert_eq!(0x7858, cpu.program_counter);
    }

    #[test]
    fn test_jump_zero(){
        let mut cpu = CPU::new();
        cpu.program_counter = 0x1234;
        cpu.bus.write_byte(0x1234, 0x56);
        cpu.bus.write_byte(0x1235, 0x78);
        cpu.registers.f.zero = true;

        cpu.jump_conditional_to_nn(Zero);

        assert_eq!(0x7856, cpu.program_counter);


        cpu.registers.f.zero = false;

        cpu.jump_conditional_to_nn(Zero);

        assert_eq!(0x7858, cpu.program_counter);
    }

    #[test]
    fn test_jump_carry(){
        let mut cpu = CPU::new();
        cpu.program_counter = 0x1234;
        cpu.bus.write_byte(0x1234, 0x56);
        cpu.bus.write_byte(0x1235, 0x78);
        cpu.registers.f.carry = true;

        cpu.jump_conditional_to_nn(Carry);

        assert_eq!(0x7856, cpu.program_counter);


        cpu.registers.f.carry = false;

        cpu.jump_conditional_to_nn(Carry);

        assert_eq!(0x7858, cpu.program_counter);
    }

    #[test]
    fn test_jump_not_carry(){
        let mut cpu = CPU::new();
        cpu.program_counter = 0x1234;
        cpu.bus.write_byte(0x1234, 0x56);
        cpu.bus.write_byte(0x1235, 0x78);
        cpu.registers.f.carry = false;

        cpu.jump_conditional_to_nn(NotCarry);

        assert_eq!(0x7856, cpu.program_counter);


        cpu.registers.f.carry = true;

        cpu.jump_conditional_to_nn(NotCarry);

        assert_eq!(0x7858, cpu.program_counter);
    }

    #[test]
    fn test_jump(){
        let mut cpu = CPU::new();
        cpu.program_counter = 0x1234;
        cpu.bus.write_byte(0x1234, 0x56);
        cpu.bus.write_byte(0x1235, 0x78);
        cpu.registers.f.carry = true;

        cpu.jump_to_nn();

        assert_eq!(0x7856, cpu.program_counter);
    }
}