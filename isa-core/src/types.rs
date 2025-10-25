use crate::{FromStr, bits};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum Opcode {
    ADD,
    SUB,
    MULT,
    ADDI,
    SUBI,
    OR,
    AND,
    NOT,

    // DATA
    LI,
    LD,
    SD,

    // Control
    JR,
    JEQ,
    JLTV,
    JGT,
    JETV,
    NOP,
    END,
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
pub enum InstrFormat {
    R2,   // opcode + reg + reg
    R3,   // opcode + reg + reg + reg
    RI,   // opcode + reg + imm
    RRI,  // opcode + reg + reg + imm
    RII,  // opcode + reg + imm + imm
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
            InstrFormat::RII => 4,
            InstrFormat::I => 2,
            InstrFormat::NoOP => 1,
        }
    }
}

// Simple operand enum without parsing-specific types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operand {
    Register(u8),     // 0..31
    Immediate(usize), // Value that fits in the instruction format
}

impl Opcode {
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
            Opcode::JGT => "JGT",
            Opcode::JEQ => "JEQ",
            Opcode::JLTV => "JLTV",
            Opcode::JETV => "JETV",
            Opcode::NOP => "NOP",
            Opcode::END => "END",
        }
    }

    /// Returns the 5-bit binary encoding for this opcode
    pub const fn code(self) -> u8 {
        match self {
            // ALU
            Opcode::ADD => 0b00001,
            Opcode::SUB => 0b00010,
            Opcode::MULT => 0b00011,
            Opcode::ADDI => 0b00100,
            Opcode::SUBI => 0b00101,
            Opcode::OR => 0b00110,
            Opcode::NOT => 0b00111,
            Opcode::AND => 0b10000,

            // DATA TRANSFER
            Opcode::LI => 0b01000,
            Opcode::LD => 0b01001,
            Opcode::SD => 0b01010,

            // CONTROL
            Opcode::JR => 0b01011,
            Opcode::JEQ => 0b01100,
            Opcode::JLTV => 0b01101,
            Opcode::JGT => 0b01110,
            Opcode::JETV => 0b01111,
            Opcode::NOP => 0b00000,
            Opcode::END => 0b11111,
        }
    }

    /// Decode a 5-bit opcode value back to an Opcode enum
    pub fn from_code(code: u8) -> Option<Opcode> {
        match code {
            0b00001 => Some(Opcode::ADD),
            0b00010 => Some(Opcode::SUB),
            0b00011 => Some(Opcode::MULT),
            0b00100 => Some(Opcode::ADDI),
            0b00101 => Some(Opcode::SUBI),
            0b00110 => Some(Opcode::OR),
            0b00111 => Some(Opcode::NOT),
            0b10000 => Some(Opcode::AND),
            0b01000 => Some(Opcode::LI),
            0b01001 => Some(Opcode::LD),
            0b01010 => Some(Opcode::SD),
            0b01011 => Some(Opcode::JR),
            0b01100 => Some(Opcode::JEQ),
            0b01101 => Some(Opcode::JLTV),
            0b01110 => Some(Opcode::JGT),
            0b01111 => Some(Opcode::JETV),
            0b00000 => Some(Opcode::NOP),
            0b11111 => Some(Opcode::END),
            _ => None,
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
            Opcode::JLTV => InstrFormat::RII,
            Opcode::JEQ | Opcode::JGT => InstrFormat::RRI,
            Opcode::JETV => InstrFormat::RII,
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
            "JGT" => JGT,
            "JEQ" => JEQ,
            "JLTV" => JLTV,
            "JETV" => JETV,
            "NOP" => NOP,
            "END" => END,
            _ => return Err(()),
        })
    }
}

impl ResolvedInstruction {
    // TODO: Move to encoder, and implement it as trait
    /// Decode a 32-bit instruction word into its components
    pub fn decode(word: u32) -> Option<Self> {
        use bits::get_bits;

        // Extract opcode from bits [31:27]
        let opcode_bits = get_bits(word, 31, 27) as u8;
        let opcode = Opcode::from_code(opcode_bits)?;

        let operands = match opcode.instruction_format() {
            InstrFormat::R3 => {
                let r1 = get_bits(word, 26, 22) as u8;
                let r2 = get_bits(word, 21, 17) as u8;
                let r3 = get_bits(word, 16, 12) as u8;
                vec![
                    Operand::Register(r1),
                    Operand::Register(r2),
                    Operand::Register(r3),
                ]
            }
            InstrFormat::R2 => {
                let r1 = get_bits(word, 26, 22) as u8;
                let r2 = get_bits(word, 21, 17) as u8;
                vec![Operand::Register(r1), Operand::Register(r2)]
            }
            InstrFormat::RI => {
                let r1 = get_bits(word, 26, 22) as u8;
                let imm = get_bits(word, 21, 0) as usize;
                vec![Operand::Register(r1), Operand::Immediate(imm)]
            }
            InstrFormat::RRI => {
                let r1 = get_bits(word, 26, 22) as u8;
                let r2 = get_bits(word, 21, 17) as u8;
                let imm = get_bits(word, 16, 0) as usize;
                vec![
                    Operand::Register(r1),
                    Operand::Register(r2),
                    Operand::Immediate(imm),
                ]
            }
            InstrFormat::RII => {
                let r1 = get_bits(word, 26, 22) as u8;
                let imm1 = get_bits(word, 21, 11) as usize;
                let imm2 = get_bits(word, 10, 0) as usize;
                vec![
                    Operand::Register(r1),
                    Operand::Immediate(imm1),
                    Operand::Immediate(imm2),
                ]
            }
            InstrFormat::I => {
                let imm = get_bits(word, 26, 0) as usize;
                vec![Operand::Immediate(imm)]
            }
            InstrFormat::NoOP => vec![],
        };

        Some(ResolvedInstruction { opcode, operands })
    }
}

// Function from ResolvedInstruction to information needed to encode
