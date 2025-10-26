use isa_core::{
    bits,
    codec::EncodeError,
    traits::{Decodable, Encodable},
    types::{Opcode, Operand, ResolvedInstruction},
};

#[test]
fn test_opcode_roundtrip() {
    for opcode in [
        Opcode::ADD,
        Opcode::SUB,
        Opcode::MULT,
        Opcode::LI,
        Opcode::LD,
        Opcode::JR,
        Opcode::NOP,
        Opcode::END,
    ] {
        let code = opcode.code();
        assert_eq!(Opcode::from_code(code), Some(opcode));
    }
}

#[test]
fn test_instruction_decode_encode() {
    // Test R3 format: ADD R1, R2, R3
    let instr = ResolvedInstruction {
        opcode: Opcode::MULT,
        operands: vec![
            Operand::Register(1),
            Operand::Register(2),
            Operand::Register(3),
        ],
    };
    let encoded = instr.encode().unwrap();
    let decoded = encoded.decode().unwrap();
    assert_eq!(instr, decoded);

    // Test RI format: LI R4, 100
    let instr = ResolvedInstruction {
        opcode: Opcode::LI,
        operands: vec![Operand::Register(4), Operand::Immediate(100)],
    };
    let encoded = instr.encode().unwrap();
    let decoded = encoded.decode().unwrap();
    assert_eq!(instr, decoded);
}

#[test]
fn test_bit_manipulation() {
    use bits::*;

    let mut word = 0u32;
    set_bits(&mut word, 31, 27, 0b11111);
    assert_eq!(get_bits(word, 31, 27), 0b11111);

    set_bits(&mut word, 26, 22, 0b10101);
    assert_eq!(get_bits(word, 26, 22), 0b10101);
    assert_eq!(get_bits(word, 31, 27), 0b11111); // Previous bits unchanged
}

// Test a immediate value that is too big
#[test]
fn test_signed_immediate_too_large() {
    let instr = ResolvedInstruction {
        opcode: Opcode::LI,
        // we need number that is too large for 22 bits
        operands: vec![Operand::Register(0), Operand::Immediate(isize::MAX)],
    };
    let result = instr.encode();
    println!("Error: {:?}", result);
    dbg!(&result);
    assert!(result.is_err());
    let err = instr.encode().unwrap_err();
    assert!(matches!(
        err,
        EncodeError::SignedImmediateOutOfRange { bits: 22, value: _ } //22 since LI takes register
                                                                      //immediate => 32-5-5 = 22
    ));
}

#[test]
fn test_unsigned_immediate_too_large() {
    let instr = ResolvedInstruction {
        opcode: Opcode::JR,
        // we need number that is too large for 27 bits
        operands: vec![Operand::Immediate(1 << 30)],
    };
    let result = instr.encode();
    assert!(result.is_err());
    let err = instr.encode().unwrap_err();
    assert!(matches!(
        err,
        EncodeError::UnsignedImmediateOutOfRange { bits: 27, value: _ } //27 since JR takes only immediate => 32-5 = 27
    ));
}

#[test]
fn test_negative_val_when_unsigned_expected() {
    let instr = ResolvedInstruction {
        opcode: Opcode::JR,
        operands: vec![Operand::Immediate(-1)],
    };
    let result = instr.encode();
    assert!(result.is_err());
    let err = instr.encode().unwrap_err();
    assert!(matches!(
        err,
        EncodeError::ExpectedUnsignedImmediate { bits: 27, value: _ } //27 since JR takes only immediate => 32-5 = 27
    ));
}
