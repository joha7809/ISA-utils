use crate::{
    bits::*,
    consts::REGISTER_LIMIT,
    format_spec::{FieldSpec, get_format_spec},
    traits::{Decodable, Encodable},
    types::{Opcode, Operand, ResolvedInstruction},
};

impl Encodable for ResolvedInstruction {
    type EncodingError = EncodeError;

    /// Encodes the instruction to 32 bit.
    fn encode(&self) -> Result<u32, EncodeError> {
        let format = self.opcode.instruction_format();
        let format_spec = get_format_spec(format);

        // The length of Instruction and its format_spec should always match by design, but lets
        // still verify
        debug_assert_eq!(
            format_spec.fields.len(),
            1 + self.operands.len(),
            "Format spec mismatch for {:?}",
            format
        );

        let mut res: u32 = 0;

        // First encode the opcode
        let opcode_field = &format_spec.fields[0];
        let opcode_range = opcode_field.bit_range();
        set_bits(
            &mut res,
            opcode_range.hi,
            opcode_range.lo,
            self.opcode.code() as u32,
        );

        // Then encode operands
        for (operand_index, (field, operand)) in format_spec.fields[1..]
            .iter()
            .zip(self.operands.iter())
            .enumerate()
        {
            let range = field.bit_range();

            match field {
                FieldSpec::Register(_) => {
                    // Register is always unsigned
                    let val = operand.get_val() as u32;
                    set_bits(&mut res, range.hi, range.lo, val);
                }
                FieldSpec::Immediate(_) => {
                    // NOTE: the code below is technically unsfae, but implementation always
                    // ensures that when an immediate is seen the function returns a some value
                    let signed = self.opcode.immediate_signedness(operand_index).unwrap();

                    let unsigned_val = if signed {
                        // Immediate is signed - use two's complement
                        let signed_val = operand.get_val();

                        // Check if it fits in the given number of bits
                        if !fits_in_signed_bits(signed_val, range.width()) {
                            return Err(EncodeError::SignedImmediateOutOfRange {
                                bits: range.width(),
                                value: signed_val,
                            });
                        }

                        // Convert to u32 using two's complement representation
                        // No bits are lost in the conversion
                        signed_val as u32
                    } else {
                        // Value is unsigned, we can extract it directly
                        // first check if number is negative
                        let val = operand.get_val();
                        if val < 0 {
                            return Err(EncodeError::ExpectedUnsignedImmediate {
                                bits: range.width(),
                                value: val,
                            });
                        }
                        let val = val as usize;
                        if !fits_in_unsigned_bits(val, range.width()) {
                            return Err(EncodeError::UnsignedImmediateOutOfRange {
                                bits: range.width(),
                                value: val,
                            });
                        }
                        val as u32
                    };

                    set_bits(&mut res, range.hi, range.lo, unsigned_val);
                }
                FieldSpec::Opcode(_) => unreachable!("Opcode already encoded"),
            }
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
        for (index, field_spec) in format_spec.fields.iter().enumerate() {
            let bit_range = field_spec.bit_range();

            match field_spec {
                FieldSpec::Opcode(_) => {
                    // Already extracted, skip
                }
                FieldSpec::Register(_) => {
                    let reg_val = get_bits(word, bit_range.hi, bit_range.lo);
                    if reg_val > REGISTER_LIMIT as u32 {
                        return Err(EncodeError::RegisterOutOfRange(reg_val as u8));
                    }
                    operands.push(Operand::Register(reg_val as u8));
                }
                FieldSpec::Immediate(_) => {
                    let operand_index = index - 1;
                    let is_signed = opcode.immediate_signedness(operand_index).unwrap();

                    if !is_signed {
                        // Unsigned immediate
                        let imm_unsigned = get_bits(word, bit_range.hi, bit_range.lo);
                        operands.push(Operand::Immediate(imm_unsigned as isize));
                        continue;
                    }

                    let imm_unsigned = get_bits(word, bit_range.hi, bit_range.lo);
                    // Sign-extend the immediate value
                    let imm_signed = sign_extend(imm_unsigned, bit_range.width());

                    operands.push(Operand::Immediate(imm_signed));
                }
            }
        }

        Ok(ResolvedInstruction { opcode, operands })
    }
}

#[derive(Debug)]
pub enum EncodeError {
    RegisterOutOfRange(u8),
    SignedImmediateOutOfRange { bits: u8, value: isize },
    UnsignedImmediateOutOfRange { bits: u8, value: usize },
    InvalidOpcode(u8),
    ExpectedUnsignedImmediate { bits: u8, value: isize },
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
            EncodeError::SignedImmediateOutOfRange { bits, value } => write!(
                f,
                "Encode error: immediate value {} does not fit in {} signed bits (range: {} to {})",
                value,
                bits,
                -(1isize << (bits - 1)),
                (1isize << (bits - 1)) - 1
            ),
            EncodeError::InvalidOpcode(word) => {
                write!(f, "Encode error: Opcode for {} not found!", word)
            }
            EncodeError::UnsignedImmediateOutOfRange { bits, value } => write!(
                f,
                "Encode error: immediate value {} does not fit in {} unsigned bits (max: {})",
                value,
                bits,
                (1usize << bits) - 1
            ),
            EncodeError::ExpectedUnsignedImmediate { bits, value } => write!(
                f,
                "Encode error: expected unsigned immediate value for {} bits, but got negative value {}",
                bits, value
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::{Decodable, Encodable};
    use crate::types::{Opcode, Operand, ResolvedInstruction};

    #[test]
    fn decode_binary() {
        // LI R20, 573
        // Correct encoding: opcode=9 (1001), R1=20, R2=0 (unused), Imm=573
        // [31:28]=1001, [27:23]=10100, [22:18]=00000, [17:0]=000000001000111101
        let input = 0b10011010000000000000001000111101 as u32;
        let instruction = input.decode();
        assert!(instruction.is_ok());
        let instruction = instruction.unwrap();
        assert_eq!(instruction.opcode.to_string(), "LI");
        // LI uses RI format which only has 2 operands: register + immediate
        // The unused register field is implicit (always 0) per ISA spec
        assert_eq!(instruction.operands.len(), 2);
        assert_eq!(instruction.operands[0].get_val(), 20);
        assert_eq!(instruction.operands[1].get_val(), 573);
    }

    #[test]
    fn encode_decode_roundtrip_li() {
        // Test LI instruction RI format, encoding and decoding
        let instruction = ResolvedInstruction {
            opcode: Opcode::LI,
            operands: vec![Operand::Register(20), Operand::Immediate(573)],
        };

        let encoded = instruction.encode().expect("Failed to encode");
        println!(
            "LI R20, 573 encoded as: 0b{:032b} (0x{:08x})",
            encoded, encoded
        );

        let decoded = encoded.decode().expect("Failed to decode");
        assert_eq!(decoded.opcode, Opcode::LI);
        assert_eq!(decoded.operands.len(), 2);
        assert_eq!(decoded.operands[0].get_val(), 20);
        assert_eq!(decoded.operands[1].get_val(), 573);
    }

    #[test]
    fn encode_decode_roundtrip_add() {
        // Test ADD instruction (R3 format)
        let instruction = ResolvedInstruction {
            opcode: Opcode::ADD,
            operands: vec![
                Operand::Register(1),
                Operand::Register(2),
                Operand::Register(3),
            ],
        };

        let encoded = instruction.encode().expect("Failed to encode");
        println!(
            "ADD R1, R2, R3 encoded as: 0b{:032b} (0x{:08x})",
            encoded, encoded
        );

        let decoded = encoded.decode().expect("Failed to decode");
        assert_eq!(decoded.opcode, Opcode::ADD);
        assert_eq!(decoded.operands.len(), 3);
        assert_eq!(decoded.operands[0].get_val(), 1);
        assert_eq!(decoded.operands[1].get_val(), 2);
        assert_eq!(decoded.operands[2].get_val(), 3);
    }

    #[test]
    fn encode_decode_roundtrip_addi() {
        // Test ADDI instruction (RRI format)
        let instruction = ResolvedInstruction {
            opcode: Opcode::ADDI,
            operands: vec![
                Operand::Register(1),
                Operand::Register(2),
                Operand::Immediate(100),
            ],
        };

        let encoded = instruction.encode().expect("Failed to encode");
        println!(
            "ADDI R1, R2, 100 encoded as: 0b{:032b} (0x{:08x})",
            encoded, encoded
        );

        let decoded = encoded.decode().expect("Failed to decode");
        assert_eq!(decoded.opcode, Opcode::ADDI);
        assert_eq!(decoded.operands.len(), 3);
        assert_eq!(decoded.operands[0].get_val(), 1);
        assert_eq!(decoded.operands[1].get_val(), 2);
        assert_eq!(decoded.operands[2].get_val(), 100);
    }

    #[test]
    fn encode_decode_roundtrip_jr() {
        // Test JR instruction (I format - just immediate)
        let instruction = ResolvedInstruction {
            opcode: Opcode::JR,
            operands: vec![Operand::Immediate(42)],
        };

        let encoded = instruction.encode().expect("Failed to encode");
        println!("JR 42 encoded as: 0b{:032b} (0x{:08x})", encoded, encoded);

        let decoded = encoded.decode().expect("Failed to decode");
        assert_eq!(decoded.opcode, Opcode::JR);
        assert_eq!(decoded.operands.len(), 1);
        assert_eq!(decoded.operands[0].get_val(), 42);
    }

    #[test]
    fn encode_decode_roundtrip_not() {
        // Test NOT instruction (R2 format)
        let instruction = ResolvedInstruction {
            opcode: Opcode::NOT,
            operands: vec![Operand::Register(5), Operand::Register(10)],
        };

        let encoded = instruction.encode().expect("Failed to encode");
        println!(
            "NOT R5, R10 encoded as: 0b{:032b} (0x{:08x})",
            encoded, encoded
        );

        let decoded = encoded.decode().expect("Failed to decode");
        assert_eq!(decoded.opcode, Opcode::NOT);
        assert_eq!(decoded.operands.len(), 2);
        assert_eq!(decoded.operands[0].get_val(), 5);
        assert_eq!(decoded.operands[1].get_val(), 10);
    }

    #[test]
    fn encode_negative_immediate() {
        // Test encoding negative immediate in ADDI
        let instruction = ResolvedInstruction {
            opcode: Opcode::ADDI,
            operands: vec![
                Operand::Register(1),
                Operand::Register(2),
                Operand::Immediate(-50),
            ],
        };

        let encoded = instruction
            .encode()
            .expect("Failed to encode negative immediate");
        let decoded = encoded.decode().expect("Failed to decode");

        assert_eq!(decoded.operands[2].get_val(), -50);
    }
}
