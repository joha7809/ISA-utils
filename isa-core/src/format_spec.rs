/// Single source of truth for instruction format specifications
/// This module defines the bit layout for all instruction formats in one place
use crate::types::InstrFormat;

// ============================================================================
// SINGLE SOURCE OF TRUTH: All bit ranges defined here
// ============================================================================

// Opcode always occupies bits [31:27]
pub const OPCODE_RANGE: BitRange = BitRange::new(31, 27);

// Register bit ranges (5 bits each)
const R1_RANGE: BitRange = BitRange::new(26, 22);
const R2_RANGE: BitRange = BitRange::new(21, 17);
const R3_RANGE: BitRange = BitRange::new(16, 12);

// Immediate bit ranges for different formats
const IMM_27_RANGE: BitRange = BitRange::new(26, 0); // I format: 27 bits
const IMM_22_RANGE: BitRange = BitRange::new(21, 0); // RI format: 22 bits
const IMM_17_RANGE: BitRange = BitRange::new(16, 0); // RRI format: 17 bits
const IMM_11_HIGH_RANGE: BitRange = BitRange::new(21, 11); // RII format: first 11 bits
const IMM_11_LOW_RANGE: BitRange = BitRange::new(10, 0); // RII format: second 11 bits

// ============================================================================
// Format specifications as static slices
// ============================================================================

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
    FieldSpec::Immediate(IMM_22_RANGE),
];

const RRI_SPEC: &[FieldSpec] = &[
    FieldSpec::Opcode(OPCODE_RANGE),
    FieldSpec::Register(R1_RANGE),
    FieldSpec::Register(R2_RANGE),
    FieldSpec::Immediate(IMM_17_RANGE),
];

const RII_SPEC: &[FieldSpec] = &[
    FieldSpec::Opcode(OPCODE_RANGE),
    FieldSpec::Register(R1_RANGE),
    FieldSpec::Immediate(IMM_11_HIGH_RANGE),
    FieldSpec::Immediate(IMM_11_LOW_RANGE),
];

const I_SPEC: &[FieldSpec] = &[
    FieldSpec::Opcode(OPCODE_RANGE),
    FieldSpec::Immediate(IMM_27_RANGE),
];

const NOOP_SPEC: &[FieldSpec] = &[
    FieldSpec::Opcode(OPCODE_RANGE),
    FieldSpec::Immediate(IMM_27_RANGE), // Padding bits, set to 0
];

/// Complete specification for an instruction format
pub struct FormatSpec {
    pub format: InstrFormat,
    pub fields: &'static [FieldSpec],
}

// pub struct BitField {
//     pub kind: FieldKind,
//     pub value: u32,
//     pub hi_bit: u8, // highest bit position
//     pub lo_bit: u8, // lowest bit position
//                     // 32 bits, 0 to 31.
//                     // hi and low are used to extract or set bits in the instruction encoding.
// }

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
        InstrFormat::RII => FormatSpec {
            format,
            fields: RII_SPEC,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_range_width() {
        assert_eq!(OPCODE_RANGE.width(), 5);
        assert_eq!(R1_RANGE.width(), 5);
        assert_eq!(IMM_22_RANGE.width(), 22);
        assert_eq!(IMM_17_RANGE.width(), 17);
        assert_eq!(IMM_11_HIGH_RANGE.width(), 11);
        assert_eq!(IMM_27_RANGE.width(), 27);
    }

    #[test]
    fn test_format_specs() {
        let r3_spec = get_format_spec(InstrFormat::R3);
        assert_eq!(r3_spec.fields.len(), 4); // opcode + 3 registers

        let ri_spec = get_format_spec(InstrFormat::RI);
        assert_eq!(ri_spec.fields.len(), 3); // opcode + register + immediate
    }

    #[test]
    fn test_no_overlapping_bits() {
        // Verify that fields don't overlap
        for format in [
            InstrFormat::R2,
            InstrFormat::R3,
            InstrFormat::RI,
            InstrFormat::RRI,
            InstrFormat::RII,
            InstrFormat::I,
        ] {
            let spec = get_format_spec(format);
            let ranges: Vec<_> = spec.fields.iter().map(|f| f.bit_range()).collect();

            for i in 0..ranges.len() {
                for j in (i + 1)..ranges.len() {
                    let r1 = ranges[i];
                    let r2 = ranges[j];
                    // Ranges should not overlap
                    assert!(
                        r1.hi < r2.lo || r2.hi < r1.lo,
                        "Overlapping ranges in {:?}: [{},{}] and [{},{}]",
                        format,
                        r1.hi,
                        r1.lo,
                        r2.hi,
                        r2.lo
                    );
                }
            }
        }
    }
}
