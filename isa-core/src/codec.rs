use crate::{
    bits::*,
    consts::REGISTER_LIMIT,
    format_spec::{FieldSpec, get_format_spec},
    traits::{Decodable, Encodable},
    types::{InstrFormat, Opcode, Operand, ResolvedInstruction},
};

impl Encodable for ResolvedInstruction {
    type EncodingError = EncodeError;

    /// Encodes the instruction to 32 bit.
    fn encode(&self) -> Result<u32, EncodeError> {
        let format = self.opcode.instruction_format();
        let format_spec = get_format_spec(format);

        // Iterator of values starting with opcode value
        let val_iter = std::iter::once(self.opcode.code() as usize)
            .chain(self.operands.iter().map(|o| o.get_val()));

        let mut res: u32 = 0;
        for (field, val) in format_spec.fields.iter().zip(val_iter) {
            let range = field.bit_range();
            if !fits_in_bits(val, range.width()) {
                // Match could be replaced with instant error, but lets do it the proper way :)
                match field {
                    FieldSpec::Immediate(bitrange) => {
                        return Err(EncodeError::ImmediateOutOfRange {
                            bits: bitrange.width(),
                            value: val,
                        });
                    }
                    // The rest are unreachable. The value of opcode is derived from its to_code
                    // function, which returns 5-bit numbers
                    // Registers have a check in the parser for the max-value
                    _ => unreachable!(),
                }
            }

            set_bits(&mut res, range.hi, range.lo, val as u32);
        }

        Ok(res)
    }
}

impl Decodable for u32 {
    type EncodingError = EncodeError;

    fn decode(&self) -> Result<ResolvedInstruction, Self::EncodingError> {
        let word = *self;

        // Retrieve the u8 num of the opcode
        let opcode_range = crate::format_spec::OPCODE_RANGE;
        let op_num = get_bits(word, opcode_range.hi, opcode_range.lo) as u8;

        let opcode = Opcode::from_code(op_num).ok_or(EncodeError::InvalidOpcode(op_num))?;
        let format = opcode.instruction_format();

        // Get the format specification for this instruction format
        let format_spec = get_format_spec(format);
        let mut operands = Vec::new();

        // Extract operands based on format specification
        for field_spec in format_spec.fields {
            let bit_range = field_spec.bit_range();

            match field_spec {
                FieldSpec::Opcode(_) => {
                    // Already extracted, skip
                }
                FieldSpec::Register(_) => {
                    let reg = get_bits(word, bit_range.hi, bit_range.lo) as u8;
                    operands.push(Operand::Register(reg));
                }
                FieldSpec::Immediate(_) => {
                    // For NoOP format, skip the padding immediate
                    if format != InstrFormat::NoOP {
                        let imm = get_bits(word, bit_range.hi, bit_range.lo) as usize;
                        operands.push(Operand::Immediate(imm));
                    }
                }
            }
        }

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
