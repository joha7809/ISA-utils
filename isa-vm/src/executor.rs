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
        let mut jump = false;
        match instruction.opcode {
            Opcode::NOP => {}
            Opcode::END => state.halted = false,
            Opcode::ADD => {
                // Get the register index values -- not the values stored in the register
                // Note the isa-core returns it as isize for easier encoding logic, but the
                // register values are enforced to be u8 in the range 0..MAX_REGISTER
                // We index directly since we know the format of all opcodes, and this is enforced
                // in the decoding stage of the VM. This is therefore safe!
                debug_assert!(instruction.operands.len() == 3);

                let rs = instruction.operands[0].get_val() as usize;
                let r1 = instruction.operands[1].get_val() as usize;
                let r2 = instruction.operands[2].get_val() as usize;

                let (result, overflow) = state.registers[r1].overflowing_add(state.registers[r2]);
                state.registers[rs] = result;
                if overflow {
                    eprintln!("Warning: Overflow in ADD at PC {}", state.pc);
                }
            }

            _ => unimplemented!(),
        }

        todo!()
    }
}
