use isa_core::{codec::EncodeError, traits::Encodable, types::ResolvedInstruction};

pub fn encode_program(program: &[ResolvedInstruction]) -> Result<Vec<u32>, EncodeError> {
    program.iter().map(|i| i.encode()).collect()
}
