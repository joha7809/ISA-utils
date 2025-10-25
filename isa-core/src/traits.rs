use crate::types::ResolvedInstruction;

// Lets just return an option, since we wont really have failures here
pub trait ToResolvedInstr {
    fn to_resolved(&self) -> Option<ResolvedInstruction>;
}

pub trait Encodable {
    type EncodingError;
    fn encode(&self) -> Result<u32, Self::EncodingError>;
}

pub trait Decodable {
    type EncodingError;
    fn decode(&self) -> Result<u32, Self::EncodingError>;
}
