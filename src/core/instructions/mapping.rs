use crate::core::instructions::definitions::Instruction;
use crate::core::instructions::definitions::Instruction::*;
use crate::core::instructions::definitions::RegisterTarget::{A, B, C, D, E, H, L};

const INSTRUCTION_ARR:[Option<Instruction>; 256] = init_instruction_array();


impl Instruction {

    pub(crate) fn from_byte(byte: u8, is_prefixed: bool) -> Option<Instruction> {
        if is_prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            INSTRUCTION_ARR[byte as usize]
        }
    }

    fn from_byte_prefixed(byte: u8) -> Option<Instruction>{
        None
    }
}

const fn init_instruction_array() -> [Option<Instruction>; 256] {
    let mut array = [None; 256];
    
    array[0x02] = Some(LDBCA);
    
    array[0x06] = Some(LDRN(B));
    
    array[0x0A] = Some(LDABC);
    
    array[0x0E] = Some(LDRN(C));
    
    array[0x12] = Some(LDDEA);
    
    array[0x16] = Some(LDRN(D));
    
    array[0x1A] = Some(LDADE);
    
    array[0x1E] = Some(LDRN(E));
    
    array[0x22] = Some(LDHLINCA);
    
    array[0x26] = Some(LDRN(H));
    
    array[0x2A] = Some(LDAHLINC);
    
    array[0x2E] = Some(LDRN(L));
    
    array[0x32] = Some(LDHLDECA);
    
    array[0x36] = Some(LDHLN);
    
    array[0x3A] = Some(LDAHLDEC);
    
    array[0x3E] = Some(LDRN(A));
    
    array[0x40] = Some(LDRR(B,B));
    array[0x41] = Some(LDRR(B,C));
    array[0x42] = Some(LDRR(B,D));
    array[0x43] = Some(LDRR(B,E));
    array[0x44] = Some(LDRR(B,H));
    array[0x45] = Some(LDRR(B,L));
    array[0x46] = Some(LDRHL(B));
    array[0x47] = Some(LDRR(B,A));
    array[0x48] = Some(LDRR(C,B));
    array[0x49] = Some(LDRR(C,C));
    array[0x4A] = Some(LDRR(C,D));
    array[0x4B] = Some(LDRR(C,E));
    array[0x4C] = Some(LDRR(C,H));
    array[0x4D] = Some(LDRR(C,L));
    array[0x4E] = Some(LDRHL(C));
    array[0x4F] = Some(LDRR(C,A));
    array[0x50] = Some(LDRR(D,B));
    array[0x51] = Some(LDRR(D,C));
    array[0x52] = Some(LDRR(D,D));
    array[0x53] = Some(LDRR(D,E));
    array[0x54] = Some(LDRR(D,H));
    array[0x55] = Some(LDRR(D,L));
    array[0x56] = Some(LDRHL(D));
    array[0x57] = Some(LDRR(D,A));
    array[0x58] = Some(LDRR(E,B));
    array[0x59] = Some(LDRR(E,C));
    array[0x5A] = Some(LDRR(E,D));
    array[0x5B] = Some(LDRR(E,E));
    array[0x5C] = Some(LDRR(E,H));
    array[0x5D] = Some(LDRR(E,L));
    array[0x5E] = Some(LDRHL(E));
    array[0x5F] = Some(LDRR(E,A));
    array[0x60] = Some(LDRR(H,B));
    array[0x61] = Some(LDRR(H,C));
    array[0x62] = Some(LDRR(H,D));
    array[0x63] = Some(LDRR(H,E));
    array[0x64] = Some(LDRR(H,H));
    array[0x65] = Some(LDRR(H,L));
    array[0x66] = Some(LDRHL(H));
    array[0x67] = Some(LDRR(H,A));
    array[0x68] = Some(LDRR(L,B));
    array[0x69] = Some(LDRR(L,C));
    array[0x6A] = Some(LDRR(L,D));
    array[0x6B] = Some(LDRR(L,E));
    array[0x6C] = Some(LDRR(L,H));
    array[0x6D] = Some(LDRR(L,L));
    array[0x6E] = Some(LDRHL(L));
    array[0x6F] = Some(LDRR(L,A));
    array[0x70] = Some(LDHLR(B));
    array[0x71] = Some(LDHLR(C));
    array[0x72] = Some(LDHLR(D));
    array[0x73] = Some(LDHLR(E));
    array[0x74] = Some(LDHLR(H));
    array[0x75] = Some(LDHLR(L));
    
    array[0x77] = Some(LDHLR(A));
    array[0x78] = Some(LDRR(A,B));
    array[0x79] = Some(LDRR(A,C));
    array[0x7A] = Some(LDRR(A,D));
    array[0x7B] = Some(LDRR(A,E));
    array[0x7C] = Some(LDRR(A,H));
    array[0x7D] = Some(LDRR(A,L));
    array[0x7E] = Some(LDRHL(A));
    array[0x7F] = Some(LDRR(A,A));
    array[0x80] = Some(ADDR(B));
    array[0x81] = Some(ADDR(C));
    array[0x82] = Some(ADDR(D));
    array[0x83] = Some(ADDR(E));
    array[0x84] = Some(ADDR(H));
    array[0x85] = Some(ADDR(L));
    array[0x86] = Some(ADDHL);
    array[0x87] = Some(ADDR(A));
    array[0x88] = Some(ADCR(B));
    array[0x89] = Some(ADCR(C));
    array[0x8A] = Some(ADCR(D));
    array[0x8B] = Some(ADCR(E));
    array[0x8C] = Some(ADCR(H));
    array[0x8D] = Some(ADCR(L));
    array[0x8E] = Some(ADCHL);
    array[0x8F] = Some(ADCR(A));

    array[0xC6] = Some(ADDN);

    array[0xCE] = Some(ADCN);

    array
}

#[cfg(test)]
mod test{
    use crate::core::instructions::mapping::init_instruction_array;

    #[test]
    fn test_init_instruction_array(){
        let mapping = init_instruction_array();

        for (i, el1) in mapping.iter().enumerate() {
            if *el1 == None {
                continue;
            }
            for el2 in &mapping[(i + 1)..] {
                if *el2 == None {
                    continue;
                }
                assert_ne!((*el1).unwrap(), (*el2).unwrap());
            }
        }
    }
}