/// Single source of truth for instruction format specifications
/// This module defines the bit layout for all instruction formats in one place
use crate::types::InstrFormat;

// =========================================================================================
// SINGLE SOURCE OF TRUTH: All bit ranges defined here
// =========================================================================================

// Opcode always occupies bits [31:28]
pub const OPCODE_RANGE: BitRange = BitRange::new(31, 28);

// Register bit ranges (5 bits each)
const R1_RANGE: BitRange = BitRange::new(27, 23);
const R2_RANGE: BitRange = BitRange::new(22, 18);
const R3_RANGE: BitRange = BitRange::new(17, 13);

// Immediate bit ranges for different formats
const IMM_28_RANGE: BitRange = BitRange::new(27, 0); // I format: 28 bits
const IMM_18_RANGE: BitRange = BitRange::new(17, 0); // RRI format: 18 bits

// =========================================================================================
// Format specifications as static slices.
// These specify what each machine instrcution type expect of arguments and their bit ranges
// The ISA specifies 3 formats, however for encoding/decoding we will expand upon these.
// Each of the following spec are still a part of the ISA's 3 formats: R, I and J.
// =========================================================================================

const R2_SPEC: &[FieldSpec] = &[
    FieldSpec::Opcode(OPCODE_RANGE),
    FieldSpec::Register(R1_RANGE),
    FieldSpec::Register(R2_RANGE),
];

const R3_SPEC: &[FieldSpec] = &[
    FieldSpec::Opcode(OPCODE_RANGE),
    FieldSpec::Register(R1_RANGE),
    FieldSpec::Register(R2_RANGE),
    FieldSpec::Register(R3_RANGE),
];

const RI_SPEC: &[FieldSpec] = &[
    FieldSpec::Opcode(OPCODE_RANGE),
    FieldSpec::Register(R1_RANGE),
    FieldSpec::Immediate(IMM_18_RANGE),
];

const RRI_SPEC: &[FieldSpec] = &[
    FieldSpec::Opcode(OPCODE_RANGE),
    FieldSpec::Register(R1_RANGE),
    FieldSpec::Register(R2_RANGE),
    FieldSpec::Immediate(IMM_18_RANGE),
];

const I_SPEC: &[FieldSpec] = &[
    FieldSpec::Opcode(OPCODE_RANGE),
    FieldSpec::Immediate(IMM_28_RANGE),
];

const NOOP_SPEC: &[FieldSpec] = &[
    FieldSpec::Opcode(OPCODE_RANGE),
    // FieldSpec::Immediate(IMM_27_RANGE), // Padding bits, set to 0
];

/// Complete specification for an instruction format
pub struct FormatSpec {
    pub format: InstrFormat,
    pub fields: &'static [FieldSpec],
}

/// Specifies the bit range for a field in an instruction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitRange {
    pub hi: u8,
    pub lo: u8,
}

/// Defines which field occupies which bit range
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldSpec {
    Opcode(BitRange),
    Register(BitRange),
    Immediate(BitRange),
}

impl BitRange {
    pub const fn new(hi: u8, lo: u8) -> Self {
        Self { hi, lo }
    }

    pub const fn width(&self) -> u8 {
        self.hi - self.lo + 1
    }
}

impl FieldSpec {
    pub fn bit_range(&self) -> BitRange {
        match self {
            FieldSpec::Opcode(r) | FieldSpec::Register(r) | FieldSpec::Immediate(r) => *r,
        }
    }
}

/// Get the format specification for a given instruction format
pub const fn get_format_spec(format: InstrFormat) -> FormatSpec {
    match format {
        InstrFormat::R2 => FormatSpec {
            format,
            fields: R2_SPEC,
        },
        InstrFormat::R3 => FormatSpec {
            format,
            fields: R3_SPEC,
        },
        InstrFormat::RI => FormatSpec {
            format,
            fields: RI_SPEC,
        },
        InstrFormat::RRI => FormatSpec {
            format,
            fields: RRI_SPEC,
        },
        InstrFormat::I => FormatSpec {
            format,
            fields: I_SPEC,
        },
        InstrFormat::NoOP => FormatSpec {
            format,
            fields: NOOP_SPEC,
        },
    }
}
