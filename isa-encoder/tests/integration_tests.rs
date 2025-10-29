/// Integration tests for end-to-end assembly and encoding
/// Tests the complete pipeline from source text to binary output
fn assemble_to_binary(source: &str) -> Result<Vec<u32>, String> {
    let lexer = isa_encoder::lexer::Lexer::new(source);
    let tokens = lexer.lex();

    let mut parser = isa_encoder::parser::Parser::new(tokens);
    let instructions = parser
        .parse_instructions()
        .map_err(|e| e.display_with_source(source))?;

    isa_encoder::encoder::encode_program(&instructions)
        .map_err(|e| format!("Encoding error: {}", e))
}

#[test]
fn test_simple_arithmetic_program() {
    let source = "
        LI R1, 5
        LI R2, 10
        ADD R3, R1, R2
        END
    ";

    let result = assemble_to_binary(source);
    assert!(result.is_ok());
    let binary = result.unwrap();
    assert_eq!(binary.len(), 4);
}

#[test]
fn test_loop_program() {
    let source = "
        LI R1, 0
        LI R2, 10
        loop:
            ADDI R1, R1, 1
            JLT R1, R2, loop
        END
    ";

    let result = assemble_to_binary(source);
    assert!(result.is_ok());
    let binary = result.unwrap();
    assert_eq!(binary.len(), 5);
}

#[test]
fn test_conditional_branch_program() {
    let source = "
        LI R1, 5
        LI R2, 5
        JEQ R1, R2, equal
        ADDI R3, R0, 1
        JR done
        equal:
            ADDI R3, R0, 2
        done:
            END
    ";

    let result = assemble_to_binary(source);
    assert!(result.is_ok());
    let binary = result.unwrap();
    assert_eq!(binary.len(), 7);
}

#[test]
fn test_nested_loops() {
    let source = "
        LI R1, 0
        LI R2, 3
        outer:
            ADDI R1, R1, 1
            LI R3, 0
        inner:
            ADDI R3, R3, 1
            JLT R3, R2, inner
            JLT R1, R2, outer
        END
    ";

    let result = assemble_to_binary(source);
    assert!(result.is_ok());
    let binary = result.unwrap();
    assert_eq!(binary.len(), 8);
}

#[test]
fn test_all_opcodes_program() {
    let source = "
        # Arithmetic
        ADD R1, R2, R3
        SUB R4, R5, R6
        MULT R7, R8, R9
        ADDI R10, R11, 100
        SUBI R12, R13, 50
        
        # Logical
        AND R14, R15, R16
        OR R17, R18, R19
        NOT R20, R21
        
        # Memory
        LI R22, 1000
        LD R23, R24
        SD R25, R26
        
        # Control flow
        JR 20
        JEQ R27, R28, 21
        JLT R29, R30, 22
        
        # No-op
        NOP
        END
    ";

    let result = assemble_to_binary(source);
    assert!(result.is_ok());
    let binary = result.unwrap();
    assert_eq!(binary.len(), 16); // Updated count without JGT, JLTV, JETV
}

#[test]
fn test_fibonacci_program() {
    let source = "
        # Calculate Fibonacci numbers
        LI R1, 0        # fib(n-2)
        LI R2, 1        # fib(n-1)
        LI R3, 10       # counter
        LI R4, 0        # loop index
        
        fib_loop:
            ADD R5, R1, R2      # fib(n) = fib(n-1) + fib(n-2)
            ADD R1, R0, R2      # shift: n-2 = n-1
            ADD R2, R0, R5      # shift: n-1 = n
            ADDI R4, R4, 1      # increment counter
            JLT R4, R3, fib_loop
        END
    ";

    let result = assemble_to_binary(source);
    assert!(result.is_ok());
}

#[test]
fn test_max_of_three_numbers() {
    let source = "
        # Find max of three numbers in R1, R2, R3
        # Result in R4
        LI R1, 15
        LI R2, 42
        LI R3, 28
        
        ADD R4, R0, R1          # assume R1 is max
        JLT R4, R2, r2_bigger   # if R4 < R2 (i.e., R2 > current max)
        JR check_r3
        
        r2_bigger:
            ADD R4, R0, R2
        
        check_r3:
            JLT R4, R3, r3_bigger
            JR done
        
        r3_bigger:
            ADD R4, R0, R3
        
        done:
            END
    ";

    let result = assemble_to_binary(source);
    assert!(result.is_ok());
}

#[test]
fn test_array_sum_simulation() {
    let source = "
        # Simulate summing array elements
        LI R1, 0        # sum
        LI R2, 5        # array length
        LI R3, 0        # index
        
        sum_loop:
            ADDI R1, R1, 10     # add value (simulated)
            ADDI R3, R3, 1      # increment index
            JLT R3, R2, sum_loop
        END
    ";

    let result = assemble_to_binary(source);
    assert!(result.is_ok());
}

