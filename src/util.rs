use std::time::{Duration, SystemTime, UNIX_EPOCH};

// pseudo-random number
pub trait Randomizable {
    fn random() -> Self;
}

impl Randomizable for u8 {
    fn random() -> u8 {
        (rand_from_system_time() % (u8::MAX as u32) + 1) as u8
    }
}

impl Randomizable for u16 {
    fn random() -> u16 {
        (rand_from_system_time() % (u16::MAX as u32) + 1) as u16
    }
}

fn rand_from_system_time() -> u32 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos()
}

pub fn join_u8(most_significant: u8, least_significant: u8) -> u16 {
    (most_significant as u16) << 8 | least_significant as u16
}

pub fn split_u16(value: u16) -> (u8, u8){
    (
        ((value & 0xFF00) >> 8) as u8,
        (value & 0xFF) as u8
    )
}

#[cfg(test)]
mod test{
    use crate::util::{join_u8, split_u16};

    #[test]
    fn test_split_u16(){
        let (a, b) = split_u16(0xABCD);

        assert_eq!(0xAB, a);
        assert_eq!(0xCD, b);
    }

    #[test]
    fn test_join_u16(){
        assert_eq!(0xABCD, join_u8(0xAB, 0xCD));
        assert_eq!(0xCD, join_u8(0x0, 0xCD));
    }
}