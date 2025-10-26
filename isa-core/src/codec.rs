use crate::{
    bits::*,
    consts::REGISTER_LIMIT,
    layout::{FieldKind, InstructionLayout},
    traits::{Decodable, Encodable},
    types::{InstrFormat, Opcode, Operand, ResolvedInstruction},
};

impl Encodable for ResolvedInstruction {
    type EncodingError = EncodeError;

    /// Encodes the instruction to 32 bit.
    fn encode(&self) -> Result<u32, EncodeError> {
        let layout: InstructionLayout = self.into();
        let mut res: u32 = 0;
        for bitfield in layout.fields {
            let val = bitfield.value; // i think this has implicit copy?
            let kind = &bitfield.kind;

            if !fits_in_bits(val as usize, bitfield.width()) {
                // Match could be replaced with instant error, but lets do it the proper way :)
                match kind {
                    FieldKind::Immediate => {
                        return Err(EncodeError::ImmediateOutOfRange {
                            bits: bitfield.width(),
                            value: val as usize,
                        });
                    }
                    // The rest are unreachable. The value of opcode is derived from its to_code
                    // function, which returns 5-bit numbers
                    // Registers have a check in the parser for the max-value
                    _ => unreachable!(),
                }
            }

            set_bits(&mut res, bitfield.hi_bit, bitfield.lo_bit, bitfield.value);
        }

        Ok(res)
    }
}

impl Decodable for u32 {
    type EncodingError = EncodeError;

    fn decode(&self) -> Result<ResolvedInstruction, Self::EncodingError> {
        // First we extract the opcode, this is always in the end of the word, 31-26
        let word = *self;
        let op_num = get_bits(word, 31, 27) as u8;
        let opcode = Opcode::from_code(op_num).ok_or(EncodeError::InvalidOpcode(op_num))?;
        let format = opcode.instruction_format();

        let operands = match format {
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

        Ok(ResolvedInstruction { opcode, operands })
    }
}

#[derive(Debug)]
pub enum EncodeError {
    RegisterOutOfRange(u8),
    ImmediateOutOfRange { bits: u8, value: usize },
    InvalidOpcode(u8),
}

impl std::fmt::Display for EncodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EncodeError::RegisterOutOfRange(r) => {
                write!(
                    f,
                    "Encode error: register R{} is out of range (1..={})",
                    r, REGISTER_LIMIT
                )
            }
            EncodeError::ImmediateOutOfRange { bits, value } => write!(
                f,
                "Encode error: immediate value {} does not fit in {} bits",
                value, bits
            ),
            EncodeError::InvalidOpcode(word) => {
                write!(f, "Encode error: Opcode for {} not found!", word)
            }
        }
    }
}
