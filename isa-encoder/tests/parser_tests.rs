/// Comprehensive parser tests for the ISA encoder
/// Tests cover: label resolution, error handling, operand validation, and complex programs
use isa_core::types::{Opcode, Operand, ResolvedInstruction};

// Helper to parse assembly source
fn parse_source(source: &str) -> Result<Vec<ResolvedInstruction>, String> {
    // Re-export the parsing logic from main
    let lexer = isa_encoder::lexer::Lexer::new(source);
    let tokens = lexer.lex();

    let mut parser = isa_encoder::parser::Parser::new(tokens);
    parser
        .parse_instructions()
        .map_err(|e| e.display_with_source(source))
}

#[test]
fn test_label_forward_reference() {
    let input = "
        JR forward_label
        NOP
        forward_label:
        END
    ";

    let result = parse_source(input);
    assert!(result.is_ok());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 3);

    // JR should reference instruction index 2 (forward_label)
    assert_eq!(instructions[0].opcode, Opcode::JR);
    if let Operand::Immediate(addr) = instructions[0].operands[0] {
        assert_eq!(addr, 2);
    } else {
        panic!("Expected immediate operand for JR");
    }
}

#[test]
fn test_label_backward_reference() {
    let input = "
        loop_start:
        ADDI R1, R1, 1
        JGT R1, R2, loop_start
    ";

    let result = parse_source(input);
    assert!(result.is_ok());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 2);

    // JGT should reference instruction index 0 (loop_start)
    assert_eq!(instructions[1].opcode, Opcode::JGT);
    if let Operand::Immediate(addr) = instructions[1].operands[2] {
        assert_eq!(addr, 0);
    } else {
        panic!("Expected immediate operand for JGT");
    }
}

#[test]
fn test_multiple_labels_same_location() {
    let input = "
        start:
        loop:
        ADD R1, R2, R3
    ";

    let result = parse_source(input);
    assert!(result.is_ok());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1); // Both labels point to same instruction
}

#[test]
fn test_undefined_label_error() {
    let input = "
        JR undefined_label
        END
    ";

    let result = parse_source(input);
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.contains("Undefined label") || error.contains("undefined_label"));
}

#[test]
fn test_duplicate_label_error() {
    let input = "
        start:
        ADD R1, R2, R3
        start:
        SUB R4, R5, R6
    ";

    let result = parse_source(input);
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.contains("Duplicate label") || error.contains("already defined"));
}

#[test]
fn test_operand_count_mismatch_too_few() {
    let input = "ADD R1, R2"; // ADD needs 3 operands

    let result = parse_source(input);
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.contains("Operand count mismatch") || error.contains("expected 3"));
}

#[test]
fn test_operand_count_mismatch_too_many() {
    let input = "NOP R1, R2"; // NOP needs 0 operands

    let result = parse_source(input);
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(
        error.contains("Operand count mismatch") || error.contains("expected 0"),
        "Unexpected error message: {}",
        error
    );
}

#[test]
fn test_operand_type_mismatch() {
    let input = "ADD R1, 100, R3"; // ADD needs three registers, not reg+imm+reg

    let result = parse_source(input);
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.contains("type mismatch") || error.contains("incorrect operand"));
}

#[test]
fn test_all_r3_instructions() {
    let input = "
        ADD R1, R2, R3
        SUB R4, R5, R6
        MULT R7, R8, R9
        OR R10, R11, R12
        AND R13, R14, R15
    ";

    let result = parse_source(input);
    assert!(result.is_ok());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 5);

    assert_eq!(instructions[0].opcode, Opcode::ADD);
    assert_eq!(instructions[1].opcode, Opcode::SUB);
    assert_eq!(instructions[2].opcode, Opcode::MULT);
    assert_eq!(instructions[3].opcode, Opcode::OR);
    assert_eq!(instructions[4].opcode, Opcode::AND);
}

#[test]
fn test_all_r2_instructions() {
    let input = "
        NOT R1, R2
        LD R3, R4
        SD R5, R6
    ";

    let result = parse_source(input);
    assert!(result.is_ok());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 3);

    assert_eq!(instructions[0].opcode, Opcode::NOT);
    assert_eq!(instructions[1].opcode, Opcode::LD);
    assert_eq!(instructions[2].opcode, Opcode::SD);
}

#[test]
fn test_all_rri_instructions() {
    let input = "
        ADDI R1, R2, 100
        SUBI R3, R4, 50
        JEQ R5, R6, 10
        JGT R7, R8, 20
    ";

    let result = parse_source(input);
    assert!(result.is_ok());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 4);

    assert_eq!(instructions[0].opcode, Opcode::ADDI);
    assert_eq!(instructions[1].opcode, Opcode::SUBI);
    assert_eq!(instructions[2].opcode, Opcode::JEQ);
    assert_eq!(instructions[3].opcode, Opcode::JGT);
}

