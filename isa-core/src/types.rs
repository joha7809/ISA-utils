use crate::FromStr;
use strum_macros::{EnumIter, FromRepr};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, FromRepr)]
pub enum Opcode {
    NOP = 0b0000,
    ADD = 0b0001,
    SUB = 0b0010,
    MULT = 0b0011,
    ADDI = 0b0100,
    SUBI = 0b0101,
    OR = 0b0110,
    NOT = 0b0111,
    AND = 0b1000,
    LI = 0b1001,
    LD = 0b1010,
    SD = 0b1011,
    JR = 0b1100,
    JEQ = 0b1101,
    JLT = 0b1110,
    END = 0b1111,
}

/// Decoded instruction - useful for VM
/// Note this instruction implements Encode and Decode
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedInstruction {
    pub opcode: Opcode,
    pub operands: Vec<Operand>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
/// Instruction Formats, these are dependant on the opcode, and specify how the 32 bit word should
/// be interpreted.
pub enum InstrFormat {
    // These InstrFormats are for validation, they each correspond to one of R, J or I, but with
    // different bit placements.
    R2,   // opcode + reg + reg
    R3,   // opcode + reg + reg + reg
    RI,   // opcode + reg + imm
    RRI,  // opcode + reg + reg + imm
    I,    // opcode + imm
    NoOP, // opcode only
}

impl InstrFormat {
    pub fn size(&self) -> usize {
        match self {
            InstrFormat::R2 => 3,
            InstrFormat::R3 => 4,
            InstrFormat::RI => 3,
            InstrFormat::RRI => 4,
            InstrFormat::I => 2,
            InstrFormat::NoOP => 1,
        }
    }
}

// Simple operand enum without parsing-specific types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operand {
    Register(u8),     // 0..31
    Immediate(isize), // Value that fits in the instruction format
}

impl Operand {
    pub fn get_val(self) -> isize {
        match self {
            Self::Register(n) => n as isize,
            Self::Immediate(n) => n,
        }
    }
}

impl Opcode {
    pub const fn code(self) -> u8 {
        self as u8
    }

    pub fn from_code(code: u8) -> Option<Self> {
        Self::from_repr(code)
    }
    pub fn to_string(self) -> &'static str {
        match self {
            Opcode::ADD => "ADD",
            Opcode::SUB => "SUB",
            Opcode::MULT => "MULT",
            Opcode::ADDI => "ADDI",
            Opcode::SUBI => "SUBI",
            Opcode::OR => "OR",
            Opcode::AND => "AND",
            Opcode::NOT => "NOT",
            Opcode::LI => "LI",
            Opcode::LD => "LD",
            Opcode::SD => "SD",
            Opcode::JR => "JR",
            Opcode::JEQ => "JEQ",
            Opcode::JLT => "JLT",
            Opcode::NOP => "NOP",
            Opcode::END => "END",
        }
    }

    pub const fn immediate_signedness(self, operand_index: usize) -> Option<bool> {
        use Opcode::*;

        match self {
            // Arithmetic instructions - signed immediates
            ADDI | SUBI => {
                match operand_index {
                    2 => Some(true), // Third operand is the immediate
                    _ => None,
                }
            }

            // Load immediate - signed (allows loading negative values)
            LI => {
                match operand_index {
                    1 => Some(true), // Second operand is the immediate
                    _ => None,
                }
            }

            // Jump instructions - unsigned (instruction addresses)
            JR => {
                match operand_index {
                    0 => Some(false), // Only operand is the jump address
                    _ => None,
                }
            }

            JEQ => {
                match operand_index {
                    2 => Some(false), // Third operand is the jump address
                    _ => None,
                }
            }

            // Value comparison + jump - mixed signedness!
            JLT => {
                match operand_index {
                    2 => Some(false), // Third operand is jump address
                    _ => None,
                }
            }

            // Instructions with no immediates
            ADD | SUB | MULT | OR | AND | NOT | LD | SD | NOP | END => None,
        }
    }

    pub fn instruction_format(self) -> InstrFormat {
        match self {
            // ALU instructions
            Opcode::ADD | Opcode::SUB | Opcode::MULT | Opcode::OR | Opcode::AND => InstrFormat::R3,
            Opcode::NOT => InstrFormat::R2,
            Opcode::LI => InstrFormat::RI,
            Opcode::ADDI | Opcode::SUBI => InstrFormat::RRI,
            // Data transfer
            Opcode::LD | Opcode::SD => InstrFormat::R2,
            // Control flow
            Opcode::JR => InstrFormat::I,
            Opcode::JLT => InstrFormat::RRI,
            Opcode::JEQ => InstrFormat::RRI,
            Opcode::NOP | Opcode::END => InstrFormat::NoOP,
        }
    }
}

impl FromStr for Opcode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Opcode::*;
        Ok(match s {
            "ADD" => ADD,
            "SUB" => SUB,
            "MULT" => MULT,
            "ADDI" => ADDI,
            "SUBI" => SUBI,
            "OR" => OR,
            "AND" => AND,
            "NOT" => NOT,
            "LI" => LI,
            "LD" => LD,
            "SD" => SD,
            "JR" => JR,
            "JEQ" => JEQ,
            "JLT" => JLT,
            "NOP" => NOP,
            "END" => END,
            _ => return Err(()),
        })
    }
}

// Function from ResolvedInstruction to information needed to encode
