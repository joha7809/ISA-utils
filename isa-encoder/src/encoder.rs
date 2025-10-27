use isa_core::{
    codec::EncodeError,
    traits::{Decodable, Encodable},
    types::ResolvedInstruction,
};

pub fn encode_program(program: &[ResolvedInstruction]) -> Result<Vec<u32>, EncodeError> {
    program.iter().map(|i| i.encode()).collect()
}

pub fn decode_program(program: &[u32]) -> Result<Vec<ResolvedInstruction>, EncodeError> {
    program.iter().map(|i| i.decode()).collect()
}
