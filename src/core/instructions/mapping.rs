use crate::core::instructions::definitions::Instruction;
use crate::core::instructions::definitions::Instruction::*;
use crate::core::instructions::definitions::JumpCondition::*;
use crate::core::instructions::definitions::RegisterTarget::{A, B, C, D, E, H, L};

const INSTRUCTION_ARR:[Option<Instruction>; 256] = init_instruction_array();
const PREFIX_INSTRUCTION_ARR:[Option<Instruction>; 256] = init_prefix_instruction_array();


impl Instruction {

    pub(crate) fn from_byte(byte: u8, is_prefixed: bool) -> Option<Instruction> {
        if is_prefixed {
            PREFIX_INSTRUCTION_ARR[byte as usize]
        } else {
            INSTRUCTION_ARR[byte as usize]
        }
    }

    fn from_byte_prefixed(byte: u8) -> Option<Instruction>{
        None
    }
}

const fn init_instruction_array() -> [Option<Instruction>; 256] {
    let mut a = [None; 256];
    
    a[0x02] = Some(LDBCA);
    
    a[0x06] = Some(LDRN(B));
    
    a[0x0A] = Some(LDABC);
    
    a[0x0E] = Some(LDRN(C));
    
    a[0x12] = Some(LDDEA);
    
    a[0x16] = Some(LDRN(D));
    
    a[0x1A] = Some(LDADE);
    
    a[0x1E] = Some(LDRN(E));
    
    a[0x22] = Some(LDHLINCA);
    
    a[0x26] = Some(LDRN(H));
    
    a[0x2A] = Some(LDAHLINC);
    
    a[0x2E] = Some(LDRN(L));
    
    a[0x32] = Some(LDHLDECA);
    
    a[0x36] = Some(LDHLN);
    
    a[0x3A] = Some(LDAHLDEC);
    
    a[0x3E] = Some(LDRN(A));
    
    a[0x40] = Some(LDRR(B, B));
    a[0x41] = Some(LDRR(B, C));
    a[0x42] = Some(LDRR(B, D));
    a[0x43] = Some(LDRR(B, E));
    a[0x44] = Some(LDRR(B, H));
    a[0x45] = Some(LDRR(B, L));
    a[0x46] = Some(LDRHL(B));
    a[0x47] = Some(LDRR(B, A));
    a[0x48] = Some(LDRR(C, B));
    a[0x49] = Some(LDRR(C, C));
    a[0x4A] = Some(LDRR(C, D));
    a[0x4B] = Some(LDRR(C, E));
    a[0x4C] = Some(LDRR(C, H));
    a[0x4D] = Some(LDRR(C, L));
    a[0x4E] = Some(LDRHL(C));
    a[0x4F] = Some(LDRR(C, A));
    a[0x50] = Some(LDRR(D, B));
    a[0x51] = Some(LDRR(D, C));
    a[0x52] = Some(LDRR(D, D));
    a[0x53] = Some(LDRR(D, E));
    a[0x54] = Some(LDRR(D, H));
    a[0x55] = Some(LDRR(D, L));
    a[0x56] = Some(LDRHL(D));
    a[0x57] = Some(LDRR(D, A));
    a[0x58] = Some(LDRR(E, B));
    a[0x59] = Some(LDRR(E, C));
    a[0x5A] = Some(LDRR(E, D));
    a[0x5B] = Some(LDRR(E, E));
    a[0x5C] = Some(LDRR(E, H));
    a[0x5D] = Some(LDRR(E, L));
    a[0x5E] = Some(LDRHL(E));
    a[0x5F] = Some(LDRR(E, A));
    a[0x60] = Some(LDRR(H, B));
    a[0x61] = Some(LDRR(H, C));
    a[0x62] = Some(LDRR(H, D));
    a[0x63] = Some(LDRR(H, E));
    a[0x64] = Some(LDRR(H, H));
    a[0x65] = Some(LDRR(H, L));
    a[0x66] = Some(LDRHL(H));
    a[0x67] = Some(LDRR(H, A));
    a[0x68] = Some(LDRR(L, B));
    a[0x69] = Some(LDRR(L, C));
    a[0x6A] = Some(LDRR(L, D));
    a[0x6B] = Some(LDRR(L, E));
    a[0x6C] = Some(LDRR(L, H));
    a[0x6D] = Some(LDRR(L, L));
    a[0x6E] = Some(LDRHL(L));
    a[0x6F] = Some(LDRR(L, A));
    a[0x70] = Some(LDHLR(B));
    a[0x71] = Some(LDHLR(C));
    a[0x72] = Some(LDHLR(D));
    a[0x73] = Some(LDHLR(E));
    a[0x74] = Some(LDHLR(H));
    a[0x75] = Some(LDHLR(L));
    
    a[0x77] = Some(LDHLR(A));
    a[0x78] = Some(LDRR(A, B));
    a[0x79] = Some(LDRR(A, C));
    a[0x7A] = Some(LDRR(A, D));
    a[0x7B] = Some(LDRR(A, E));
    a[0x7C] = Some(LDRR(A, H));
    a[0x7D] = Some(LDRR(A, L));
    a[0x7E] = Some(LDRHL(A));
    a[0x7F] = Some(LDRR(A, A));
    a[0x80] = Some(ADDR(B));
    a[0x81] = Some(ADDR(C));
    a[0x82] = Some(ADDR(D));
    a[0x83] = Some(ADDR(E));
    a[0x84] = Some(ADDR(H));
    a[0x85] = Some(ADDR(L));
    a[0x86] = Some(ADDHL);
    a[0x87] = Some(ADDR(A));
    a[0x88] = Some(ADCR(B));
    a[0x89] = Some(ADCR(C));
    a[0x8A] = Some(ADCR(D));
    a[0x8B] = Some(ADCR(E));
    a[0x8C] = Some(ADCR(H));
    a[0x8D] = Some(ADCR(L));
    a[0x8E] = Some(ADCHL);
    a[0x8F] = Some(ADCR(A));

    a[0xC2] = Some(JPCCNN(NotZero));
    a[0xC3] = Some(JPNN);

    a[0xC6] = Some(ADDN);

    a[0xCA] = Some(JPCCNN(Zero));

    a[0xCE] = Some(ADCN);

    a[0xD2] = Some(JPCCNN(NotCarry));

    a[0xDA] = Some(JPCCNN(Carry));

    a[0xE0] = Some(LDHNA);

    a[0xE2] = Some(LDHCA);
    // None
    // None

    a[0xEA] = Some(LDNNA);

    a[0xF0] = Some(LDHAN);

    a[0xF2] = Some(LDHAC);

    a[0xFA] = Some(LDANN);

    a
}

const fn init_prefix_instruction_array() -> [Option<Instruction>; 256] {
    let mut array = [None; 256];
    array
}

#[cfg(test)]
mod test{
    use crate::core::instructions::definitions::Instruction;
    use crate::core::instructions::mapping::{init_instruction_array, init_prefix_instruction_array};

    #[test]
    fn test_init_instruction_array(){
        test_no_duplicates(init_instruction_array());
    }

    #[test]
    fn test_init_prefix_instruction_array(){
        test_no_duplicates(init_prefix_instruction_array());
    }

    fn test_no_duplicates(array: [Option<Instruction>; 256]){
        for (i, el1) in array.iter().enumerate() {
            if *el1 == None {
                continue;
            }
            for el2 in &array[(i + 1)..] {
                if *el2 == None {
                    continue;
                }
                assert_ne!((*el1).unwrap(), (*el2).unwrap());
            }
        }
    }
}