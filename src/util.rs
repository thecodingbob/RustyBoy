use std::time::{SystemTime, UNIX_EPOCH};

// pseudo-random number
pub fn rand_8(max: u32) -> u8{
    (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos() % max) as u8
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