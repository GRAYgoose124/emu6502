use bitmatch::bitmatch;
use std::fmt::{Debug, Formatter, Result};

use crate::prelude::*;

pub mod prelude {
    pub use crate::control::Mode;
    pub use crate::control::VMControl;
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

pub trait VMControl {
    fn match_instr(&mut self, instr: u8);

    /// macro? fn op_to_mode(&mut self, op: u8) -> (u8, u8);

    fn set_cc0_mode(&mut self, a: u8, b: u8);
    fn set_cc1_mode(&mut self, a: u8, b: u8);
    fn set_cc2_mode(&mut self, a: u8, b: u8);
}

impl VMControl for VirtM {
    fn set_cc1_mode(&mut self, a: u8, b: u8) {
        self.addr_mode = match b {
            0x00 => Mode::IndirectX,
            0x01 => Mode::ZeroPage,
            0x02 => match a {
                0x04 => panic!("Illegal opcode 0x04 for Immediate mode."),
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

    fn set_cc2_mode(&mut self, a: u8, b: u8) {
        self.addr_mode = match b {
            0x00 => match a {
                0x05 => Mode::Immediate,
                _ => panic!("Illegal opcode 0x{:02X} for ZeroPage mode.", a),
            },
            0x01 => Mode::ZeroPage,
            0x02 => match a {
                0x00..=0x03 => Mode::Accumulator,
                0x04..=0x07 => Mode::Implied,
                _ => panic!("Illegal opcode 0x{:02X} for Accumulator/Implied mode.", a),
            },
            0x03 => Mode::Absolute,
            0x04 => Mode::ZeroPageX,
            0x05 => Mode::AbsoluteX,
            _ => panic!("Invalid cc2 mode: {}", b),
        }
    }

    fn set_cc0_mode(&mut self, _a: u8, b: u8) {
        self.addr_mode = match b {
            0x00 => Mode::IndirectX,
            _ => panic!("Invalid cc0 mode: {}", b),
        }
    }

    #[bitmatch]
    fn match_instr(&mut self, instr: u8) {
        #[bitmatch]
        match instr {
            "00000000" => self.brk(),
            "00100000" => self.jsr(), // absolute jsr
            "01000000" => self.rti(),
            "01100000" => self.rts(),
            // cc = 01
            "aaabbb01" => {
                self.set_cc1_mode(a, b);
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
            "aaabbb10" => {
                self.set_cc2_mode(a, b);
                match a {
                    0x00 => self.asl(),
                    0x01 => self.rol(),
                    0x02 => self.lsr(),
                    0x03 => self.ror(),
                    0x04 => self.stx(),
                    0x05 => self.ldx(),
                    0x06 => self.dec(),
                    0x07 => self.inc(),
                    _ => self.nop(),
                }
            }
            // cc = 00
            "aaabbb00" => {
                self.set_cc0_mode(a, b);
                match a {
                    0x00 => self.bit(),
                    0x01 => self.jmp(),
                    0x02 => self.jmp(),
                    0x03 => self.sty(),
                    0x04 => self.ldy(),
                    0x05 => self.cpy(),
                    0x06 => self.cpx(),
                    _ => self.nop(),
                }
            }
            // conditional jumps = xxy10000
            "00010000" => self.bpl(),
            "00110000" => self.bmi(),
            "01010000" => self.bvc(),
            "01110000" => self.bvs(),
            "10010000" => self.bcc(),
            "10110000" => self.bcs(),
            "11010000" => self.bne(),
            "11110000" => self.beq(),
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
