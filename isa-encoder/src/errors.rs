use crate::lexer::Span;

// Make some errors for our parser
#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken {
        expected: String,
        found: String,
        span: Span,
    },
    UnexpectedEndOfInput,
    DuplicateLabel {
        label: String,
        span: Span,
    },
    UndefinedLabel {
        label: String,
        span: Span,
    },
    OperandCountMismatch {
        expected: usize,
        found: usize,
        span: Span,
    },
    OperandTypeMismatch {
        span: Span,
    },
    InvalidRegister {
        span: Span,
    },
    InvalidInstruction,
}

impl ParseError {
    /// Display a pretty error message with source context
    pub fn display_with_source(&self, source: &str) -> String {
        match self {
            ParseError::UnexpectedToken {
                expected,
                found,
                span,
            } => {
                format!(
                    "Parse Error: Expected {}, found {}\n{}",
                    expected,
                    found,
                    format_error_location(source, span, "unexpected token here")
                )
            }
            ParseError::UnexpectedEndOfInput => "Parse Error: Unexpected end of input".to_string(),
            ParseError::DuplicateLabel { label, span } => {
                format!(
                    "Parse Error: Duplicate label '{}'\n{}",
                    label,
                    format_error_location(source, span, "label already defined")
                )
            }
            ParseError::UndefinedLabel { label, span } => {
                format!(
                    "Parse Error: Undefined label '{}'\n{}",
                    label,
                    format_error_location(source, span, "label not found")
                )
            }
            ParseError::OperandCountMismatch {
                expected,
                found,
                span,
            } => {
                format!(
                    "Parse Error: Operand count mismatch, expected {}, found {}\n{}",
                    expected,
                    found,
                    format_error_location(source, span, "wrong number of operands")
                )
            }
            ParseError::OperandTypeMismatch { span } => {
                format!(
                    "Parse Error: Operand type mismatch\n{}",
                    format_error_location(source, span, "incorrect operand type")
                )
            }
            ParseError::InvalidRegister { span } => {
                format!(
                    "Parse Error: Invalid register\n{}",
                    format_error_location(source, span, "register out of range")
                )
            }
            ParseError::InvalidInstruction => {
                "Something went wrong when resolving an Insruction. This should not happen!"
                    .to_string()
            }
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedToken {
                expected,
                found,
                span,
            } => {
                write!(
                    f,
                    "Parse Error at line {}: Expected {}, found {}",
                    span.line + 1,
                    expected,
                    found
                )
            }
            ParseError::UnexpectedEndOfInput => {
                write!(f, "Parse Error: Unexpected end of input")
            }
            ParseError::DuplicateLabel { label, span } => {
                write!(
                    f,
                    "Parse Error at line {}: Duplicate label '{}'",
                    span.line + 1,
                    label
                )
            }
            ParseError::UndefinedLabel { label, span } => {
                write!(
                    f,
                    "Parse Error at line {}: Undefined label '{}'",
                    span.line + 1,
                    label
                )
            }
            ParseError::OperandCountMismatch {
                expected,
                found,
                span,
            } => {
                write!(
                    f,
                    "Parse Error at line {}: Operand count mismatch, expected {}, found {}",
                    span.line + 1,
                    expected,
                    found
                )
            }
            ParseError::OperandTypeMismatch { span } => {
                write!(
                    f,
                    "Parse Error at line {}: Operand type mismatch",
                    span.line + 1
                )
            }
            ParseError::InvalidRegister { span } => {
                write!(f, "Parse Error at line {}: Invalid register", span.line + 1)
            }

            ParseError::InvalidInstruction => {
                write!(
                    f,
                    "Error happened at instruction resolving. This should never happen, contact the author!"
                )
            }
        }
    }
}

/// Format an error with source code context and highlighting

fn format_error_location(source: &str, span: &Span, message: &str) -> String {
    let lines: Vec<&str> = source.lines().collect();

    if span.line >= lines.len() {
        return format!("  at line {}", span.line + 1);
    }

    let line = lines[span.line];
    let line_num = span.line + 1;
    let line_num_width = line_num.to_string().len();

    // Compute the start index of this line in the full source
    let line_start_index: usize = source
        .lines()
        .take(span.line)
        .map(|l| l.len() + 1) // +1 for the '\n'
        .sum();

    // Column is the offset from the start of the line
    let col_in_line_start = span.start.saturating_sub(line_start_index);
    let col_in_line_end = span.end.saturating_sub(line_start_index);

    let mut result = String::new();
    result.push_str(&format!("{:width$} |\n", "", width = line_num_width));
    result.push_str(&format!("{} | {}\n", line_num, line));
    result.push_str(&format!("{:width$} | ", "", width = line_num_width));
    result.push_str(&" ".repeat(col_in_line_start));
    result.push_str(&"^".repeat((col_in_line_end - col_in_line_start).max(1)));
    result.push_str(&format!(" {}", message));

    result
}
