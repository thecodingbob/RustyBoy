use crate::core::instructions::definitions::Instruction;
use crate::core::memory::MemoryBus;
use crate::core::registers::Registers;

#[derive(Debug)]
pub struct CPU {
    pub (super) registers: Registers,
    pub (super) program_counter: u16,
    pub (super) bus: MemoryBus
}
impl CPU {
    pub (super) fn new() -> Self {
        CPU{
            registers: Registers::new(),
            program_counter: 0,
            bus: MemoryBus::new()
        }
    }

    fn step(&mut self){
        let mut instruction_byte = self.read_and_increment_pc();
        let is_prefixed = instruction_byte == 0xCB;
        if is_prefixed {
            instruction_byte = self.read_and_increment_pc();
        }
        if let Some(instruction) = Instruction::from_byte(instruction_byte, is_prefixed){
            self.execute(instruction);
        } else {
            let description = format!("0x{}{:x}", if is_prefixed { "cb" } else { "" }, instruction_byte);
            panic!("Unknown instruction for: 0x{}", description)
        };
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            // 8 bit arithmetic
            Instruction::ADDR(target) => {
                self.add_r(target);
            },
            Instruction::ADDHL => {
                self.add_hl();
            },
            Instruction::ADDN => {
                self.add_n();
            }
            Instruction::ADCR(target) => {
                self.adc_r(target);
            },
            Instruction::ADCHL =>{
                self.adc_hl();
            },
            Instruction::ADCN => {
                self.adc_n();
            }
            // 8 bit load
            Instruction::LDRR(source, receiver) => {
                self.ld_r_r(source, receiver);
            },
            Instruction::LDRN(receiver) => {
                self.ld_r_n(receiver);
            },
            Instruction::LDRHL(receiver) => {
                self.ld_r_hl(receiver);
            },
            Instruction::LDHLR(source) => {
                self.ld_hl_r(source);
            },
            Instruction::LDHLN => {
                self.ld_hl_n();
            },
            Instruction::LDABC => {
                self.ld_a_bc();
            },
            Instruction::LDADE => {
                self.ld_a_de();
            },
            Instruction::LDBCA => {
                self.ld_bc_a();
            },
            Instruction::LDDEA => {
                self.ld_de_a();
            },
            Instruction::LDANN => {
                self.ld_a_nn();
            },
            Instruction::LDNNA => {
                self.ld_nn_a();
            },
            Instruction::LDHAC => {
                self.ld_h_a_c();
            },
            Instruction::LDHCA => {
                self.ld_h_c_a();
            },
            Instruction::LDHAN => {
                self.ld_h_a_n();
            }
            Instruction::LDHNA => {
                self.ld_h_n_a();
            },
            Instruction::LDAHLDEC => {
                self.ld_a_hl_dec();
            },
            Instruction::LDHLDECA => {
                self.ld_hl_dec_a();
            },
            Instruction::LDAHLINC => {
                self.ld_a_hl_inc();
            },
            Instruction::LDHLINCA => {
                self.ld_hl_inc_a();
            },
            // jump instructions
            Instruction::JP(jump_condition) => {
                self.jp(jump_condition);
            }
        }
    }

}

#[cfg(test)]
mod test{
    use crate::core::cpu::base::CPU;
    use crate::core::instructions::definitions::{Instruction, RegisterTarget};

    #[test]
    fn test_step(){
        let mut cpu = CPU::new();

        //cpu.step();
    }

    #[test]
    fn test_execute(){
        let mut cpu = CPU::new();
        cpu.registers.a = 0x1;

        cpu.execute(Instruction::ADDR(RegisterTarget::A));

        assert_eq!(0x2, cpu.registers.a);
    }

}