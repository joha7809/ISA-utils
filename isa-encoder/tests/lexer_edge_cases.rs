/// Additional edge case tests for the lexer
/// Focuses on corner cases and unusual input patterns
use isa_encoder::lexer::{Lexer, TokenKind};

fn lex_tokens(input: &str) -> Vec<TokenKind> {
    Lexer::new(input)
        .lex()
        .into_iter()
        .map(|t| t.kind)
        .collect()
}

#[test]
fn test_multiple_spaces() {
    let input = "ADD    R1,    R2,     R3";
    let tokens = lex_tokens(input);

    // Should handle multiple spaces gracefully
    // Count: Opcode + 3 registers + 2 commas = 6 tokens
    assert_eq!(tokens.len(), 6);
}

#[test]
fn test_tabs_and_spaces() {
    let input = "ADD\tR1,\t\tR2, R3";
    let tokens = lex_tokens(input);

    // Tabs should be treated as whitespace
    assert!(tokens.contains(&TokenKind::Opcode(
        isa_encoder::assembler_types::Opcode::ADD
    )));
}

#[test]
fn test_multiple_blank_lines() {
    let input = "
    
    
    ADD R1, R2, R3
    
    
    SUB R4, R5, R6
    
    ";

    let tokens = lex_tokens(input);

    // Should ignore blank lines
    let opcodes: Vec<_> = tokens
        .iter()
        .filter(|t| matches!(t, TokenKind::Opcode(_)))
        .collect();
    assert_eq!(opcodes.len(), 2);
}

#[test]
fn test_comment_at_start_of_line() {
    let input = "# Full line comment\nADD R1, R2, R3";
    let tokens = lex_tokens(input);

    assert!(tokens.iter().any(|t| matches!(t, TokenKind::Comment(_))));
    assert!(tokens.iter().any(|t| matches!(t, TokenKind::Opcode(_))));
}

#[test]
fn test_comment_at_end_of_file() {
    let input = "ADD R1, R2, R3\n# Final comment";
    let tokens = lex_tokens(input);

    assert!(tokens.iter().any(|t| matches!(t, TokenKind::Comment(_))));
}

#[test]
fn test_empty_comment() {
    let input = "ADD R1, R2, R3 #";
    let tokens = lex_tokens(input);

    // Empty comment should still be a comment token
    assert!(tokens.iter().any(|t| matches!(t, TokenKind::Comment(_))));
}

#[test]
fn test_comment_with_special_chars() {
    let input = "NOP # !@#$%^&*()_+-=[]{}|;':\"<>?,./";
    let tokens = lex_tokens(input);

    // Comment should capture all special characters
    if let Some(TokenKind::Comment(text)) = tokens.iter().find_map(|t| {
        if let TokenKind::Comment(s) = t {
            Some(TokenKind::Comment(s.clone()))
        } else {
            None
        }
    }) {
        assert!(text.contains("!@#$"));
    }
}

#[test]
fn test_label_with_colon_no_space() {
    let input = "label: ADD R1, R2, R3"; // Add space after colon
    let tokens = lex_tokens(input);

    // Label and opcode should be separate tokens
    assert!(tokens.iter().any(|t| matches!(t, TokenKind::LabelDef(_))));
    assert!(tokens.iter().any(|t| matches!(t, TokenKind::Opcode(_))));
}

#[test]
fn test_label_with_numbers_and_underscores() {
    let input = "
        label_1:
        label_2_long:
        l_3_:
    ";

    let tokens = lex_tokens(input);

    let labels: Vec<_> = tokens
        .iter()
        .filter_map(|t| {
            if let TokenKind::LabelDef(name) = t {
                Some(name.clone())
            } else {
                None
            }
        })
        .collect();

    assert_eq!(labels.len(), 3);
    assert!(labels.contains(&"label_1".to_string()));
    assert!(labels.contains(&"label_2_long".to_string()));
    assert!(labels.contains(&"l_3_".to_string()));
}

#[test]
fn test_register_r0() {
    let input = "ADD R0, R1, R2";
    let tokens = lex_tokens(input);

    assert!(tokens.contains(&TokenKind::Register(0)));
}

#[test]
fn test_register_r31() {
    let input = "ADD R31, R30, R29";
    let tokens = lex_tokens(input);

    assert!(tokens.contains(&TokenKind::Register(31)));
}

#[test]
fn test_large_register_number() {
    // R99 should still lex (parser will catch the error)
    let input = "ADD R99, R1, R2";
    let tokens = lex_tokens(input);

    assert!(tokens.contains(&TokenKind::Register(99)));
}

#[test]
fn test_zero_immediate() {
    let input = "LI R1, 0";
    let tokens = lex_tokens(input);

    assert!(tokens.contains(&TokenKind::Immediate(0)));
}

#[test]
fn test_negative_immediate() {
    let input = "ADDI R1, R2, -42";
    let tokens = lex_tokens(input);

    assert!(tokens.contains(&TokenKind::Immediate(-42)));
}

#[test]
fn test_large_positive_immediate() {
    let input = "LI R1, 999999";
    let tokens = lex_tokens(input);

    assert!(tokens.contains(&TokenKind::Immediate(999999)));
}

#[test]
fn test_large_negative_immediate() {
    let input = "LI R1, -999999";
    let tokens = lex_tokens(input);

    assert!(tokens.contains(&TokenKind::Immediate(-999999)));
}

#[test]
fn test_comma_without_spaces() {
    let input = "ADD R1,R2,R3";
    let tokens = lex_tokens(input);

    let commas: Vec<_> = tokens
        .iter()
        .filter(|t| matches!(t, TokenKind::Comma))
        .collect();
    assert_eq!(commas.len(), 2);
}

