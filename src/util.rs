use std::time::{SystemTime, UNIX_EPOCH};

// pseudo-random number
pub fn rand_8(max: u32) -> u8{
    (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos() % max) as u8
}