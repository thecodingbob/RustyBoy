use crate::core::cpu::base::CPU;
use crate::core::instructions::definitions::RegisterTarget16;
use crate::util::join_u8;

impl CPU {
    pub(super) fn ld_rr_nn(&mut self, target: RegisterTarget16) {
        let lsb_nn = self.read_and_increment_pc();
        let msb_nn = self.read_and_increment_pc();
        self.set_register_value_16(target, join_u8(msb_nn, lsb_nn));
    }
}


#[cfg(test)]
mod test{
    use strum::IntoEnumIterator;
    use crate::core::cpu::base::CPU;
    use crate::core::instructions::definitions::RegisterTarget16;
    use crate::util::{Randomizable, split_u16};

    #[test]
    fn test_ld_rr_nn(){
        for receiver in RegisterTarget16::iter() {
            let mut cpu = CPU::new();
            let value = u16::random();
            let (msb_value, lsb_value) = split_u16(value);
            let pc = u16::random();
            cpu.program_counter = pc;
            cpu.bus.write_byte(pc, lsb_value);
            cpu.bus.write_byte(pc.wrapping_add(1), msb_value);

            cpu.ld_rr_nn(receiver);

            assert_eq!(value, cpu.get_register_value_16(receiver));
            assert_eq!(pc.wrapping_add(2), cpu.program_counter);
        }
    }
}