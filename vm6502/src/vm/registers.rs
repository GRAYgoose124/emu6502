/// This module implements the 6502 registers.
///
///    Note: The status register (SR) is also known as the P register.
use std::fmt::{Debug, Formatter, Result};

pub mod prelude {
    pub use crate::vm::registers::Registers;
}

///
///
///
#[derive(Clone, Copy)]
pub struct Registers {
    /// Program counter
    pub pc: u16,
    /// Accumulator
    pub ac: u8,
    /// X register
    pub x: u8,
    /// Y register
    pub y: u8,
    /// Stack register
    pub sr: u8,
    /// Flag register
    /// - `SR`  Flags (bit 7 to bit 0)
    /// - `N`	Negative
    /// - `V`	Overflow
    /// - `-`	ignored
    /// - `B`	Break
    /// - `D`	Decimal (use BCD for arithmetics)
    /// - `I`	Interrupt (IRQ disable)
    /// - `Z`	Zero
    /// - `C`	Carry
    pub sp: u8,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            pc: 0x0000,
            ac: 0x00,
            x: 0x00,
            y: 0x00,
            sr: 0x00,
            sp: 0x00,
        }
    }
}

impl Debug for Registers {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "PC: {:04X}\tSP: {:02X}\tAC: {:02X}\n\t\tX: {:02X}\tY: {:02X}\t\tSR: {:02X}",
            self.pc, self.sp, self.ac, self.x, self.y, self.sr
        )
    }
}