#[test]
fn test_all_rii_instructions() {
    let input = "
        JLTV R1, 100, 200
        JETV R2, 300, 400
    ";

    let result = parse_source(input);
    assert!(result.is_ok());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 2);

    assert_eq!(instructions[0].opcode, Opcode::JLTV);
    assert_eq!(instructions[1].opcode, Opcode::JETV);
}

#[test]
fn test_ri_instruction() {
    let input = "LI R1, 12345";

    let result = parse_source(input);
    assert!(result.is_ok());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    assert_eq!(instructions[0].opcode, Opcode::LI);
}

#[test]
fn test_i_instruction() {
    let input = "JR 42";

    let result = parse_source(input);
    assert!(result.is_ok());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 1);
    assert_eq!(instructions[0].opcode, Opcode::JR);
}

#[test]
fn test_noop_instructions() {
    let input = "
        NOP
        END
    ";

    let result = parse_source(input);
    assert!(result.is_ok());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 2);
    assert_eq!(instructions[0].opcode, Opcode::NOP);
    assert_eq!(instructions[1].opcode, Opcode::END);
}

#[test]
fn test_negative_immediates() {
    let input = "
        ADDI R1, R2, -10
        LI R3, -999
    ";

    let result = parse_source(input);
    assert!(result.is_ok());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 2);

    if let Operand::Immediate(val) = instructions[0].operands[2] {
        assert_eq!(val, -10);
    } else {
        panic!("Expected negative immediate");
    }
}

#[test]
fn test_complex_program_with_nested_labels() {
    let input = "
        main:
            LI R1, 0
            LI R2, 10
        outer_loop:
            ADDI R1, R1, 1
            LI R3, 0
        inner_loop:
            ADDI R3, R3, 1
            JGT R3, R2, inner_loop
            JGT R1, R2, outer_loop
        done:
            END
    ";

    let result = parse_source(input);
    assert!(result.is_ok());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 8);
}

#[test]
fn test_comments_ignored() {
    let input = "
        # This is a comment
        ADD R1, R2, R3  # inline comment
        # Another comment
        SUB R4, R5, R6
    ";

    let result = parse_source(input);
    assert!(result.is_ok());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 2); // Only two instructions
}

#[test]
fn test_empty_input() {
    let input = "";
    let result = parse_source(input);
    assert!(result.is_err()); // Should error on empty input
}

#[test]
fn test_only_comments_and_whitespace() {
    let input = "
        # Just comments
        
        # More comments
    ";
    let result = parse_source(input);
    assert!(result.is_err()); // Should error - no instructions
}

#[test]
fn test_register_boundary_r0() {
    let input = "ADD R0, R1, R2";
    let result = parse_source(input);
    assert!(result.is_ok()); // R0 should be valid
}

#[test]
fn test_register_boundary_r31() {
    let input = "ADD R31, R30, R29";
    let result = parse_source(input);
    assert!(result.is_ok()); // R31 should be valid (if REGISTER_LIMIT is 32)
}

#[test]
fn test_label_as_jump_target() {
    let input = "
        LI R1, 5
        JEQ R1, R2, target
        ADDI R1, R1, 1
        target:
        END
    ";

    let result = parse_source(input);
    assert!(result.is_ok());
    let instructions = result.unwrap();

    // JEQ operand should point to instruction 3 (the END after target label)
    if let Operand::Immediate(addr) = instructions[1].operands[2] {
        assert_eq!(addr, 3);
    }
}

#[test]
fn test_zero_immediate() {
    let input = "
        LI R1, 0
        ADDI R2, R3, 0
    ";

    let result = parse_source(input);
    assert!(result.is_ok());
    let instructions = result.unwrap();
    assert_eq!(instructions.len(), 2);
}

#[test]
fn test_case_sensitive_opcodes() {
    // Opcodes should be case-sensitive (uppercase only)
    let input = "add R1, R2, R3";
    let result = parse_source(input);
    assert!(result.is_err()); // lowercase should fail
}

#[test]
fn test_label_with_underscores() {
    let input = "
        my_long_label_name:
            NOP
            JR my_long_label_name
    ";

    let result = parse_source(input);
    assert!(result.is_ok());
}

#[test]
fn test_label_with_numbers() {
    let input = "
        loop_1:
            NOP
            JR loop_1
    ";

    let result = parse_source(input);
    assert!(result.is_ok());
}
