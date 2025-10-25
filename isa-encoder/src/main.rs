mod encoder;
mod errors;
mod isa;
mod lexer;
mod parser;
use clap::*;
use clap::{Parser as ClapParser, Subcommand};
use isa_core::types::ResolvedInstruction;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(ClapParser)]
#[command(name = "isa-encoder")]
#[command(about = "An assembler for the custom ISA", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Assemble a source file into binary machine code
    Assemble {
        /// Input assembly file
        #[arg(short, long)]
        input: PathBuf,

        /// Output file for the binary (default: input with .bin extension)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Output format: binary, hex, or txt (default).
        #[arg(short, long, default_value = "txt")]
        format: OutputFormat,
    },
    /// Check assembly file for errors without generating output
    Check {
        /// Input assembly file
        input: PathBuf,
    },
}

#[derive(Clone, Copy)]
enum OutputFormat {
    Binary,
    Hex,
    Txt,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "binary" | "bin" => Ok(OutputFormat::Binary),
            "hex" => Ok(OutputFormat::Hex),
            "txt" => Ok(OutputFormat::Txt),
            _ => Err(format!("Invalid format: {}", s)),
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Assemble {
            input,
            output,
            format,
        } => {
            if let Err(e) = assemble_file(&input, output.as_ref(), format) {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
        Commands::Check { input } => {
            if let Err(e) = check_file(&input) {
                eprintln!("{}", e);
                std::process::exit(1);
            }
            println!("✓ Assembly file is valid!");
        }
    }
}

fn assemble_file(
    input: &PathBuf,
    output: Option<&PathBuf>,
    format: OutputFormat,
) -> Result<(), String> {
    let source =
        fs::read_to_string(input).map_err(|e| format!("Failed to read input file: {}", e))?;

    let instructions = parse_source(&source)?;

    let encoded =
        encoder::encode_program(&instructions).map_err(|e| format!("Encoding error: {}", e))?;

    let output_path = output.cloned().unwrap_or_else(|| {
        let mut path = input.clone();
        path.set_extension("bin");
        path
    });

    match format {
        OutputFormat::Binary => {
            write_binary(&output_path, &encoded)?;
            println!("✓ Binary output written to {}", output_path.display());
        }
        OutputFormat::Hex => {
            write_hex(&output_path, &encoded)?;
            println!("✓ Hex output written to {}", output_path.display());
        }
        OutputFormat::Txt => {
            let mut bin_path = output_path.clone();
            bin_path.set_extension("txt");
            write_txt_binary(&bin_path, &encoded)?;
            println!("✓ Binary txt output written to {}", bin_path.display());
        }
    }

    Ok(())
}

fn check_file(input: &PathBuf) -> Result<(), String> {
    let source =
        fs::read_to_string(input).map_err(|e| format!("Failed to read input file: {}", e))?;

    let _instructions = parse_source(&source)?;

    Ok(())
}

fn parse_source(source: &str) -> Result<Vec<ResolvedInstruction>, String> {
    let lexer = lexer::Lexer::new(source);
    let tokens = lexer.lex();

    let mut parser = parser::Parser::new(tokens);
    parser
        .parse_instructions()
        .map_err(|e| e.display_with_source(source))
}

fn write_binary(path: &PathBuf, encoded: &[u32]) -> Result<(), String> {
    let mut file =
        fs::File::create(path).map_err(|e| format!("Failed to create output file: {}", e))?;

    for word in encoded {
        file.write_all(&word.to_be_bytes())
            .map_err(|e| format!("Failed to write binary data: {}", e))?;
    }

    Ok(())
}

fn write_txt_binary(path: &PathBuf, encoded: &[u32]) -> Result<(), String> {
    let mut output = String::new();

    for word in encoded.iter() {
        // debug print resultant lines in 32 bit format
        output.push_str(&format!("{:032b}\n", word));
    }

    fs::write(path, output).map_err(|e| format!("Failed to write text binary file: {}", e))?;

    Ok(())
}

fn write_hex(path: &PathBuf, encoded: &[u32]) -> Result<(), String> {
    let mut output = String::new();

    for word in encoded.iter() {
        output.push_str(&format!("{:08x}\n", word));
    }

    fs::write(path, output).map_err(|e| format!("Failed to write hex file: {}", e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_program() {
        let input = "
            ADD R1, R2, R3
            LI R4, 100
            END
        ";

        let result = parse_source(input);
        assert!(result.is_ok());
        let instructions = result.unwrap();
        assert_eq!(instructions.len(), 3);
    }

    #[test]
    fn test_error_reporting() {
        let input = "
            ADD R1, R2
        ";

        let result = parse_source(input);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Operand count mismatch"));
    }
}
