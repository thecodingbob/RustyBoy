use crate::core::cpu::base::CPU;
use crate::core::instructions::definitions::JumpCondition;

impl CPU{
    pub (super) fn jp(&mut self, jump_condition: JumpCondition){
        let should_jump = match jump_condition {
            JumpCondition::NotZero => !self.registers.f.zero,
            JumpCondition::Zero => self.registers.f.zero,
            JumpCondition::NotCarry => !self.registers.f.carry,
            JumpCondition::Carry => self.registers.f.carry,
            JumpCondition::Always => true
        };
        // TODO self.jump(jump_condition)
    }
}