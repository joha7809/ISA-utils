use crate::types::{InstrFormat, Operand};

pub struct InstructionLayout {
    pub format: InstrFormat,
    pub fields: Vec<BitField>,
}

pub struct BitField {
    pub kind: FieldKind,
    pub value: u32,
    pub hi_bit: u8, // highest bit position
    pub lo_bit: u8, // lowest bit position
                    // 32 bits, 0 to 31.
                    // hi and low are used to extract or set bits in the instruction encoding.
}

impl BitField {
    pub fn width(&self) -> u8 {
        // Calculate width of the bit field we add one since both hi_bit and lo_bit are inclusive
        self.hi_bit - self.lo_bit + 1
    }
}

pub enum FieldKind {
    Opcode,
    Register,
    Immediate,
}

fn to_u32(op: Operand) -> u32 {
    // I was lazy and just made this to extract u32 from Operand enum
    match op {
        Operand::Register(n) => n as u32,
        Operand::Immediate(n) => n as u32,
    }
}

// Implement From ResolvedInstruction to InstructionLayout conversion
impl From<&crate::types::ResolvedInstruction> for InstructionLayout {
    fn from(value: &crate::types::ResolvedInstruction) -> Self {
        // Since Instruction has been resolved we are sure it matches its format!
        let format = value.opcode.instruction_format();
        let opcode_value = value.opcode.code();
        assert!(value.operands.len() == format.size() - 1); // -1 since opcode is not in operands vec

        let mut fields = Vec::new();

        // All formats start with opcode in bits [31:27]
        fields.push(BitField {
            kind: FieldKind::Opcode,
            value: opcode_value as u32,
            hi_bit: 31,
            lo_bit: 27,
        });

        // Ugly code incomming
        match format {
            InstrFormat::R2 => {
                let r1 = to_u32(value.operands[0]);
                let r2 = to_u32(value.operands[1]);

                fields.push(BitField {
                    kind: FieldKind::Register,
                    value: r1,
                    hi_bit: 26,
                    lo_bit: 22,
                });
                fields.push(BitField {
                    kind: FieldKind::Register,
                    value: r2,
                    hi_bit: 21,
                    lo_bit: 17,
                });
            }

            InstrFormat::R3 => {
                let r1 = to_u32(value.operands[0]);
                let r2 = to_u32(value.operands[1]);
                let r3 = to_u32(value.operands[2]);

                fields.push(BitField {
                    kind: FieldKind::Register,
                    value: r1,
                    hi_bit: 26,
                    lo_bit: 22,
                });
                fields.push(BitField {
                    kind: FieldKind::Register,
                    value: r2,
                    hi_bit: 21,
                    lo_bit: 17,
                });
                fields.push(BitField {
                    kind: FieldKind::Register,
                    value: r3,
                    hi_bit: 16,
                    lo_bit: 12,
                });
            }

            InstrFormat::RI => {
                let r = to_u32(value.operands[0]);
                let imm = to_u32(value.operands[1]);

                fields.push(BitField {
                    kind: FieldKind::Register,
                    value: r,
                    hi_bit: 26,
                    lo_bit: 22,
                });
                fields.push(BitField {
                    kind: FieldKind::Immediate,
                    value: imm,
                    hi_bit: 21,
                    lo_bit: 0,
                });
            }

            InstrFormat::RRI => {
                let r1 = to_u32(value.operands[0]);
                let r2 = to_u32(value.operands[1]);
                let imm = to_u32(value.operands[2]);

                fields.push(BitField {
                    kind: FieldKind::Register,
                    value: r1,
                    hi_bit: 26,
                    lo_bit: 22,
                });
                fields.push(BitField {
                    kind: FieldKind::Register,
                    value: r2,
                    hi_bit: 21,
                    lo_bit: 17,
                });
                fields.push(BitField {
                    kind: FieldKind::Immediate,
                    value: imm,
                    hi_bit: 16,
                    lo_bit: 0,
                });
            }

            InstrFormat::RII => {
                let r = to_u32(value.operands[0]);
                let imm1 = to_u32(value.operands[1]);
                let imm2 = to_u32(value.operands[2]);

                fields.push(BitField {
                    kind: FieldKind::Register,
                    value: r,
                    hi_bit: 26,
                    lo_bit: 22,
                });
                fields.push(BitField {
                    kind: FieldKind::Immediate,
                    value: imm1,
                    hi_bit: 21,
                    lo_bit: 11,
                });
                fields.push(BitField {
                    kind: FieldKind::Immediate,
                    value: imm2,
                    hi_bit: 10,
                    lo_bit: 0,
                });
            }

            InstrFormat::I => {
                let imm = to_u32(value.operands[0]);
                fields.push(BitField {
                    kind: FieldKind::Immediate,
                    value: imm,
                    hi_bit: 26,
                    lo_bit: 0,
                });
            }

            InstrFormat::NoOP => fields.push(BitField {
                kind: FieldKind::Immediate,
                value: 0,
                hi_bit: 26,
                lo_bit: 0,
            }),
        }

        Self { format, fields }
    }
}
