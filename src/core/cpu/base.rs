use crate::core::instructions::definitions::Instruction;
use crate::core::memory::MemoryBus;
use crate::core::registers::Registers;

#[derive(Debug)]
pub struct CPU {
    pub(super) registers: Registers,
    pub(super) program_counter: u16,
    pub(super) stack_pointer: u16,
    pub(super) bus: MemoryBus
}
impl CPU {
    pub (super) fn new() -> Self {
        CPU{
            registers: Registers::new(),
            program_counter: 0,
            stack_pointer:0,
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
            Instruction::AddRegister(target) => {
                self.add_register(target);
            },
            Instruction::AddIndirectHl => {
                self.add_indirect_hl();
            }
            Instruction::AddN => {
                self.add_n();
            }
            Instruction::AddCarryRegister(target) => {
                self.add_carry_register(target);
            },
            Instruction::AdcIndirectHl =>{
                self.add_carry_indirect_hl();
            }
            Instruction::AddCarryN => {
                self.add_carry_n();
            }
            // 8 bit load
            Instruction::LoadRegisterRegister(source, receiver) => {
                self.load_register_register(source, receiver);
            },
            Instruction::LoadRegisterN(receiver) => {
                self.load_register_n(receiver);
            },
            Instruction::LoadRegisterIndirectHl(receiver) => {
                self.load_register_indirect_hl(receiver);
            },
            Instruction::LoadIndirectHlRegister(source) => {
                self.load_indirect_hl_register(source);
            },
            Instruction::LoadIndirectHlN => {
                self.load_indirect_hl_n();
            }
            Instruction::LoadAIndirectBc => {
                self.load_a_bc();
            }
            Instruction::LoadAIndirectDe => {
                self.load_a_de();
            }
            Instruction::LoadIndirectBcA => {
                self.load_bc_a();
            }
            Instruction::LoadIndirectDeA => {
                self.load_de_a();
            }
            Instruction::LoadANn => {
                self.load_a_nn();
            }
            Instruction::LoadNnA => {
                self.load_nn_a();
            }
            Instruction::LoadHalfAC => {
                self.load_half_a_c();
            }
            Instruction::LoadHalfCA => {
                self.load_half_c_a();
            }
            Instruction::LoadHalfAN => {
                self.load_half_a_n();
            }
            Instruction::LoadHalfNA => {
                self.load_half_n_a();
            }
            Instruction::LoadAIndirectHlDecrement => {
                self.load_a_indirect_hl_decrement();
            }
            Instruction::LoadIndirectHlDecrementA => {
                self.load_indirect_hl_decrement_a();
            }
            Instruction::LoadAIndirectHlIncrement => {
                self.load_a_indirect_hl_increment();
            }
            Instruction::LoadIndirectHlIncrementA => {
                self.load_indirect_hl_increment_a();
            }
            Instruction::LoadRegister16Nn(target) => {
                self.load_register16_nn(target);
            }
            // jump instructions
            Instruction::JumpToNn => {
                self.jump_to_nn();
            }
            Instruction::JumpConditionalToNn(jump_condition) => {
                self.jump_conditional_to_nn(jump_condition);
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

        cpu.execute(Instruction::AddRegister(RegisterTarget::A));

        assert_eq!(0x2, cpu.registers.a);
    }

}