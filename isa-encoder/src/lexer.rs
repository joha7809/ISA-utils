use isa_core::consts::REGISTER_LIMIT;
use std::{fmt::Display, str::FromStr};

use crate::isa::{self, Opcode};

/// Represents one lexical unit (token) in the assembly code.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Opcode(Opcode),
    Register(usize),
    Immediate(usize),
    LabelDef(String), // e.g., "loop:" defines a label
    LabelRef(String), // e.g., used in a jump instruction
    Comma,
    Comment(String),
    Terminator,
}

/// A token instance with its type and position in the input string.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

// Implement readable debuf display for Token
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            TokenKind::Opcode(op) => write!(f, "Opcode({:?})", op),
            TokenKind::Register(reg) => write!(f, "Register(R{})", reg),
            TokenKind::Immediate(imm) => write!(f, "Immediate({})", imm),
            TokenKind::LabelDef(label) => write!(f, "LabelDef({})", label),
            TokenKind::LabelRef(label) => write!(f, "LabelRef({})", label),
            TokenKind::Comma => write!(f, "Comma(,)"),
            TokenKind::Comment(comment) => write!(f, "Comment(# {})", comment),
            TokenKind::Terminator => write!(f, "Terminator(;)"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
}

impl From<(usize, usize, usize)> for Span {
    fn from(tuple: (usize, usize, usize)) -> Span {
        Span {
            start: tuple.0,
            end: tuple.1,
            line: tuple.2,
        }
    }
}

