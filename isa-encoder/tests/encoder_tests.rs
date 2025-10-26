/// Tests for the encoder module
/// Focuses on binary encoding correctness and edge cases
use isa_core::{
    traits::{Decodable, Encodable},
    types::{Opcode, Operand, ResolvedInstruction},
};
use isa_encoder::encoder::encode_program;

#[test]
fn test_encode_decode_all_r3_formats() {
    let test_cases = vec![
        (Opcode::ADD, 1, 2, 3),
        (Opcode::SUB, 4, 5, 6),
        (Opcode::MULT, 7, 8, 9),
        (Opcode::OR, 10, 11, 12),
        (Opcode::AND, 13, 14, 15),
    ];

    for (opcode, rd, rs1, rs2) in test_cases {
        let instr = ResolvedInstruction {
            opcode,
            operands: vec![
                Operand::Register(rd),
                Operand::Register(rs1),
                Operand::Register(rs2),
            ],
        };

        let encoded = instr.encode().unwrap();
        let decoded = encoded.decode().unwrap();

        assert_eq!(instr, decoded, "Failed roundtrip for {:?}", opcode);
    }
}

#[test]
fn test_encode_decode_r2_formats() {
    let test_cases = vec![(Opcode::NOT, 1, 2), (Opcode::LD, 3, 4), (Opcode::SD, 5, 6)];

    for (opcode, rd, rs) in test_cases {
        let instr = ResolvedInstruction {
            opcode,
            operands: vec![Operand::Register(rd), Operand::Register(rs)],
        };

        let encoded = instr.encode().unwrap();
        let decoded = encoded.decode().unwrap();

        assert_eq!(instr, decoded, "Failed roundtrip for {:?}", opcode);
    }
}

#[test]
fn test_encode_decode_ri_format() {
    let test_values = vec![0, 1, 100, 1000, 10000, 100000, 1000000];

    for val in test_values {
        let instr = ResolvedInstruction {
            opcode: Opcode::LI,
            operands: vec![Operand::Register(1), Operand::Immediate(val)],
        };

        let encoded = instr.encode().unwrap();
        let decoded = encoded.decode().unwrap();

        assert_eq!(instr, decoded, "Failed roundtrip for LI with value {}", val);
    }
}

#[test]
fn test_encode_decode_rri_format() {
    let test_cases = vec![
        (Opcode::ADDI, 1, 2, 100),
        (Opcode::SUBI, 3, 4, 50),
        (Opcode::JEQ, 5, 6, 10),
        (Opcode::JGT, 7, 8, 20),
    ];

    for (opcode, rd, rs, imm) in test_cases {
        let instr = ResolvedInstruction {
            opcode,
            operands: vec![
                Operand::Register(rd),
                Operand::Register(rs),
                Operand::Immediate(imm),
            ],
        };

        let encoded = instr.encode().unwrap();
        let decoded = encoded.decode().unwrap();

        assert_eq!(instr, decoded, "Failed roundtrip for {:?}", opcode);
    }
}

#[test]
fn test_encode_decode_rii_format() {
    let test_cases = vec![(Opcode::JLTV, 1, 100, 200), (Opcode::JETV, 2, 300, 400)];

    for (opcode, rd, imm1, imm2) in test_cases {
        let instr = ResolvedInstruction {
            opcode,
            operands: vec![
                Operand::Register(rd),
                Operand::Immediate(imm1),
                Operand::Immediate(imm2),
            ],
        };

        let encoded = instr.encode().unwrap();
        let decoded = encoded.decode().unwrap();

        assert_eq!(instr, decoded, "Failed roundtrip for {:?}", opcode);
    }
}

#[test]
fn test_encode_decode_i_format() {
    let test_values = vec![0, 10, 100, 1000, 10000];

    for val in test_values {
        let instr = ResolvedInstruction {
            opcode: Opcode::JR,
            operands: vec![Operand::Immediate(val)],
        };

        let encoded = instr.encode().unwrap();
        let decoded = encoded.decode().unwrap();

        assert_eq!(instr, decoded, "Failed roundtrip for JR with value {}", val);
    }
}

