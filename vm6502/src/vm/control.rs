use bitmatch::bitmatch;
use std::fmt::{Debug, Formatter, Result};

use crate::prelude::*;

pub mod prelude {
    pub use crate::vm::control::InstructionController;
    pub use crate::vm::control::Mode;
}

#[derive(PartialEq)]
pub enum Mode {
    Accumulator,
    Implied,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
}

impl Debug for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Mode::Accumulator => write!(f, "Accumulator"),
            Mode::Implied => write!(f, "Implied"),
            Mode::Immediate => write!(f, "Immediate"),
            Mode::ZeroPage => write!(f, "ZeroPage"),
            Mode::ZeroPageX => write!(f, "ZeroPageX"),
            Mode::ZeroPageY => write!(f, "ZeroPageY"),
            Mode::Relative => write!(f, "Relative"),
            Mode::Absolute => write!(f, "Absolute"),
            Mode::AbsoluteX => write!(f, "AbsoluteX"),
            Mode::AbsoluteY => write!(f, "AbsoluteY"),
            Mode::Indirect => write!(f, "Indirect"),
            Mode::IndirectX => write!(f, "IndirectX"),
            Mode::IndirectY => write!(f, "IndirectY"),
        }
    }
}

pub trait InstructionController {
    fn run_op(&mut self, op: u8);

    /// macro? fn op_to_mode(&mut self, op: u8) -> (u8, u8);
    fn mode(&mut self, op: u8) -> Mode;

    fn cc0_mode(&mut self, a: u8, b: u8) -> Mode;
    fn cc1_mode(&mut self, a: u8, b: u8) -> Mode;
    fn cc2_mode(&mut self, a: u8, b: u8) -> Mode;
}

impl InstructionController for VirtualMachine {
    fn cc1_mode(&mut self, a: u8, b: u8) -> Mode {
        match b {
            0x00 => Mode::IndirectX,
            0x01 => Mode::ZeroPage,
            0x02 => match a {
                0x04 => panic!("Illegal opcode 0x04 for cc1.(b=0x02)"),
                _ => Mode::Immediate,
            },
            0x03 => Mode::Absolute,
            0x04 => Mode::IndirectY,
            0x05 => Mode::ZeroPageX,
            0x06 => Mode::AbsoluteY,
            0x07 => Mode::AbsoluteX,
            _ => panic!("Invalid cc1 mode: {}", b),
        }
    }

    fn cc2_mode(&mut self, a: u8, b: u8) -> Mode {
        match b {
            0x00 => match a {
                0x00 => Mode::Implied,
                0x05 => Mode::Immediate,
                _ => panic!("Illegal a value {} for cc2(b=0x00)", a),
            },
            0x01 => Mode::ZeroPage,
            0x02 => match a {
                0x00..=0x03 => Mode::Accumulator,
                0x04..=0x07 => Mode::Implied,
                _ => panic!("Illegal a value {} for cc2(b=0x02)", a),
            },
            0x03 => Mode::Absolute,
            0x04 => Mode::ZeroPageX,
            0x05 => match a {
                0x00..=0x03 | 0x06 | 0x07 => Mode::ZeroPageX,
                0x04 | 0x05 => Mode::ZeroPageY,
                _ => panic!("Illegal a value {} for cc2.(b=0x05)", a),
            },
            0x06 => match a {
                0x04 | 0x05 => Mode::Implied,
                _ => panic!("Illegal a value {} for cc2.(b=0x06)", a),
            },
            0x07 => match a {
                0x00..=0x03 | 0x06 | 0x07 => Mode::AbsoluteX,
                0x05 => Mode::AbsoluteY,
                _ => panic!("Illegal a value {} for cc2.(b=0x07)", a),
            },
            _ => panic!("Invalid cc2 mode: {}", b),
        }
    }