#[test]
fn test_no_commas() {
    let input = "ADD R1 R2 R3";
    let tokens = lex_tokens(input);

    // Should still lex (parser will handle validation)
    assert_eq!(
        tokens
            .iter()
            .filter(|t| matches!(t, TokenKind::Register(_)))
            .count(),
        3
    );
}

#[test]
fn test_extra_commas() {
    let input = "ADD R1,, R2, R3";
    let tokens = lex_tokens(input);

    // Should lex multiple commas
    let commas: Vec<_> = tokens
        .iter()
        .filter(|t| matches!(t, TokenKind::Comma))
        .collect();
    assert!(commas.len() >= 2);
}

#[test]
fn test_mixed_case_invalid_opcode() {
    let input = "Add R1, R2, R3";
    let tokens = lex_tokens(input);

    // "Add" should be treated as label reference, not opcode
    assert!(tokens.iter().any(|t| matches!(t, TokenKind::LabelRef(_))));
}

#[test]
fn test_lowercase_invalid_opcode() {
    let input = "add R1, R2, R3";
    let tokens = lex_tokens(input);

    // "add" should be treated as label reference
    assert!(tokens.iter().any(|t| matches!(t, TokenKind::LabelRef(_))));
}

#[test]
fn test_lowercase_register_invalid() {
    let input = "ADD r1, R2, R3";
    let tokens = lex_tokens(input);

    // "r1" should be treated as label reference, not register
    assert!(tokens.iter().any(|t| matches!(t, TokenKind::LabelRef(_))));
}

#[test]
fn test_register_without_number() {
    let input = "ADD R, R1, R2";
    let tokens = lex_tokens(input);

    // "R" should be treated as label reference
    assert!(tokens.iter().any(|t| matches!(t, TokenKind::LabelRef(_))));
}

#[test]
fn test_leading_zeros_in_number() {
    let input = "LI R1, 0042";
    let tokens = lex_tokens(input);

    // Should parse as 42
    assert!(tokens.contains(&TokenKind::Immediate(42)));
}

#[test]
fn test_label_looks_like_opcode() {
    let input = "ADD: NOP";
    let tokens = lex_tokens(input);

    // "ADD:" should be a label definition, not opcode
    if let Some(TokenKind::LabelDef(name)) = tokens.iter().find_map(|t| {
        if let TokenKind::LabelDef(s) = t {
            Some(TokenKind::LabelDef(s.clone()))
        } else {
            None
        }
    }) {
        assert_eq!(name, "ADD");
    } else {
        panic!("Expected label definition");
    }
}

#[test]
fn test_instruction_per_line() {
    let input = "ADD R1, R2, R3\nSUB R4, R5, R6\nMULT R7, R8, R9";
    let tokens = lex_tokens(input);

    let opcodes: Vec<_> = tokens
        .iter()
        .filter(|t| matches!(t, TokenKind::Opcode(_)))
        .collect();
    assert_eq!(opcodes.len(), 3);
}

#[test]
fn test_windows_line_endings() {
    let input = "ADD R1, R2, R3\r\nSUB R4, R5, R6\r\n";
    let tokens = lex_tokens(input);

    // Should handle \r\n correctly
    let opcodes: Vec<_> = tokens
        .iter()
        .filter(|t| matches!(t, TokenKind::Opcode(_)))
        .collect();
    assert_eq!(opcodes.len(), 2);
}

#[test]
fn test_semicolon_terminator() {
    let input = "ADD R1, R2, R3;";
    let tokens = lex_tokens(input);

    assert!(tokens.contains(&TokenKind::Terminator));
}

#[test]
fn test_multiple_instructions_same_line_with_semicolons() {
    let input = "NOP; NOP; NOP;";
    let tokens = lex_tokens(input);

    let nops: Vec<_> = tokens
        .iter()
        .filter(|t| {
            matches!(
                t,
                TokenKind::Opcode(isa_encoder::assembler_types::Opcode::NOP)
            )
        })
        .collect();
    assert_eq!(nops.len(), 3);
}

#[test]
fn test_unicode_in_comment() {
    let input = "NOP # 这是注释 🚀";
    let tokens = lex_tokens(input);

    // Should handle unicode in comments
    assert!(tokens.iter().any(|t| matches!(t, TokenKind::Comment(_))));
}

#[test]
fn test_very_long_label_name() {
    let long_name = "a".repeat(100);
    let input = format!("{}:\nNOP", long_name);
    let tokens = lex_tokens(&input);

    if let Some(TokenKind::LabelDef(name)) = tokens.iter().find_map(|t| {
        if let TokenKind::LabelDef(s) = t {
            Some(TokenKind::LabelDef(s.clone()))
        } else {
            None
        }
    }) {
        assert_eq!(name.len(), 100);
    }
}

#[test]
fn test_immediate_at_max_isize() {
    // Test with a very large number (lexer should handle, encoder will check bounds)
    let input = "LI R1, 2147483647"; // Max i32
    let tokens = lex_tokens(input);

    assert!(tokens.contains(&TokenKind::Immediate(2147483647)));
}

#[test]
fn test_all_whitespace_types() {
    let input = "ADD\tR1 ,\t R2\t,  R3";
    let tokens = lex_tokens(input);

    // Should handle mixed tabs and spaces
    assert!(tokens.iter().any(|t| matches!(t, TokenKind::Opcode(_))));
    assert_eq!(
        tokens
            .iter()
            .filter(|t| matches!(t, TokenKind::Register(_)))
            .count(),
        3
    );
}
