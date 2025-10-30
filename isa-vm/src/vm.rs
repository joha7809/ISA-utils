use isa_core::traits::Decodable;
use isa_core::types::ResolvedInstruction;

use crate::{errors::VMError, executor::Executor, memory::Memory};

pub struct VM<M: Memory, E: Executor<M>> {
    pub executor: E,
    pub state: VMState<M>,
}

pub struct VMState<M: Memory> {
    pub registers: [i32; 32],
    pub memory: M,
    pub mem_size: usize,
    pub program: Vec<ResolvedInstruction>,
    pub pc: usize,
    pub halted: bool,
    pub cycles: usize,
}

impl<M: Memory> VMState<M> {
    pub fn read_mem(&self, address: usize) -> Result<i32, VMError> {
        if address > self.memory.size() - 1 {
            return Err(VMError::InvalidMemoryAdress(address));
        }
        Ok(self.memory.read(address))
    }

    pub fn write_mem(&mut self, address: usize, value: i32) -> Result<(), VMError> {
        if address > self.memory.size() - 1 {
            return Err(VMError::InvalidMemoryAdress(address));
        }

        self.memory.store(address, value);
        Ok(())
    }

    pub fn get_instruction(&self, pc: usize) -> Result<&ResolvedInstruction, VMError> {
        self.program
            .get(pc)
            .ok_or(VMError::InstructionOutOfRange(pc))
    }
}
