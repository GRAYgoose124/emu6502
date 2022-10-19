/// This module implements the 6502 registers.
///
///    Note: The status register (SR) is also known as the P register.

///
#[derive(Debug, Clone, Copy)]
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
    ///
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