    fn cc0_mode(&mut self, a: u8, b: u8) -> Mode {
        match b {
            0x00 => match a {
                0x00 => Mode::Implied,
                0x01 => Mode::Absolute,
                0x02 | 0x03 => Mode::Implied,
                0x05..=0x07 => Mode::Immediate,
                _ => panic!("Illegal a value {} for cc0.(b=0x00)", a),
            },
            0x01 => match a {
                0x01 => Mode::ZeroPage,
                0x04..=0x07 => Mode::ZeroPage,
                _ => panic!("Illegal a value {:02X} for cc0.(b=0x04..0x07)", a),
            },
            0x02 => Mode::Implied,
            0x03 => match a {
                0x00 => panic!("Illegal opcode 0x00 for cc0."),
                0x03 => Mode::Indirect,
                0x01 | 0x02 | 0x04..=0x07 => Mode::Absolute,
                _ => panic!("Illegal a value {} for cc0.(b=0x01..0x07)", a),
            },
            0x04 => Mode::Relative,
            0x05 => match a {
                0x04 | 0x05 => Mode::ZeroPageX,
                _ => panic!("Illegal a value {} for cc0.(b=0x04|0x05)", a),
            },
            0x06 => Mode::Implied,
            0x07 => match a {
                0x05 => Mode::AbsoluteX,
                _ => panic!("Illegal a value {} for cc0.", a),
            },
            _ => panic!("Invalid cc0 mode: {}", b),
        }
    }

    #[bitmatch]
    fn mode(&mut self, op: u8) -> Mode {
        #[bitmatch]
        match op {
            "aaabbbcc" => match c {
                0x00 => self.cc0_mode(a, b),
                0x01 => self.cc1_mode(a, b),
                0x02 => self.cc2_mode(a, b),
                _ => panic!("Invalid mode: {}", c),
            },
        }
    }

    #[bitmatch]
    fn run_op(&mut self, op: u8) {
        self.addr_mode = self.mode(op);

        #[bitmatch]
        match op {
            "00000000" => self.brk(),
            "01000000" => self.rti(),
            "01100000" => self.rts(),
            // cc = 01
            "aaabbb01" => {
                // We could fetch with self.mode but we avoid the extraneous match.
                match a {
                    0x00 => self.ora(),
                    0x01 => self.and(),
                    0x02 => self.eor(),
                    0x03 => self.adc(),
                    0x04 => self.sta(),
                    0x05 => self.lda(),
                    0x06 => self.cmp(),
                    0x07 => self.sbc(),
                    _ => self.nop(),
                }
            }
            // cc = 10
            "aaabbb10" => match a {
                0x00 => self.asl(),
                0x01 => self.rol(),
                0x02 => self.lsr(),
                0x03 => self.ror(),
                0x04 => self.stx(),
                0x05 => self.ldx(),
                0x06 => self.dec(),
                0x07 => self.inc(),
                _ => self.nop(),
            },
            // cc = 00
            "aaabbb00" => match a {
                0x00 => self.bit(),
                0x01 => self.jsr(),
                0x02 => self.jmp(),
                0x03 => self.sty(),
                0x04 => self.ldy(),
                0x05 => self.cpy(),
                0x06 => self.cpx(),
                _ => self.nop(),
            },
            // conditional jumps = aab10000
            "aab10000" => {
                let bb = b + 1;
                match "aab" {
                    "000" => self.bpl(),
                    "001" => self.bmi(),
                    "010" => self.bvc(),
                    "011" => self.bvs(),
                    "100" => self.bcc(),
                    "101" => self.bcs(),
                    "110" => self.bne(),
                    "111" => self.beq(),
                    _ => self.nop(),
                }
            }

            // no pattern
            "00001000" => self.php(),
            "00101000" => self.plp(),
            "01001000" => self.pha(),
            "01101000" => self.pla(),
            "10001000" => self.dey(),
            "10101000" => self.tay(),
            "01001100" => self.iny(),
            "11101000" => self.inx(),
            "00011000" => self.clc(),
            "00111000" => self.sec(),
            "01011000" => self.cli(),
            "01111000" => self.sei(),
            "10011000" => self.tya(),
            "10111000" => self.clv(),
            "11011000" => self.cld(),
            "11111000" => self.sed(),
            "10001010" => self.txa(),
            "10011010" => self.txs(),
            "10101010" => self.tax(),
            "10111010" => self.tsx(),
            "11001010" => self.dex(),
            "11101010" => self.nop(),
            _ => self.nop(),
        };
    }
}
