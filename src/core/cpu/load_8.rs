use crate::core::cpu::base::CPU;
use crate::core::instructions::RegisterTarget;
use crate::util::join_u8;

impl CPU{
    // 8 BIT LOAD /////////////////////
    pub (super) fn ld_r_r(&mut self, source: RegisterTarget, receiver: RegisterTarget) {
        let value = self.get_register_value(source);
        self.set_register_value(receiver, value);
    }
    pub (super) fn ld_r_n(&mut self, target: RegisterTarget) {
        let n = self.read_and_increment_pc();
        self.set_register_value(target, n);
    }
    pub (super) fn ld_r_hl(&mut self, target: RegisterTarget) {
        let address = self.registers.get_hl();
        let value = self.bus.read_byte(address);
        self.set_register_value(target, value);
    }
    pub (super) fn ld_hl_r(&mut self, source: RegisterTarget) {
        let value = self.get_register_value(source);
        let address = self.registers.get_hl();
        self.bus.write_byte(address, value);
    }

    pub (super) fn ld_hl_n(&mut self) {
        let address = self.registers.get_hl();
        let n = self.read_and_increment_pc();
        self.bus.write_byte(address, n);
    }

    pub (super) fn ld_a_bc(&mut self) {
        let address = self.registers.get_bc();
        let value = self.bus.read_byte(address);
        self.registers.a = value;
    }

    pub (super) fn ld_a_de(&mut self) {
        let address = self.registers.get_de();
        let value = self.bus.read_byte(address);
        self.registers.a = value;
    }

    pub (super) fn ld_bc_a(&mut self) {
        let address = self.registers.get_bc();
        let value = self.registers.a;
        self.bus.write_byte(address, value);
    }

    pub (super) fn ld_de_a(&mut self){
        let address = self.registers.get_de();
        let value = self.registers.a;
        self.bus.write_byte(address, value);
    }

    pub (super) fn ld_a_nn(&mut self){
        let lsb_address = self.read_and_increment_pc();
        let msb_address = self.read_and_increment_pc();
        let value = self.bus.read_byte(join_u8(msb_address, lsb_address));
        self.registers.a = value;
    }

    pub (super) fn ld_nn_a(&mut self){
        let value = self.registers.a;
        let address = self.read_address_and_increment_pc();
        self.bus.write_byte(address, value);
    }

    pub (super) fn ld_h_a_c(&mut self){
        let address = join_u8(0xFF, self.registers.c);
        let value = self.bus.read_byte(address);
        self.registers.a = value;
    }

    pub (super) fn ld_h_c_a(&mut self){
        let address = join_u8(0xFF, self.registers.c);
        let value = self.registers.a;
        self.bus.write_byte(address, value);
    }

    pub (super) fn ld_h_a_n(&mut self){
        let lsb_address = self.read_and_increment_pc();
        let address = join_u8(0xFF, lsb_address);
        let value = self.bus.read_byte(address);
        self.registers.a = value;
    }

    pub (super) fn ld_h_n_a(&mut self){
        let lsb_address = self.read_and_increment_pc();
        let address = join_u8(0xFF, lsb_address);
        let value = self.registers.a;
        self.bus.write_byte(address, value);
    }

    pub (super) fn ld_a_hl_dec(&mut self){
        let address = self.registers.get_hl();
        let value = self.bus.read_byte(address);

        self.registers.a = value;
        self.registers.set_hl(address.wrapping_sub(1));
    }

    pub (super) fn ld_hl_dec_a(&mut self){
        let address = self.registers.get_hl();
        let value = self.registers.a;

        self.bus.write_byte(address, value);
        self.registers.set_hl(address.wrapping_sub(1));
    }

    pub (super) fn ld_a_hl_inc(&mut self){
        let address = self.registers.get_hl();
        let value = self.bus.read_byte(address);

        self.registers.a = value;
        self.registers.set_hl(address.wrapping_add(1));
    }

    pub (super) fn ld_hl_inc_a(&mut self){
        let address = self.registers.get_hl();
        let value = self.registers.a;

        self.bus.write_byte(address, value);
        self.registers.set_hl(address.wrapping_add(1));
    }
    ///////////////////////////////////////////////////
}

#[cfg(test)]
mod test{
    use strum::IntoEnumIterator;
    use crate::core::cpu::base::CPU;
    use crate::core::instructions::{RegisterTarget};
    use crate::util::{join_u8, Randomizable, split_u16};

    #[test]
    fn test_ld_hl_inc_a(){
        let mut cpu = CPU::new();
        let hl_address = u16::MAX;
        cpu.registers.set_hl(hl_address);

        cpu.ld_hl_inc_a();

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

        cpu.ld_a_hl_inc();

        let expected_value = cpu.bus.read_byte(hl_address);

        assert_eq!(expected_value, cpu.registers.a);
        assert_eq!(hl_address.wrapping_add(1), cpu.registers.get_hl())
    }

    #[test]
    fn test_ld_hl_dec_a(){
        let mut cpu = CPU::new();
        let hl_address = 0;
        cpu.registers.set_hl(hl_address);

        cpu.ld_a_hl_dec();

        let expected_value = cpu.bus.read_byte(hl_address);

        // TODO after implementing memory operations
        assert_eq!(hl_address.wrapping_sub(1), cpu.registers.get_hl())
    }

    #[test]
    fn test_ld_a_hl_dec(){
        let mut cpu = CPU::new();
        let hl_address = 0;
        cpu.registers.set_hl(hl_address);

        cpu.ld_a_hl_dec();

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

        cpu.ld_h_a_n();

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

        cpu.ld_h_a_c();

        let expected_value = cpu.bus.read_byte(full_address);

        assert_eq!(expected_value, cpu.registers.a);
    }

    #[test]
    fn test_ld_a_nn(){
        let mut cpu = CPU::new();
        let address = 0x12;

        cpu.ld_a_nn();

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

        cpu.ld_a_de();

        let address = cpu.registers.get_de();
        let expected_value = cpu.bus.read_byte(address);

        assert_eq!(expected_value, cpu.registers.a);
    }

    #[test]
    fn test_ld_a_bc(){
        let mut cpu = CPU::new();

        cpu.ld_a_bc();

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
}