#[test]
fn test_encode_decode_noop_format() {
    let test_cases = vec![Opcode::NOP, Opcode::END];

    for opcode in test_cases {
        let instr = ResolvedInstruction {
            opcode,
            operands: vec![],
        };

        let encoded = instr.encode().unwrap();
        let decoded = encoded.decode().unwrap();

        assert_eq!(instr, decoded, "Failed roundtrip for {:?}", opcode);
    }
}

#[test]
fn test_all_registers_encode_correctly() {
    // Test that all register values (0-31) encode/decode correctly
    for reg in 0..32 {
        let instr = ResolvedInstruction {
            opcode: Opcode::NOT,
            operands: vec![Operand::Register(reg), Operand::Register(0)],
        };

        let encoded = instr.encode().unwrap();
        let decoded = encoded.decode().unwrap();

        assert_eq!(instr, decoded, "Failed roundtrip for register R{}", reg);
    }
}

#[test]
fn test_immediate_boundary_values() {
    // Test boundary values for different immediate sizes
    // For LI (RI format): 22 bits available (32 - 5 opcode - 5 register)
    let max_22_bit = (1 << 21) - 1; // 2^21 - 1
    let min_22_bit = -(1 << 21); // -2^21

    let instr = ResolvedInstruction {
        opcode: Opcode::LI,
        operands: vec![Operand::Register(1), Operand::Immediate(max_22_bit)],
    };

    let instr_min = ResolvedInstruction {
        opcode: Opcode::LI,
        operands: vec![Operand::Register(1), Operand::Immediate(min_22_bit)],
    };

    let encoded1 = instr.encode().unwrap();
    let decoded1 = encoded1.decode().unwrap();
    assert_eq!(instr, decoded1);

    let encoded2 = instr_min.encode().unwrap();
    let decoded2 = encoded2.decode().unwrap();
    assert_eq!(instr_min, decoded2);
}

#[test]
fn test_immediate_too_large_for_ri() {
    // For LI: 22 bits max
    let too_large = 1 << 22; // Just over the limit

    let instr = ResolvedInstruction {
        opcode: Opcode::LI,
        operands: vec![Operand::Register(1), Operand::Immediate(too_large)],
    };

    let result = instr.encode();
    assert!(
        result.is_err(),
        "Should fail encoding for immediate too large"
    );
}

#[test]
fn test_immediate_too_large_for_rri() {
    // For ADDI (RRI format): 17 bits available (32 - 5 opcode - 5 rd - 5 rs)
    let too_large = 1 << 17;

    let instr = ResolvedInstruction {
        opcode: Opcode::ADDI,
        operands: vec![
            Operand::Register(1),
            Operand::Register(2),
            Operand::Immediate(too_large),
        ],
    };

    let result = instr.encode();
    assert!(
        result.is_err(),
        "Should fail encoding for immediate too large"
    );
}

#[test]
fn test_immediate_too_large_for_i() {
    // For JR (I format): 27 bits available (32 - 5 opcode)
    let too_large = 1isize << 27;

    let instr = ResolvedInstruction {
        opcode: Opcode::JR,
        operands: vec![Operand::Immediate(too_large)],
    };

    let result = instr.encode();
    assert!(
        result.is_err(),
        "Should fail encoding for immediate too large"
    );
}

#[test]
fn test_zero_values_encode_correctly() {
    let instr = ResolvedInstruction {
        opcode: Opcode::ADD,
        operands: vec![
            Operand::Register(0),
            Operand::Register(0),
            Operand::Register(0),
        ],
    };

    let encoded = instr.encode().unwrap();
    let decoded = encoded.decode().unwrap();

    assert_eq!(instr, decoded);
}

#[test]
fn test_max_register_values() {
    // R31 is the maximum register
    let instr = ResolvedInstruction {
        opcode: Opcode::ADD,
        operands: vec![
            Operand::Register(31),
            Operand::Register(31),
            Operand::Register(31),
        ],
    };

    let encoded = instr.encode().unwrap();
    let decoded = encoded.decode().unwrap();

    assert_eq!(instr, decoded);
}

#[test]
fn test_opcode_bits_correct() {
    // Verify that opcodes are encoded in the correct bit positions
    // Opcodes should be in bits [31:27]

    let instr = ResolvedInstruction {
        opcode: Opcode::ADD,
        operands: vec![
            Operand::Register(0),
            Operand::Register(0),
            Operand::Register(0),
        ],
    };

    let encoded = instr.encode().unwrap();

    // Extract opcode bits [31:27]
    let opcode_bits = (encoded >> 27) & 0b11111;
    assert_eq!(opcode_bits as u8, Opcode::ADD.code());
}

