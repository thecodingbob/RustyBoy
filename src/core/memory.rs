use crate::util::{join_u8, split_u16};

#[derive(Debug)]
pub (super) struct MemoryBus {
    //TODO: check if this is correct, as the guide stated 0xFFFF had to be used, but that caused oob
    memory: [u8; 0x10000]
}

impl MemoryBus {

    pub (super) fn new() -> Self {
        MemoryBus {
            memory: [0; 0x10000]
        }
    }
    pub (super) fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub (super) fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    pub (super) fn read_word(&mut self, lsb_address: u16) -> u16 {
        let lsb_value = self.memory[lsb_address as usize];
        let msb_value = self.memory[lsb_address.wrapping_add(1) as usize];
        join_u8(msb_value, lsb_value)
    }

    pub (super) fn write_word(&mut self, lsb_address: u16, word: u16){
        let (msb_word, lsb_word) = split_u16(word);
        self.memory[lsb_address as usize] = lsb_word;
        self.memory[lsb_address.wrapping_add(1) as usize] = msb_word;
    }
}

#[cfg(test)]

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

    #[test]
    fn test_read_word(){
        let mut bus = MemoryBus::new();
        let value = 0x1234;
        bus.memory[0xFFFF] = 0x34;
        bus.memory[0x0] = 0x12;

        assert_eq!(value, bus.read_word(0xFFFF));
    }

    #[test]
    fn test_write_word(){
        let mut bus = MemoryBus::new();
        let word = 0x1234;

        bus.write_word(0xFFFF, word);

        assert_eq!(0x34, bus.memory[0xFFFF]);
        assert_eq!(0x12, bus.memory[0x0]);
    }
}