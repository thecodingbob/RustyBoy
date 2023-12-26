use crate::core::cpu::base::CPU;
use crate::core::instructions::{RegisterTarget, RegisterTarget16};
use crate::util::join_u8;

impl CPU{
    pub (super) fn read_and_increment_pc(&mut self) -> u8 {
        let address = self.program_counter;
        self.program_counter = address.wrapping_add(1);
        self.bus.read_byte(address)
    }

    pub (super) fn read_address_and_increment_pc(&mut self) -> u16 {
        let lsb_address = self.read_and_increment_pc();
        let msb_address = self.read_and_increment_pc();
        join_u8(msb_address, lsb_address)
    }

    pub (super) fn get_register_value(&mut self, target: RegisterTarget) -> u8 {
        match target {
            RegisterTarget::A => self.registers.a,
            RegisterTarget::B => self.registers.b,
            RegisterTarget::C => self.registers.c,
            RegisterTarget::D => self.registers.d,
            RegisterTarget::E => self.registers.e,
            RegisterTarget::H => self.registers.h,
            RegisterTarget::L => self.registers.l
        }
    }

    pub (super) fn get_register_pointer(&mut self, target: RegisterTarget) -> &mut u8 {
        match target {
            RegisterTarget::A => &mut self.registers.a,
            RegisterTarget::B => &mut self.registers.b,
            RegisterTarget::C => &mut self.registers.c,
            RegisterTarget::D => &mut self.registers.d,
            RegisterTarget::E => &mut self.registers.e,
            RegisterTarget::H => &mut self.registers.h,
            RegisterTarget::L => &mut self.registers.l
        }
    }

    pub (super) fn set_register_value(&mut self, target: RegisterTarget, value: u8) {
        *self.get_register_pointer(target) = value;
    }

    pub (super) fn get_register_value_16(&mut self, target: RegisterTarget16) -> u16 {
        match target {
            RegisterTarget16::BC => self.registers.get_bc(),
            RegisterTarget16::DE => self.registers.get_de(),
            RegisterTarget16::HL => self.registers.get_hl()
        }
    }

    pub (super) fn set_register_value_16(&mut self, target: RegisterTarget16, value: u16) {
        match target {
            RegisterTarget16::BC => self.registers.set_bc(value),
            RegisterTarget16::DE => self.registers.set_de(value),
            RegisterTarget16::HL => self.registers.set_hl(value)
        }
    }

    fn get_8_bit_targets_from_16_bit_target(&mut self, target: RegisterTarget16) -> (RegisterTarget, RegisterTarget) {
        match target {
            RegisterTarget16::BC => (RegisterTarget::B, RegisterTarget::C),
            RegisterTarget16::DE => (RegisterTarget::D, RegisterTarget::E),
            RegisterTarget16::HL => (RegisterTarget::H, RegisterTarget::L)
        }
    }
}

#[cfg(test)]
mod test{
    use strum::IntoEnumIterator;
    use crate::core::cpu::base::CPU;
    use crate::core::instructions::{RegisterTarget, RegisterTarget16};
    use crate::util::{join_u8, Randomizable, split_u16};

    #[test]
    fn test_read_and_increment_pc(){
        let mut cpu = CPU::new();
        let address = 0xFFFF;
        let value = 0x12;
        cpu.program_counter = address;
        cpu.bus.write_byte(address, value);

        let pc_read_value = cpu.read_and_increment_pc();

        assert_eq!(value, pc_read_value);
        assert_eq!(0x0, cpu.program_counter);
    }
    #[test]
    fn test_read_address_and_increment_pc(){
        let mut cpu = CPU::new();
        let lsb_stored_address = 0xFFFF;
        let msb_stored_address = 0x0;
        let lsb_target_address = 0x12;
        let msb_target_address = 0x20;

        cpu.program_counter = lsb_stored_address;

        cpu.bus.write_byte(lsb_stored_address, lsb_target_address);
        cpu.bus.write_byte(msb_stored_address, msb_target_address);

        let result = cpu.read_address_and_increment_pc();

        println!("{:x}", result);
        println!("{:x}", join_u8(msb_target_address, lsb_target_address));
        assert_eq!(join_u8(msb_target_address, lsb_target_address), result);
        assert_eq!(msb_stored_address.wrapping_add(1), cpu.program_counter);
    }

    #[test]
    fn test_set_register_value(){
        let mut cpu = CPU::new();
        for target in RegisterTarget::iter(){
            let val = u8::random();
            cpu.set_register_value(target, val);

            assert_eq!(val, *cpu.get_register_pointer(target));
        }

        // Not relying on any helper functions
        let val = 0x12;
        cpu.set_register_value(RegisterTarget::E, val);

        assert_eq!(val, cpu.registers.e);
    }

    #[test]
    fn test_get_register_value_and_pointer(){
        let mut cpu = CPU::new();
        for target in RegisterTarget::iter(){
            assert_eq!(0x0, *cpu.get_register_pointer(target));
        }
        for target in RegisterTarget::iter(){
            let val = u8::random();
            *cpu.get_register_pointer(target) = val;

            assert_eq!(val, cpu.get_register_value(target));
        }
    }

    #[test]
    fn test_get_register_value_16(){
        let mut cpu = CPU::new();
        for target in RegisterTarget16::iter(){
            assert_eq!(0x0, cpu.get_register_value_16(target));
        }
        for target in RegisterTarget16::iter(){
            let val = u16::random();
            let (msb, lsb) = split_u16(val);
            let (msb_target, lsb_target) = cpu.get_8_bit_targets_from_16_bit_target(target);
            cpu.set_register_value(msb_target, msb);
            cpu.set_register_value(lsb_target, lsb);

            assert_eq!(val, cpu.get_register_value_16(target));
        }
    }

    #[test]
    fn test_set_register_value_16(){
        let mut cpu = CPU::new();
        for target in RegisterTarget16::iter(){
            assert_eq!(0x0, cpu.get_register_value_16(target));
        }
        for target in RegisterTarget16::iter(){
            let val = u16::random();

            cpu.set_register_value_16(target, val);

            assert_eq!(val, cpu.get_register_value_16(target));
        }
    }
}