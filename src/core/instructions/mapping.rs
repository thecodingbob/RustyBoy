use crate::core::instructions::definitions::{Instruction, PushPopTarget};
use crate::core::instructions::definitions::Instruction::*;
use crate::core::instructions::definitions::JumpCondition::*;
use crate::core::instructions::definitions::RegisterTarget::*;
use crate::core::instructions::definitions::RegisterTarget16::*;

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

    a[0x01] = Some(LoadRegister16Nn(BC));
    a[0x02] = Some(LoadIndirectBcA);
    
    a[0x06] = Some(LoadRegisterN(B));

    a[0x08] = Some(LoadNnFromStackPointer);

    a[0x0A] = Some(LoadAIndirectBc);
    
    a[0x0E] = Some(LoadRegisterN(C));

    a[0x11] = Some(LoadRegister16Nn(DE));
    a[0x12] = Some(LoadIndirectDeA);
    
    a[0x16] = Some(LoadRegisterN(D));
    
    a[0x1A] = Some(LoadAIndirectDe);
    
    a[0x1E] = Some(LoadRegisterN(E));

    a[0x21] = Some(LoadRegister16Nn(HL));
    a[0x22] = Some(LoadIndirectHlIncrementA);
    
    a[0x26] = Some(LoadRegisterN(H));
    
    a[0x2A] = Some(LoadAIndirectHlIncrement);
    
    a[0x2E] = Some(LoadRegisterN(L));

    a[0x31] = Some(LoadRegister16Nn(SP));
    a[0x32] = Some(LoadIndirectHlDecrementA);
    
    a[0x36] = Some(LoadIndirectHlN);
    
    a[0x3A] = Some(LoadAIndirectHlDecrement);
    
    a[0x3E] = Some(LoadRegisterN(A));
    
    a[0x40] = Some(LoadRegisterRegister(B, B));
    a[0x41] = Some(LoadRegisterRegister(B, C));
    a[0x42] = Some(LoadRegisterRegister(B, D));
    a[0x43] = Some(LoadRegisterRegister(B, E));
    a[0x44] = Some(LoadRegisterRegister(B, H));
    a[0x45] = Some(LoadRegisterRegister(B, L));
    a[0x46] = Some(LoadRegisterIndirectHl(B));
    a[0x47] = Some(LoadRegisterRegister(B, A));
    a[0x48] = Some(LoadRegisterRegister(C, B));
    a[0x49] = Some(LoadRegisterRegister(C, C));
    a[0x4A] = Some(LoadRegisterRegister(C, D));
    a[0x4B] = Some(LoadRegisterRegister(C, E));
    a[0x4C] = Some(LoadRegisterRegister(C, H));
    a[0x4D] = Some(LoadRegisterRegister(C, L));
    a[0x4E] = Some(LoadRegisterIndirectHl(C));
    a[0x4F] = Some(LoadRegisterRegister(C, A));
    a[0x50] = Some(LoadRegisterRegister(D, B));
    a[0x51] = Some(LoadRegisterRegister(D, C));
    a[0x52] = Some(LoadRegisterRegister(D, D));
    a[0x53] = Some(LoadRegisterRegister(D, E));
    a[0x54] = Some(LoadRegisterRegister(D, H));
    a[0x55] = Some(LoadRegisterRegister(D, L));
    a[0x56] = Some(LoadRegisterIndirectHl(D));
    a[0x57] = Some(LoadRegisterRegister(D, A));
    a[0x58] = Some(LoadRegisterRegister(E, B));
    a[0x59] = Some(LoadRegisterRegister(E, C));
    a[0x5A] = Some(LoadRegisterRegister(E, D));
    a[0x5B] = Some(LoadRegisterRegister(E, E));
    a[0x5C] = Some(LoadRegisterRegister(E, H));
    a[0x5D] = Some(LoadRegisterRegister(E, L));
    a[0x5E] = Some(LoadRegisterIndirectHl(E));
    a[0x5F] = Some(LoadRegisterRegister(E, A));
    a[0x60] = Some(LoadRegisterRegister(H, B));
    a[0x61] = Some(LoadRegisterRegister(H, C));
    a[0x62] = Some(LoadRegisterRegister(H, D));
    a[0x63] = Some(LoadRegisterRegister(H, E));
    a[0x64] = Some(LoadRegisterRegister(H, H));
    a[0x65] = Some(LoadRegisterRegister(H, L));
    a[0x66] = Some(LoadRegisterIndirectHl(H));
    a[0x67] = Some(LoadRegisterRegister(H, A));
    a[0x68] = Some(LoadRegisterRegister(L, B));
    a[0x69] = Some(LoadRegisterRegister(L, C));
    a[0x6A] = Some(LoadRegisterRegister(L, D));
    a[0x6B] = Some(LoadRegisterRegister(L, E));
    a[0x6C] = Some(LoadRegisterRegister(L, H));
    a[0x6D] = Some(LoadRegisterRegister(L, L));
    a[0x6E] = Some(LoadRegisterIndirectHl(L));
    a[0x6F] = Some(LoadRegisterRegister(L, A));
    a[0x70] = Some(LoadIndirectHlRegister(B));
    a[0x71] = Some(LoadIndirectHlRegister(C));
    a[0x72] = Some(LoadIndirectHlRegister(D));
    a[0x73] = Some(LoadIndirectHlRegister(E));
    a[0x74] = Some(LoadIndirectHlRegister(H));
    a[0x75] = Some(LoadIndirectHlRegister(L));
    
    a[0x77] = Some(LoadIndirectHlRegister(A));
    a[0x78] = Some(LoadRegisterRegister(A, B));
    a[0x79] = Some(LoadRegisterRegister(A, C));
    a[0x7A] = Some(LoadRegisterRegister(A, D));
    a[0x7B] = Some(LoadRegisterRegister(A, E));
    a[0x7C] = Some(LoadRegisterRegister(A, H));
    a[0x7D] = Some(LoadRegisterRegister(A, L));
    a[0x7E] = Some(LoadRegisterIndirectHl(A));
    a[0x7F] = Some(LoadRegisterRegister(A, A));
    a[0x80] = Some(AddRegister(B));
    a[0x81] = Some(AddRegister(C));
    a[0x82] = Some(AddRegister(D));
    a[0x83] = Some(AddRegister(E));
    a[0x84] = Some(AddRegister(H));
    a[0x85] = Some(AddRegister(L));
    a[0x86] = Some(AddIndirectHl);
    a[0x87] = Some(AddRegister(A));
    a[0x88] = Some(AddCarryRegister(B));
    a[0x89] = Some(AddCarryRegister(C));
    a[0x8A] = Some(AddCarryRegister(D));
    a[0x8B] = Some(AddCarryRegister(E));
    a[0x8C] = Some(AddCarryRegister(H));
    a[0x8D] = Some(AddCarryRegister(L));
    a[0x8E] = Some(AdcIndirectHl);
    a[0x8F] = Some(AddCarryRegister(A));

    a[0xC2] = Some(JumpConditionalToNn(NotZero));
    a[0xC3] = Some(JumpToNn);

    a[0xC5] = Some(PushFromRegister(PushPopTarget::BC));
    a[0xC6] = Some(AddN);

    a[0xCA] = Some(JumpConditionalToNn(Zero));

    a[0xCE] = Some(AddCarryN);

    a[0xD2] = Some(JumpConditionalToNn(NotCarry));

    a[0xDA] = Some(JumpConditionalToNn(Carry));

    a[0xD5] = Some(PushFromRegister(PushPopTarget::DE));

    a[0xE0] = Some(LoadHalfNA);

    a[0xE2] = Some(LoadHalfCA);
    // None
    // None

    a[0xE5] = Some(PushFromRegister(PushPopTarget::HL));

    a[0xEA] = Some(LoadNnA);

    a[0xF0] = Some(LoadHalfAN);

    a[0xF2] = Some(LoadHalfAC);

    a[0xFA] = Some(LoadANn);

    a[0xF5] = Some(PushFromRegister(PushPopTarget::AF));

    a[0xF9] = Some(LoadStackPointerFromHl);

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