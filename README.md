# ISA Utils - Custom Assembly Language Toolchain

A Cargo workspace containing a complete toolchain for a custom instruction set architecture (ISA), including assembler and virtual machine.

## Project Structure

This project is organized as a Cargo workspace with three crates:

- **isa-core/** - Shared library with `Opcode`, `InstrFormat`, `DecodedInstruction`, and encode/decode functions
- **isa-encoder/** - Assembler that converts `.asm` files to binary machine code  
- **isa-vm/** - Virtual machine for executing assembled programs

The shared `isa-core` library means both the assembler and VM can reuse all the opcode definitions and encoding/decoding logic.

## Features

- ✅ Full ISA support (arithmetic, logical, memory, and control flow instructions)
- ✅ Label support for jumps and branches
- ✅ Pretty error messages with source code context
- ✅ Multiple output formats (binary, hex, txt)
- ✅ Syntax checking without assembly
- ✅ **Cargo workspace structure with three crates**

## Installation

```bash
cargo build --release
```

## Usage

### Assemble a file

```bash
# Generate text binary output (default)
cargo run -p isa-encoder -- assemble -i example.asm -o output.bin

# Generate hex output
cargo run -p isa-encoder -- assemble -i example.asm -o output.hex --format hex

# Generate raw binary output
cargo run -p isa-encoder -- assemble -i example.asm -o output.bin --format binary
```

### Check syntax without assembling

```bash
cargo run -p isa-encoder -- check example.asm
```

### VM Usage

```bash
cargo run -p isa-vm <binary_file>
```

## For VM Implementation

Your VM can use `isa-core` to decode instructions:

```rust
use isa_core::{DecodedInstruction, Opcode, Operand};

// Decode a 32-bit instruction word
let instr = DecodedInstruction::decode(word)?;

// Execute based on opcode
match instr.opcode {
    Opcode::ADD => {
        if let [Operand::Register(rd), Operand::Register(rs1), Operand::Register(rs2)] = 
            instr.operands[..] {
            registers[rd] = registers[rs1] + registers[rs2];
        }
    }
    // ... other opcodes
}
```

See `isa-vm/src/main.rs` for a starter template.

## Supported Instructions

### Arithmetic

- `ADD Rd, Rs1, Rs2` - Add two registers
- `SUB Rd, Rs1, Rs2` - Subtract registers
- `MULT Rd, Rs1, Rs2` - Multiply registers
- `ADDI Rd, Rs, imm` - Add immediate
- `SUBI Rd, Rs, imm` - Subtract immediate

### Logical

- `AND Rd, Rs1, Rs2` - Bitwise AND
- `OR Rd, Rs1, Rs2` - Bitwise OR
- `NOT Rd, Rs` - Bitwise NOT

### Memory

- `LI Rd, imm` - Load immediate
- `LD Rd, Rs` - Load from memory
- `SD Rs, Rd` - Store to memory

### Control Flow

- `JR imm` - Jump to address
- `JEQ Rs1, Rs2, imm` - Jump if equal
- `JLTV Rd, imm1, imm2` - Jump if less than value
- `JGT Rs1, Rs2, imm` - Jump if greater than
- `JETV Rd, imm1, imm2` - Jump if equal to value
- `NOP` - No operation
- `END` - End program

### Labels

Labels can be defined with a colon and referenced in jump instructions:

```assembly
loop_start:
    ADDI R1, R1, 1
    JGT R1, R2, loop_start
```

## Error Reporting

The assembler provides detailed error messages with source context:

```
Parse Error: Operand count mismatch, expected 3, found 2
  |
3 | ADD R3, R1
  | ^^^ wrong number of operands
```

## Testing

```bash
# Test everything
cargo test --workspace

# Test individual crates
cargo test -p isa-core
cargo test -p isa-encoder
cargo test -p isa-vm
```

## Examples

- `erosion.asm` - Erosion algorithm implementation
- `sieve.asm` - Sieve of Eratosthenes

```bash
cargo run -p isa-encoder -- assemble -i erosion.asm
cargo run -p isa-encoder -- assemble -i sieve.asm
```
