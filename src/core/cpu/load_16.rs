use crate::core::cpu::base::CPU;
use crate::core::instructions::definitions::{PushPopTarget, RegisterTarget16};

impl CPU {
    pub(super) fn load_register16_nn(&mut self, target: RegisterTarget16) {
        let nn = self.read_word_and_increment_pc();
        self.set_register_value_16(target, nn);
    }

    pub(super) fn load_nn_from_stack_pointer(&mut self){
        let nn_address = self.read_word_and_increment_pc();
        let stack_pointer = self.stack_pointer;
        self.bus.write_word(nn_address, stack_pointer);
    }

    pub(super) fn load_stack_pointer_from_hl(&mut self){
        let hl_value = self.registers.get_hl();
        self.stack_pointer = hl_value;
    }

    pub(super) fn push_from_register(&mut self, source: PushPopTarget) {
        let new_stack_pointer = self.stack_pointer.wrapping_sub(2);
        let value = self.get_push_pop_target_value(source);
        self.bus.write_word(new_stack_pointer, value);
        self.stack_pointer = new_stack_pointer;
    }

    pub(super) fn pop_into_register(&mut self, target: PushPopTarget) {
        let stack_pointer = self.stack_pointer;
        let value = self.bus.read_word(stack_pointer);
        self.set_push_pop_target_value(target, value);
        self.stack_pointer = stack_pointer.wrapping_add(2);
    }

}


#[cfg(test)]
mod test{
    use strum::IntoEnumIterator;
    use crate::core::cpu::base::CPU;
    use crate::core::instructions::definitions::{PushPopTarget, RegisterTarget16};
    use crate::core::registers::AF_BIT_MASK;
    use crate::util::{Randomizable, split_u16};

    #[test]
    fn test_load_register16_nn(){
        for receiver in RegisterTarget16::iter() {
            let mut cpu = CPU::new();
            let value = u16::random();
            let (msb_value, lsb_value) = split_u16(value);
            let pc = u16::random();
            cpu.program_counter = pc;
            cpu.bus.write_byte(pc, lsb_value);
            cpu.bus.write_byte(pc.wrapping_add(1), msb_value);

            cpu.load_register16_nn(receiver);

            assert_eq!(value, cpu.get_register_value_16(receiver));
            assert_eq!(pc.wrapping_add(2), cpu.program_counter);
        }
    }

    #[test]
    fn test_load_nn_from_stack_pointer(){
        let mut cpu = CPU::new();
        let sp_address = u16::random();
        let nn = u16::random();
        let nn_address = u16::random();

        cpu.bus.write_word(nn_address, nn);
        cpu.program_counter = nn_address;
        cpu.stack_pointer = sp_address;

        cpu.load_nn_from_stack_pointer();

        assert_eq!(nn_address.wrapping_add(2), cpu.program_counter);
        assert_eq!(sp_address, cpu.bus.read_word(nn));
    }

    #[test]
    fn test_load_stack_pointer_from_hl(){
        let mut cpu = CPU::new();
        let value = u16::random();
        cpu.registers.set_hl(value);

        cpu.load_stack_pointer_from_hl();

        assert_eq!(value, cpu.stack_pointer);
    }

    #[test]
    fn test_push_from_register(){
        for source in PushPopTarget::iter() {
            let mut cpu = CPU::new();
            let mut value = u16::random();
            if source == PushPopTarget::AF {
                value = value & AF_BIT_MASK;
            }
            let old_stack_pointer = cpu.stack_pointer;
            cpu.set_push_pop_target_value(source, value);

            cpu.push_from_register(source);

            assert_eq!(value, cpu.bus.read_word(cpu.stack_pointer));
            assert_eq!(old_stack_pointer.wrapping_sub(2), cpu.stack_pointer);
        }
    }

    #[test]
    fn test_pop_into_register(){
        for target in PushPopTarget::iter() {
            let mut cpu = CPU::new();
            let mut value = u16::random();
            let sp = u16::random();
            cpu.stack_pointer = sp;
            if target == PushPopTarget::AF {
                value = value & AF_BIT_MASK;
            }
            cpu.bus.write_word(sp, value);

            cpu.pop_into_register(target);

            assert_eq!(value, cpu.get_push_pop_target_value(target));
            assert_eq!(sp.wrapping_add(2), cpu.stack_pointer);
        }
    }
}