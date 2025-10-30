use isa_core::types::Opcode;

use crate::{errors::VMError, memory::Memory, vm::VMState};

/// The executor trait. It contains execute and run method. The execute method returns a result of
/// bool or VMError. The boolean represents if the program has finished i.e. have we encountered
/// the END opcode
pub trait Executor<M: Memory> {
    fn execute(&mut self, state: &mut VMState<M>) -> Result<bool, VMError>;
}

/// The most basic interpreter, all other executors will wrap this. Here the basic instructions are
/// implemented to modify the VMState
pub struct BaseInterpreter;

impl<M: Memory> Executor<M> for BaseInterpreter {
    fn execute(&mut self, state: &mut VMState<M>) -> Result<bool, VMError> {
        // On each execute we get the instruction from the PC and change PC at end of execution
        // On Opcode::END halt is set to true but Ok is returned
        let instruction = state.get_instruction(state.pc)?;

        // Helper closure to extract register indices
        // We index directly since we know the format of all opcodes, and this is enforced
        // in the decoding stage of the VM. This is therefore safe!
        let reg_idx = |i: usize| instruction.operands[i].get_val() as usize;

        let mut jump = false;
        match instruction.opcode {
            Opcode::NOP => {}
            Opcode::END => state.halted = true,
            Opcode::ADD => {
                debug_assert!(instruction.operands.len() == 3);
                let (rs, r1, r2) = (reg_idx(0), reg_idx(1), reg_idx(2));
                let (result, overflow) = state.registers[r1].overflowing_add(state.registers[r2]);
                state.registers[rs] = result;
                if overflow {
                    eprintln!("Warning: Overflow in ADD at PC {}", state.pc);
                }
            }
            Opcode::SUB => {
                debug_assert!(instruction.operands.len() == 3);
                let (rs, r1, r2) = (reg_idx(0), reg_idx(1), reg_idx(2));
                let (result, overflow) = state.registers[r1].overflowing_sub(state.registers[r2]);
                state.registers[rs] = result;
                if overflow {
                    eprintln!("Warning: Overflow in SUB at PC {}", state.pc);
                }
            }
            Opcode::MULT => {
                debug_assert!(instruction.operands.len() == 3);
                let (rs, r1, r2) = (reg_idx(0), reg_idx(1), reg_idx(2));
                let (result, overflow) = state.registers[r1].overflowing_mul(state.registers[r2]);
                state.registers[rs] = result;
                if overflow {
                    eprintln!("Warning: Overflow in MULT at PC {}", state.pc);
                }
            }
            Opcode::ADDI => {
                debug_assert!(instruction.operands.len() == 3);
                let (rs, r1) = (reg_idx(0), reg_idx(1));
                let imm = instruction.operands[2].get_val() as i32;
                let (result, overflow) = state.registers[r1].overflowing_add(imm);
                state.registers[rs] = result;
                if overflow {
                    eprintln!("Warning: Overflow in ADDI at PC {}", state.pc);
                }
            }
            Opcode::SUBI => {
                debug_assert!(instruction.operands.len() == 3);
                let (rs, r1) = (reg_idx(0), reg_idx(1));
                let imm = instruction.operands[2].get_val() as i32;
                let (result, overflow) = state.registers[r1].overflowing_sub(imm);
                state.registers[rs] = result;
                if overflow {
                    eprintln!("Warning: Overflow in ADDI at PC {}", state.pc);
                }
            }
            Opcode::OR => {
                debug_assert!(instruction.operands.len() == 3);
                let (rs, r1, r2) = (reg_idx(0), reg_idx(1), reg_idx(2));
                let result = state.registers[r1] | state.registers[r2];
                state.registers[rs] = result;
            }
            Opcode::AND => {
                debug_assert!(instruction.operands.len() == 3);
                let (rs, r1, r2) = (reg_idx(0), reg_idx(1), reg_idx(2));
                let result = state.registers[r1] & state.registers[r2];
                state.registers[rs] = result;
            }
            Opcode::NOT => {
                debug_assert!(instruction.operands.len() == 2);
                let (rs, r1) = (reg_idx(0), reg_idx(1));
                let result = !state.registers[r1];
                state.registers[rs] = result;
            }
            Opcode::LI => {
                debug_assert!(instruction.operands.len() == 2);
                let rs = reg_idx(0);
                let imm = instruction.operands[1].get_val() as i32;
                state.registers[rs] = imm;
            }
            Opcode::LD => {
                debug_assert!(instruction.operands.len() == 2);
                let (rs, r1) = (reg_idx(0), reg_idx(1));
                let r_val = state.registers[r1];
                if r_val < 0 {
                    return Err(VMError::NegativeMemoryAdress(r_val as isize));
                }
                state.registers[rs] = state.read_mem(r_val as usize)?;
            }
            Opcode::SD => {
                debug_assert!(instruction.operands.len() == 2);
                let (r1, rs) = (reg_idx(0), reg_idx(1));
                let (r_set_val, val) = (state.registers[rs], state.registers[r1]);
                if r_set_val < 0 {
                    return Err(VMError::NegativeMemoryAdress(r_set_val as isize));
                }
                state.write_mem(r_set_val as usize, val)?;
            }
            Opcode::JR => {
                debug_assert!(instruction.operands.len() == 1);
                // the CPU implementation treats this as a unsigned integer
                let imm = instruction.operands[0].get_val() as usize;
                if imm >= state.mem_size {
                    return Err(VMError::InstructionOutOfRange(imm));
                }
                jump = true;
                state.pc = imm;
            }
            Opcode::JEQ => {
                debug_assert!(instruction.operands.len() == 3);

                let (r1, r2) = (reg_idx(0), reg_idx(1));
                let imm = instruction.operands[2].get_val() as usize;
                if imm >= state.mem_size {
                    return Err(VMError::InstructionOutOfRange(imm));
                }
                if state.registers[r1] == state.registers[r2] {
                    jump = true;
                    state.pc = imm;
                }
            }
            Opcode::JLT => {
                debug_assert!(instruction.operands.len() == 3);
                let (r1, r2) = (reg_idx(0), reg_idx(1));
                let imm = instruction.operands[2].get_val() as usize;
                if imm >= state.mem_size {
                    return Err(VMError::InstructionOutOfRange(imm));
                }
                if state.registers[r1] < state.registers[r2] {
                    jump = true;
                    state.pc = imm;
                }
            }
        }

        if !state.halted && !jump {
            state.pc += 1;
        }

        state.cycles += 1;
        Ok(state.halted)
    }
}
