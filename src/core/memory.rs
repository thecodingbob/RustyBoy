#[derive(Debug)]
pub (super) struct MemoryBus {
    memory: [u8; 0xFFFF]
}

impl MemoryBus {

    pub (super) fn new() -> Self {
        MemoryBus {
            memory: [0; 0xFFFF]
        }
    }
    pub (super) fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub (super) fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}

mod test{
    use crate::core::memory::MemoryBus;

    #[test]
    fn test_write_byte(){
        let mut bus = MemoryBus::new();
        let value = 0x1A;
        let address = 0xFF;

        bus.write_byte(address, value);

        assert_eq!(value, bus.memory[address as usize])
    }

    #[test]
    fn test_read_byte(){
        let mut bus = MemoryBus::new();
        let value = 0x1A;
        let address = 0xFF;
        bus.memory[address as usize] = value;

        assert_eq!(value, bus.read_byte(address))
    }
}