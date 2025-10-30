// Virtual Machine for the custom ISA
// Use the shared isa-core library for instruction decoding

mod errors;
mod executor;
mod memory;
mod vm;

use executor::{BaseInterpreter, Executor};
use isa_core::traits::Decodable;
use isa_core::types::ResolvedInstruction;
use memory::DynamicMemory;
use std::fs::File;
use std::io::{BufRead, BufReader};
use vm::{VM, VMState};

use crate::memory::Memory;

fn main() {
    println!("=== ISA Virtual Machine ===\n");

    // Read binary file
    let filename = "/Users/johannessigvardsen/Files/Projects/Fun/rust-projects/isa-utils/examples/first_n_primes.txt";
    println!("Reading program from {}...", filename);

    let file = File::open(filename).expect("Failed to open program file");
    let reader = BufReader::new(file);

    let mut encoded: Vec<u32> = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Parse as binary (with or without 0b prefix)
        let line = line.strip_prefix("0b").unwrap_or(line);
        let word = u32::from_str_radix(line, 2).expect("Failed to parse binary");
        encoded.push(word);
    }

    println!("Loaded {} instructions\n", encoded.len());

    // Decode program
    println!("Decoding program...");
    let program: Vec<ResolvedInstruction> = encoded
        .iter()
        .map(|word| word.decode().expect("Failed to decode instruction"))
        .collect();

    println!("Decoded {} instructions\n", program.len());

    // Create memory and VM state
    let memory = DynamicMemory::new(900);
    let mem_size = memory.size();

    let state = VMState {
        registers: [0; 32],
        memory,
        mem_size,
        program,
        pc: 0,
        halted: false,
        cycles: 0,
    };

    // Create VM with base interpreter
    let mut vm = VM {
        executor: BaseInterpreter,
        state,
    };

    println!("Running program...\n");

    // Run until halted or error
    loop {
        match vm.executor.execute(&mut vm.state) {
            Ok(halted) => {
                if halted {
                    println!("Program halted after {} cycles\n", vm.state.cycles);
                    break;
                }
            }
            Err(e) => {
                eprintln!("VM Error: {}", e);
                eprintln!("PC: {}, Cycles: {}", vm.state.pc, vm.state.cycles);
                break;
            }
        }
    }

    // Print register state
    println!("=== Register State ===");
    for i in 0..10 {
        println!("R{}: {}", i, vm.state.registers[i]);
    }

    // Print first 10 memory addresses
    println!("\n=== Memory State (first 100 addresses) ===");
    for i in 0..800 {
        println!("M[{}]: {}", i, vm.state.memory.read(i));
    }

    println!("\n=== Execution Complete ===");
}