#[test]
fn test_register_bits_correct() {
    // Test that registers are encoded in correct bit positions for R3 format
    // Format: opcode[31:27] | rd[26:22] | rs1[21:17] | rs2[16:12] | unused[11:0]

    let instr = ResolvedInstruction {
        opcode: Opcode::ADD,
        operands: vec![
            Operand::Register(5),  // rd
            Operand::Register(10), // rs1
            Operand::Register(15), // rs2
        ],
    };

    let encoded = instr.encode().unwrap();

    // Extract register fields
    let rd = (encoded >> 22) & 0b11111;
    let rs1 = (encoded >> 17) & 0b11111;
    let rs2 = (encoded >> 12) & 0b11111;

    assert_eq!(rd, 5);
    assert_eq!(rs1, 10);
    assert_eq!(rs2, 15);
}

#[test]
fn test_immediate_bits_correct_for_li() {
    // LI format: opcode[31:27] | rd[26:22] | imm[21:0]
    let test_immediate = 12345isize;

    let instr = ResolvedInstruction {
        opcode: Opcode::LI,
        operands: vec![Operand::Register(3), Operand::Immediate(test_immediate)],
    };

    let encoded = instr.encode().unwrap();

    // Extract immediate field [21:0]
    let imm = encoded & 0x3FFFFF;

    assert_eq!(imm as isize, test_immediate);
}

#[test]
fn test_different_instructions_produce_different_encodings() {
    let add = ResolvedInstruction {
        opcode: Opcode::ADD,
        operands: vec![
            Operand::Register(1),
            Operand::Register(2),
            Operand::Register(3),
        ],
    };

    let sub = ResolvedInstruction {
        opcode: Opcode::SUB,
        operands: vec![
            Operand::Register(1),
            Operand::Register(2),
            Operand::Register(3),
        ],
    };

    let add_encoded = add.encode().unwrap();
    let sub_encoded = sub.encode().unwrap();

    assert_ne!(
        add_encoded, sub_encoded,
        "Different opcodes should produce different encodings"
    );
}

#[test]
fn test_decode_invalid_opcode() {
    // Create a word with an invalid opcode (e.g., 0b11110 which is not assigned)
    let invalid_opcode = 0b11110u8;
    let word = (invalid_opcode as u32) << 27;

    let result = word.decode();
    assert!(result.is_err(), "Should fail decoding invalid opcode");
}

#[test]
fn test_encode_program_sequence() {
    use isa_encoder::encoder::encode_program;

    let program = vec![
        ResolvedInstruction {
            opcode: Opcode::LI,
            operands: vec![Operand::Register(1), Operand::Immediate(10)],
        },
        ResolvedInstruction {
            opcode: Opcode::ADDI,
            operands: vec![
                Operand::Register(1),
                Operand::Register(1),
                Operand::Immediate(5),
            ],
        },
        ResolvedInstruction {
            opcode: Opcode::END,
            operands: vec![],
        },
    ];

    let encoded = encode_program(&program);
    assert!(encoded.is_ok());
    let words = encoded.unwrap();
    assert_eq!(words.len(), 3);
}

#[test]
fn test_roundtrip_negative_immediate() {
    let program = vec![
        ResolvedInstruction {
            opcode: Opcode::LI,
            operands: vec![Operand::Register(1), Operand::Immediate(-100)],
        },
        ResolvedInstruction {
            opcode: Opcode::ADDI,
            operands: vec![
                Operand::Register(1),
                Operand::Register(1),
                Operand::Immediate(-500),
            ],
        },
        ResolvedInstruction {
            opcode: Opcode::END,
            operands: vec![],
        },
    ];

    let encoded = encode_program(&program);
    if !encoded.is_ok() {
        panic!("Encoding failed: {}", encoded.err().unwrap());
    }
    assert!(encoded.is_ok());
    let encoded_program = encoded.unwrap();

    for instr in encoded_program {
        let decoded = instr.decode();
        assert!(decoded.is_ok());
        let resolved_instr = decoded.unwrap();
        assert!(program.contains(&resolved_instr));
    }
}
