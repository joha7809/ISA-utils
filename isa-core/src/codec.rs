use crate::{
    bits::*,
    consts::REGISTER_LIMIT,
    layout::{FieldKind, InstructionLayout},
    traits::{Decodable, Encodable},
    types::ResolvedInstruction,
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

impl Decodable for ResolvedInstruction {
    type EncodingError = EncodeError;

    fn decode(&self) -> Result<u32, Self::EncodingError> {
        todo!()
    }
}

#[derive(Debug)]
pub enum EncodeError {
    RegisterOutOfRange(u8),
    ImmediateOutOfRange { bits: u8, value: usize },
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
        }
    }
}
