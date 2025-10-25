#[cfg(test)]
use crate::{
    bits,
    traits::Encodable,
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
        opcode: Opcode::ADD,
        operands: vec![
            Operand::Register(1),
            Operand::Register(2),
            Operand::Register(3),
        ],
    };
    let encoded = instr.encode().unwrap();
    let decoded = ResolvedInstruction::decode(encoded).unwrap();
    assert_eq!(instr, decoded);

    // Test RI format: LI R4, 100
    let instr = ResolvedInstruction {
        opcode: Opcode::LI,
        operands: vec![Operand::Register(4), Operand::Immediate(100)],
    };
    let encoded = instr.encode().unwrap();
    let decoded = ResolvedInstruction::decode(encoded).unwrap();
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
