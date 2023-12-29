use crate::core::instructions::definitions::Instruction;
use crate::core::instructions::definitions::Instruction::{LDABC, LDADE, LDAHLDEC, LDAHLINC, LDBCA, LDDEA, LDHLDECA, LDHLINCA, LDHLN, LDRHL, LDRN, LDRR};
use crate::core::instructions::definitions::RegisterTarget::{A, B, C, D, E, H, L};

static INSTRUCTION_ARR:[Some<Instruction>; 256] = init_instruction_arr();


impl Instruction {

    pub(crate) fn from_byte(byte: u8, is_prefixed: bool) -> Option<Instruction>{
        if is_prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            INSTRUCTION_ARR[byte]
        }
    }

    fn from_byte_prefixed(byte: u8) -> Option<Instruction>{
        None
    }
}

fn init_instruction_arr() -> [Some<Instruction>; 256] {
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

    array
}