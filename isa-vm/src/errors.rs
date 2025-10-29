pub enum VMError {
    InvalidMemoryAdress(usize),
    InstructionOutOfRange(usize),
}

impl std::fmt::Display for VMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VMError::InvalidMemoryAdress(addr) => {
                write!(f, "Invalid memory address accessed: {}", addr)
            }
            VMError::InstructionOutOfRange(pc) => {
                write!(f, "Program counter out of range: {}", pc)
            }
        }
    }
}
