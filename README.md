# ISA Utils - Custom Assembly Language Toolchain

A Cargo workspace containing a complete toolchain for a custom instruction set architecture (ISA), including core types, assembler, and virtual machine.

## Project Structure

This project is organized as a Cargo workspace with three crates:

- **isa-core/** - Core library with ISA types, instruction encoding/decoding, and format specifications
- **isa-encoder/** - Assembler that parses `.asm` files and generates binary machine code
- **isa-vm/** - Virtual machine for executing assembled programs _(unimplemented)_

The shared `isa-core` library provides a single source of truth for opcode definitions, instruction formats, and encoding/decoding logic that is used by both the assembler and VM.

## Features

- ✅ Full ISA support (16 instructions: arithmetic, logical, memory, and control flow)
- ✅ 32-bit instruction encoding with multiple formats (R2, R3, RI, RRI, I, NoOP)
- ✅ Label support for jump instructions
- ✅ Signed and unsigned immediate values with proper two's complement encoding
- ✅ Comprehensive error handling with detailed source context
- ✅ Multiple output formats (text binary, hex, raw binary)
- ✅ Syntax validation without assembly
- ✅ Fully tested encoder and decoder with roundtrip verification

## Installation

```bash
# Build all crates in the workspace
cargo build --release

# Build individual crates
cargo build -p isa-core --release
cargo build -p isa-encoder --release
cargo build -p isa-vm --release
```

## Usage

### Assembler (isa-encoder)

The assembler converts `.asm` assembly source files into binary machine code.

```bash
# Generate text binary output (default) - human-readable 0s and 1s
cargo run -p isa-encoder -- assemble -i examples/example.asm -o output.txt

# Generate hex output - hexadecimal representation
cargo run -p isa-encoder -- assemble -i examples/example.asm -o output.hex --format hex

# Generate raw binary output - actual binary file for VM execution
cargo run -p isa-encoder -- assemble -i examples/example.asm -o output.bin --format binary
```

### Syntax Validation

Check assembly syntax without generating output:

```bash
cargo run -p isa-encoder -- check examples/example.asm
```

### Virtual Machine (isa-vm)

⚠️ **The VM is currently unimplemented.** The basic structure exists but execution logic is not yet implemented.

When completed, it will execute assembled binary programs:

```bash
cargo run -p isa-vm -- <binary_file>
```

## ISA Specification

This toolchain implements a custom 32-bit RISC-style instruction set. For complete details, see [`isa.md`](isa.md).

### Instruction Formats

All instructions are 32 bits wide with the following encoding formats:

| Format   | Bit Layout                                          | Description               |
| -------- | --------------------------------------------------- | ------------------------- |
| **R3**   | `OPCODE(4) · REG(5) · REG(5) · REG(5) · unused(13)` | Three-register operations |
| **R2**   | `OPCODE(4) · REG(5) · REG(5) · unused(18)`          | Two-register operations   |
| **RRI**  | `OPCODE(4) · REG(5) · REG(5) · IMM(18)`             | Two registers + immediate |
| **RI**   | `OPCODE(4) · REG(5) · unused(5) · IMM(18)`          | Register + immediate      |
| **I**    | `OPCODE(4) · IMM(28)`                               | Immediate only (jumps)    |
| **NoOP** | `OPCODE(4) · unused(28)`                            | No operands               |

### Resulting Formats

When implementing the CPU in chisel, the above formats are treated as:

| Format | Bit Layout                                          | Description               |
| ------ | --------------------------------------------------- | ------------------------- |
| **R**  | `OPCODE(4) · REG(5) · REG(5) · REG(5) · unused(13)` | Three-register operations |
| **I**  | `OPCODE(4) · REG(5) · REG(5) · IMM(18)`             | Two registers + immediate |
| **J**  | `OPCODE(4) · IMM(28)`                               | Immediate only (jumps)    |

### Encoding Details

- **Opcodes**: 4 bits `[31:28]` - 16 possible instructions
- **Registers**: 5 bits each - 32 registers (R0-R31)
- **Immediates**: Signed (two's complement) except for jump addresses which are unsigned
- **Byte order**: Big-endian bit ordering

## Supported Instructions

### Arithmetic Operations

| Instruction | Syntax              | Description              | Format |
| ----------- | ------------------- | ------------------------ | ------ |
| `ADD`       | `ADD Rd, Rs1, Rs2`  | `Rd = Rs1 + Rs2`         | R3     |
| `SUB`       | `SUB Rd, Rs1, Rs2`  | `Rd = Rs1 - Rs2`         | R3     |
| `MULT`      | `MULT Rd, Rs1, Rs2` | `Rd = Rs1 * Rs2`         | R3     |
| `ADDI`      | `ADDI Rd, Rs, imm`  | `Rd = Rs + imm` (signed) | RRI    |
| `SUBI`      | `SUBI Rd, Rs, imm`  | `Rd = Rs - imm` (signed) | RRI    |

### Logical Operations

| Instruction | Syntax             | Description       | Format |
| ----------- | ------------------ | ----------------- | ------ |
| `AND`       | `AND Rd, Rs1, Rs2` | `Rd = Rs1 & Rs2`  | R3     |
| `OR`        | `OR Rd, Rs1, Rs2`  | `Rd = Rs1 \| Rs2` | R3     |
| `NOT`       | `NOT Rd, Rs`       | `Rd = ~Rs`        | R2     |

### Data Transfer

| Instruction | Syntax       | Description                 | Format |
| ----------- | ------------ | --------------------------- | ------ |
| `LI`        | `LI Rd, imm` | `Rd = imm` (load immediate) | RI     |
| `LD`        | `LD Rd, Rs`  | `Rd = memory[Rs]`           | R2     |
| `SD`        | `SD Rs, Rd`  | `memory[Rd] = Rs`           | R2     |

### Control Flow

| Instruction | Syntax               | Description          | Format |
| ----------- | -------------------- | -------------------- | ------ |
| `JR`        | `JR addr`            | Jump to address      | I      |
| `JEQ`       | `JEQ Rs1, Rs2, addr` | Jump if `Rs1 == Rs2` | RRI    |
| `JLT`       | `JLT Rs1, Rs2, addr` | Jump if `Rs1 < Rs2`  | RRI    |
| `NOP`       | `NOP`                | No operation         | NoOP   |
| `END`       | `END`                | Halt execution       | NoOP   |

### Labels

Labels are defined with a colon and can be used as jump targets:

```assembly
start:
    LI R1, 0
    LI R2, 10

loop:
    ADDI R1, R1, 1
    JLT R1, R2, loop

end:
    END
```

During assembly, labels are resolved to instruction addresses.

## Error Handling

The assembler provides detailed error messages with source code context:

```
Parse Error: Operand count mismatch, expected 3, found 2
  |
3 | ADD R3, R1
  | ^^^^^^^^^^ wrong number of operands
```

Error types include:

- **Syntax errors**: Invalid instruction format, operand count mismatches
- **Type errors**: Register used where immediate expected, etc.
- **Range errors**: Immediate values out of range for bit width
- **Reference errors**: Undefined labels, invalid register numbers

## Development

### Testing

The project has comprehensive test coverage for encoding, decoding, and parsing:

```bash
# Run all tests in the workspace
cargo test --workspace

# Test individual crates
cargo test -p isa-core      # Core types and codec tests
cargo test -p isa-encoder   # Parser and assembler tests
cargo test -p isa-vm        # VM tests (when implemented)

# Run with output
cargo test -- --nocapture
```

Test coverage includes:

- Instruction encoding/decoding roundtrip tests
- All instruction format verification
- Signed and unsigned immediate handling
- Parser error cases and edge conditions
- Label resolution

### Project Structure Details

```
isa-utils/
├── isa-core/           # Core library
│   ├── src/
│   │   ├── bits.rs           # Bit manipulation utilities
│   │   ├── codec.rs          # Encoding/decoding implementation
│   │   ├── consts.rs         # Constants (register limits, etc.)
│   │   ├── format_spec.rs    # Instruction format specifications
│   │   ├── traits.rs         # Encodable/Decodable traits
│   │   └── types.rs          # Core types (Opcode, Operand, etc.)
│   └── Cargo.toml
├── isa-encoder/        # Assembler
│   ├── src/
│   │   ├── main.rs           # CLI entry point
│   │   ├── parser.rs         # Assembly parser
│   │   └── errors.rs         # Error types
│   └── Cargo.toml
├── isa-vm/             # Virtual machine (unimplemented)
│   ├── src/
│   │   ├── main.rs           # VM entry point
│   │   ├── vm.rs             # VM state and execution
│   │   ├── memory.rs         # Memory implementation
│   │   └── executor.rs       # Instruction executor
│   └── Cargo.toml
├── examples/           # Example assembly programs
│   ├── example.asm
│   ├── erosion.asm
│   └── sieve.asm
├── isa.md             # Complete ISA specification
└── README.md
```

## Example Programs

The `examples/` directory contains sample assembly programs:

- **example.asm** - Basic syntax examples
- **erosion.asm** - Image erosion algorithm implementation
- **sieve.asm** - Sieve of Eratosthenes (prime number generation)

Assemble an example:

```bash
cargo run -p isa-encoder -- assemble -i examples/erosion.asm -o erosion.txt
cargo run -p isa-encoder -- assemble -i examples/sieve.asm -o sieve.bin --format binary
```

## Virtual Machine Implementation Status

The `isa-vm` crate contains the basic structure for a virtual machine but is **not yet implemented**.

### Planned VM Features

- Instruction fetch and decode using `isa-core`
- Register file (32 registers)
- Memory system (configurable size)
- Instruction execution loop
- Program counter management
- Jump/branch handling
- Debug mode with instruction tracing