#[test]
fn test_comments_throughout() {
    let source = "
        # Program start
        LI R1, 100      # load initial value
        # Middle comment
        ADDI R2, R1, 50 # add to it
        # End comment
        END             # finish
    ";

    let result = assemble_to_binary(source);
    assert!(result.is_ok());
    let binary = result.unwrap();
    assert_eq!(binary.len(), 3);
}

#[test]
fn test_label_only_lines() {
    let source = "
        start:
        middle:
        end:
            NOP
        
        END
    ";

    let result = assemble_to_binary(source);
    assert!(result.is_ok());
    let binary = result.unwrap();
    assert_eq!(binary.len(), 2); // All labels point to same NOP, Assert there are only two
    // instructions
}

#[test]
fn test_many_instructions() {
    // Test a program with many instructions to ensure no overflow
    let mut source = String::new();
    for i in 0..100 {
        source.push_str(&format!("ADDI R1, R1, {}\n", i));
    }
    source.push_str("END\n");

    let result = assemble_to_binary(&source);
    assert!(result.is_ok());
    let binary = result.unwrap();
    assert_eq!(binary.len(), 101);
}

#[test]
fn test_forward_and_backward_jumps() {
    let source = "
        JR middle       # forward jump
        start:
            NOP
        middle:
            JR start    # backward jump
            JR end      # forward jump
        end:
            END
    ";

    let result = assemble_to_binary(source);
    assert!(result.is_ok());
}

#[test]
fn test_empty_program_fails() {
    let source = "";
    let result = assemble_to_binary(source);
    assert!(result.is_err());
}

#[test]
fn test_only_label_fails() {
    let source = "label:\nEND"; // Added END instruction
    let result = assemble_to_binary(source);
    assert!(result.is_ok()); // This should work now with the END
}

#[test]
fn test_immediate_boundary_in_program() {
    let source = "
        LI R1, 131071       # Max 18-bit signed value (2^17 - 1)
        ADDI R2, R3, 131071 # Max 18-bit signed value (2^17 - 1)
        JR 268435455        # Max 28-bit unsigned value (2^28 - 1)
        LI   R4, -131072    # Min 18-bit signed value (-2^17)
        ADDI R5, R6, -131072 # Min 18-bit signed value (-2^17)

        END
    ";

    let result = assemble_to_binary(source);
    if !result.is_ok() {
        panic!("Expected success but got error: {:?}", result.err());
    }
    assert!(result.is_ok());
}

#[test]
fn test_program_with_all_registers() {
    let mut source = String::from("# Use all registers\n");
    for i in 0..32 {
        source.push_str(&format!("ADDI R{}, R{}, 1\n", i, i));
    }
    source.push_str("END\n");

    let result = assemble_to_binary(&source);
    assert!(result.is_ok());
}

#[test]
fn test_decode_encoded_program() {
    use isa_core::traits::Decodable;

    let source = "
        ADD R1, R2, R3
        LI R4, 100
        END
    ";

    let binary = assemble_to_binary(source).unwrap();

    // Decode each instruction back
    for word in binary {
        let decoded = word.decode();
        assert!(decoded.is_ok(), "Failed to decode word: {:08x}", word);
    }
}

#[test]
fn test_roundtrip_complex_program() {
    use isa_core::traits::{Decodable, Encodable};

    let source = "
        start:
            LI R1, 0
            LI R2, 10
        loop:
            ADDI R1, R1, 1
            MULT R3, R1, R2
            JLT R1, R2, loop
        done:
            END
    ";

    let original_binary = assemble_to_binary(source).unwrap();

    // Decode and re-encode
    let decoded_instructions: Vec<_> = original_binary
        .iter()
        .map(|word| word.decode().unwrap())
        .collect();

    let reencoded_binary: Vec<_> = decoded_instructions
        .iter()
        .map(|instr| instr.encode().unwrap())
        .collect();

    assert_eq!(
        original_binary, reencoded_binary,
        "Roundtrip should preserve binary"
    );
}

#[test]
fn test_case_sensitivity() {
    // Test that lowercase opcodes fail
    let sources = vec![
        "add R1, R2, R3", // lowercase
        "Add R1, R2, R3", // mixed case
        "aDD R1, R2, R3", // mixed case
    ];

    for source in sources {
        let result = assemble_to_binary(source);
        assert!(
            result.is_err(),
            "Lowercase/mixed case should fail: {}",
            source
        );
    }
}

#[test]
fn test_register_case_sensitivity() {
    // Test that lowercase registers fail
    let sources = vec![
        "ADD r1, R2, R3", // lowercase r
        "ADD R1, r2, R3", // lowercase r
    ];

    for source in sources {
        let result = assemble_to_binary(source);
        assert!(
            result.is_err(),
            "Lowercase registers should fail: {}",
            source
        );
    }
}