pub struct Lexer<'a> {
    // Returns a vector of tokens (Opcode and Operands) from the input stringin
    chars: std::iter::Peekable<std::str::CharIndices<'a>>,
    line: usize,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            chars: input.char_indices().peekable(),
            line: 0,
            position: 0,
        }
    }

    pub fn lex(mut self) -> Vec<Token> {
        let mut tokens = Vec::with_capacity(16);
        while let Some(tok) = self.next_token() {
            tokens.push(tok);
        }
        tokens
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let (start, c) = *self.chars.peek()?;
        self.position = start;

        let kind = match c {
            '#' => return Some(self.parse_comment(start)),
            ',' => TokenKind::Comma,
            c if c.is_ascii_digit() || c == '-' => return Some(self.parse_number(start)),
            ';' => TokenKind::Terminator,
            c if c.is_alphabetic() => return Some(self.branch_identifier(start, c)),
            _ => unreachable!(), // skip unknowns
        };
        // Branches that hasnt returned yet are simple tokens, so consume the chars
        self.chars.next();

        Some(Token {
            kind,
            span: (start, start + 1, self.line).into(),
        })
    }

    fn skip_whitespace(&mut self) {
        while let Some((idx, c)) = self.chars.peek() {
            if c.is_whitespace() {
                if *c == '\n' {
                    self.line += 1;
                    self.chars.next();
                } else {
                    self.chars.next();
                }
            } else {
                break;
            }
        }
    }

    fn parse_comment(&mut self, start: usize) -> Token {
        // Safely consume '#'
        self.chars.next();
        let comment: String = self
            .chars
            .by_ref()
            .take_while(|(_, c)| *c != '\n')
            .map(|(_, c)| c)
            .collect();
        let len = comment.chars().count();
        self.line += 1;

        Token {
            kind: TokenKind::Comment(comment),
            span: (start, start + len, self.line).into(),
        }
    }

    /// Parse a register token starting with 'R' followed by digits. E.g., "R1", "R15"
    /// Is is assumed that the 'R' has already been consumed.
    fn parse_register(&mut self, start: usize) -> Token {
        // Safely consume 'R'
        let r = self.chars.next();
        debug_assert!(r.is_some() && r.unwrap().1 == 'R');
        let mut digits: String = String::with_capacity(2);

        while let Some(&(_, c)) = self.chars.peek() {
            if c.is_ascii_digit() {
                digits.push(c);
                self.chars.next();
            } else {
                break;
            }
        }

        let num = digits.parse::<usize>().unwrap();
        if num > REGISTER_LIMIT {
            // TODO: return error token or panic
        }

        Token {
            kind: TokenKind::Register(num),
            span: (start, start + digits.len(), self.line).into(),
        }
    }
    /// Parse a number token, which can be a positive or negative integer.
    /// It is assumed that the first digit or '-' has already been peeked, but not consumed.
    fn parse_number(&mut self, start: usize) -> Token {
        let mut number = String::with_capacity(4);
        while let Some(&(_, c)) = self.chars.peek() {
            if c.is_ascii_digit() || c == '-' {
                number.push(c);
                self.chars.next();
            } else {
                break;
            }
        }

        let value = number.parse::<usize>().unwrap();
        Token {
            kind: TokenKind::Immediate(value),
            span: (start, start + number.len(), self.line).into(),
        }
    }

    /// Decide if the identifier is a register or a label/opcode, and parse accordingly.
    /// It is assumed that the first character has already been peeked, but not consumed.
    fn branch_identifier(&mut self, start: usize, first: char) -> Token {
        let mut lookahead = self.chars.clone();
        lookahead.next(); // consume first char in lookahead

        if first == 'R'
            && let Some((_, c)) = lookahead.next()
            && c.is_ascii_digit()
        {
            return self.parse_register(start);
        }
        self.parse_identifier(start, first)
    }

    /// Parse an identifier which can be an opcode, label definition, or label reference.
    /// It is assumed that the first character has been peeked and consumed.
    fn parse_identifier(&mut self, start: usize, first: char) -> Token {
        // Safely advance
        self.chars.next();

        let mut ident = String::from(first);
        // Read alphanumeric + underscore only
        while let Some(&(_, c)) = self.chars.peek() {
            if c.is_alphanumeric() || c == '_' || c == ':' {
                ident.push(c);
                self.chars.next();
            } else {
                break;
            }
        }

        let len = ident.len();

        let kind = if ident.ends_with(':') {
            TokenKind::LabelDef(ident.trim_end_matches(':').to_string())
        } else if let Ok(opcode) = isa::Opcode::from_str(&ident) {
            TokenKind::Opcode(opcode)
        } else {
            TokenKind::LabelRef(ident)
        };

        Token {
            kind,
            span: (start, start + len, self.line).into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::isa::Opcode;

    fn lex_tokens(input: &str) -> Vec<TokenKind> {
        Lexer::new(input)
            .lex()
            .into_iter()
            .map(|t| t.kind)
            .collect()
    }

    #[test]
    fn test_arithmetic_instructions() {
        let input = "
            ADD R1, R2, R3
            SUB R4, R5, R6
            MULT R7, R8, R9
            ADDI R10, R11, 42
            SUBI R12, R13, 7
            AND R1, R2, R3
            OR R4, R5, R6
            NOT R7, R8
        ";

        let expected = vec![
            TokenKind::Opcode(Opcode::ADD),
            TokenKind::Register(1),
            TokenKind::Comma,
            TokenKind::Register(2),
            TokenKind::Comma,
            TokenKind::Register(3),
            TokenKind::Opcode(Opcode::SUB),
            TokenKind::Register(4),
            TokenKind::Comma,
            TokenKind::Register(5),
            TokenKind::Comma,
            TokenKind::Register(6),
            TokenKind::Opcode(Opcode::MULT),
            TokenKind::Register(7),
            TokenKind::Comma,
            TokenKind::Register(8),
            TokenKind::Comma,
            TokenKind::Register(9),
            TokenKind::Opcode(Opcode::ADDI),
            TokenKind::Register(10),
            TokenKind::Comma,
            TokenKind::Register(11),
            TokenKind::Comma,
            TokenKind::Immediate(42),
            TokenKind::Opcode(Opcode::SUBI),
            TokenKind::Register(12),
            TokenKind::Comma,
            TokenKind::Register(13),
            TokenKind::Comma,
            TokenKind::Immediate(7),
            TokenKind::Opcode(Opcode::AND),
            TokenKind::Register(1),
            TokenKind::Comma,
            TokenKind::Register(2),
            TokenKind::Comma,
            TokenKind::Register(3),
            TokenKind::Opcode(Opcode::OR),
            TokenKind::Register(4),
            TokenKind::Comma,
            TokenKind::Register(5),
            TokenKind::Comma,
            TokenKind::Register(6),
            TokenKind::Opcode(Opcode::NOT),
            TokenKind::Register(7),
            TokenKind::Comma,
            TokenKind::Register(8),
        ];

        assert_eq!(lex_tokens(input), expected);
    }

    #[test]
    fn test_data_transfer() {
        let input = "
            LI R1, 100
            LD R2, R3
            SD R4, R5
        ";

        let expected = vec![
            TokenKind::Opcode(Opcode::LI),
            TokenKind::Register(1),
            TokenKind::Comma,
            TokenKind::Immediate(100),
            TokenKind::Opcode(Opcode::LD),
            TokenKind::Register(2),
            TokenKind::Comma,
            TokenKind::Register(3),
            TokenKind::Opcode(Opcode::SD),
            TokenKind::Register(4),
            TokenKind::Comma,
            TokenKind::Register(5),
        ];

        assert_eq!(lex_tokens(input), expected);
    }

    #[test]
    fn test_control_flow() {
        let input = "
            JR 10
            JEQ 12, R1, R2
            JLTV 15, R3, R4
            JGT 20, R5, R6
            JETV 25, R7, 50
            NOP
            END
        ";

        let expected = vec![
            TokenKind::Opcode(Opcode::JR),
            TokenKind::Immediate(10),
            TokenKind::Opcode(Opcode::JEQ),
            TokenKind::Immediate(12),
            TokenKind::Comma,
            TokenKind::Register(1),
            TokenKind::Comma,
            TokenKind::Register(2),
            TokenKind::Opcode(Opcode::JLTV),
            TokenKind::Immediate(15),
            TokenKind::Comma,
            TokenKind::Register(3),
            TokenKind::Comma,
            TokenKind::Register(4),
            TokenKind::Opcode(Opcode::JGT),
            TokenKind::Immediate(20),
            TokenKind::Comma,
            TokenKind::Register(5),
            TokenKind::Comma,
            TokenKind::Register(6),
            TokenKind::Opcode(Opcode::JETV),
            TokenKind::Immediate(25),
            TokenKind::Comma,
            TokenKind::Register(7),
            TokenKind::Comma,
            TokenKind::Immediate(50),
            TokenKind::Opcode(Opcode::NOP),
            TokenKind::Opcode(Opcode::END),
        ];

        assert_eq!(lex_tokens(input), expected);
    }

    #[test]
    fn test_labels_and_comments() {
        let input = "
            start:
                ADD R1, R2, R3  # addition
            loop_start:
                SUB R4, R5, R6
            end:
                END
        ";

        let expected = vec![
            TokenKind::LabelDef("start".to_string()),
            TokenKind::Opcode(Opcode::ADD),
            TokenKind::Register(1),
            TokenKind::Comma,
            TokenKind::Register(2),
            TokenKind::Comma,
            TokenKind::Register(3),
            TokenKind::Comment(" addition".to_string()),
            TokenKind::LabelDef("loop_start".to_string()),
            TokenKind::Opcode(Opcode::SUB),
            TokenKind::Register(4),
            TokenKind::Comma,
            TokenKind::Register(5),
            TokenKind::Comma,
            TokenKind::Register(6),
            TokenKind::LabelDef("end".to_string()),
            TokenKind::Opcode(Opcode::END),
        ];

        assert_eq!(lex_tokens(input), expected);
    }

    #[test]
    fn test_edge_cases() {
        //TODO: implement lexer errors
        let input = "
            ADDI R1, R2, 15
            ADD R16, R1, R2
            loop_1: NOP
        ";

        let expected = vec![
            TokenKind::Opcode(Opcode::ADDI),
            TokenKind::Register(1),
            TokenKind::Comma,
            TokenKind::Register(2),
            TokenKind::Comma,
            TokenKind::Immediate(15),
            TokenKind::Opcode(Opcode::ADD),
            TokenKind::Register(16),
            TokenKind::Comma,
            TokenKind::Register(1),
            TokenKind::Comma,
            TokenKind::Register(2),
            TokenKind::LabelDef("loop_1".to_string()),
            TokenKind::Opcode(Opcode::NOP),
        ];

        assert_eq!(lex_tokens(input), expected);
    }
}
