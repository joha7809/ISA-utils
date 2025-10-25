// Core ISA definitions shared between assembler and VM
// This crate contains only the essential types and constants needed by both

use std::str::FromStr;

pub mod bits;
pub mod codec;
pub mod consts;
pub mod layout;
pub mod traits;
pub mod types;

mod tests;
