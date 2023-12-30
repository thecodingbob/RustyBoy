use crate::core::cpu::base::CPU;
use crate::core::instructions::definitions::RegisterTarget16;
use crate::util::split_u16;

impl CPU {
    pub(super) fn load_register16_nn(&mut self, target: RegisterTarget16) {
        let nn = self.read_word_and_increment_pc();
        self.set_register_value_16(target, nn);
    }

    pub(super) fn load_nn_from_stack_pointer(&mut self){
        let nn_address = self.read_word_and_increment_pc();
        let (msb_stack_pointer, lsb_stack_pointer) = split_u16(self.stack_pointer);
        self.bus.write_byte(nn_address, lsb_stack_pointer);
        self.bus.write_byte(nn_address.wrapping_add(1), msb_stack_pointer);
    }

    pub(super) fn load_stack_pointer_from_hl(&mut self){
        let hl_value = self.registers.get_hl();
        self.stack_pointer = hl_value;
    }
}


#[cfg(test)]
mod test{
    use strum::IntoEnumIterator;
    use crate::core::cpu::base::CPU;
    use crate::core::instructions::definitions::RegisterTarget16;
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
        let (msb_sp, lsb_sp) = split_u16(sp_address);
        let nn = u16::random();
        let nn_address = u16::random();
        let (msb_nn, lsb_nn) = split_u16(nn);

        cpu.bus.write_byte(nn_address, lsb_nn);
        cpu.bus.write_byte(nn_address.wrapping_add(1), msb_nn);
        cpu.program_counter = nn_address;
        cpu.stack_pointer = sp_address;

        cpu.load_nn_from_stack_pointer();

        assert_eq!(nn_address.wrapping_add(2), cpu.program_counter);
        assert_eq!(lsb_sp, cpu.bus.read_byte(nn));
        assert_eq!(msb_sp, cpu.bus.read_byte(nn.wrapping_add(1)));
    }

    #[test]
    fn test_load_stack_pointer_from_hl(){
        let mut cpu = CPU::new();
        let value = u16::random();
        cpu.registers.set_hl(value);

        cpu.load_stack_pointer_from_hl();

        assert_eq!(value, cpu.stack_pointer);
    }
}