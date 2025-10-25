use std::collections::HashMap;

use isa_core::{consts::REGISTER_LIMIT, traits::ToResolvedInstr, types::ResolvedInstruction};

use crate::{
    errors::ParseError,
    isa::{InstrFormatValidator, Spanned, UnresolvedInstruction, UnresolvedOperand},
    lexer::{Token, TokenKind},
};

pub struct Parser {
    tokens: std::iter::Peekable<std::vec::IntoIter<Token>>,
}
/// Returns a vec of instructions.
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens.into_iter().peekable(),
        }
    }

    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    pub fn parse_instructions(&mut self) -> Result<Vec<ResolvedInstruction>, ParseError> {
        // First pass: collect raw instructions with label positions
        let mut instructions = Vec::with_capacity(20);
        let mut labels = HashMap::with_capacity(2);

        // if self.peek().is_none() {
        //     return Err(ParseError::UnexpectedEndOfInput);
        // }

        while let Some(token) = self.next() {
            match token.kind {
                TokenKind::LabelDef(name) => {
                    if labels.insert(name.clone(), instructions.len()).is_some() {
                        return Err(ParseError::DuplicateLabel {
                            label: name,
                            span: token.span,
                        });
                    }
                }
                TokenKind::Opcode(opcode) => {
                    let operands = self.parse_operands()?;
                    instructions.push(UnresolvedInstruction {
                        opcode: Spanned::new(opcode, token.span),
                        operands,
                    });
                }
                TokenKind::Comment(_) | TokenKind::Terminator => continue,
                _ => {
                    // I think this is unreachable, but just in case
                    return Err(ParseError::UnexpectedToken {
                        expected: "label definition or opcode".to_string(),
                        found: format!("{:?}", token.kind),
                        span: token.span,
                    });
                }
            }
        }

        // Second pass: resolve label references
        resolve_labels(&mut instructions, &labels)?;

        // Third pass: validate
        let resolved = validate_instructions(&instructions)?;

        Ok(resolved)
    }

    /// Parse operands for a specific opcode.
    fn parse_operands(&mut self) -> Result<Vec<Spanned<UnresolvedOperand>>, ParseError> {
        let mut operands = Vec::new();

        while let Some(token) = self.peek() {
            match &token.kind {
                TokenKind::Register(r) => {
                    if *r >= REGISTER_LIMIT {
                        return Err(ParseError::InvalidRegister { span: token.span });
                    }

                    operands.push(Spanned::new(
                        UnresolvedOperand::Register(*r as u8),
                        token.span,
                    ));

                    self.next();
                }
                TokenKind::Immediate(i) => {
                    operands.push(Spanned::new(UnresolvedOperand::Immediate(*i), token.span));
                    self.next();
                }
                TokenKind::LabelRef(label) => {
                    // Store as placeholder, resolve later
                    operands.push(Spanned::new(
                        UnresolvedOperand::LabelRef(label.clone()),
                        token.span,
                    ));
                    self.next();
                }
                TokenKind::Comma => {
                    self.next(); // skip commas
                }
                // Stop conditions
                TokenKind::Opcode(_) | TokenKind::LabelDef(_) => break,
                _ => {
                    self.next();
                } // skip comments, terminators
            }
        }

        Ok(operands)
    }
}

fn validate_instructions(
    instructions: &[UnresolvedInstruction],
) -> Result<Vec<ResolvedInstruction>, ParseError> {
    let mut res = Vec::with_capacity(20);
    for instr in instructions {
        instr
            .opcode
            .as_ref()
            .instruction_format()
            .validate(&instr.operands, &instr.get_total_span())?;

        let res_instr = instr.to_resolved().ok_or(ParseError::InvalidInstruction)?;
        res.push(res_instr);
    }
    Ok(res)
}

fn resolve_labels(
    instructions: &mut [UnresolvedInstruction],
    labels: &HashMap<String, usize>,
) -> Result<(), ParseError> {
    // For each opcode that is a label reference, replace with immediate row_index
    for instr in instructions.iter_mut() {
        for op in instr.operands.iter_mut() {
            if let UnresolvedOperand::LabelRef(label) = op.as_ref() {
                if let Some(&row_index) = labels.get(label) {
                    *op.as_mut() = UnresolvedOperand::Immediate(row_index);
                } else {
                    return Err(ParseError::UndefinedLabel {
                        label: label.clone(),
                        span: op.span,
                    });
                }
            }
        }
    }
    Ok(())
}
