use std::ops::{Index, IndexMut};

/*
PC	program counter	(16 bit)
AC	accumulator	(8 bit)
X	X register	(8 bit)
Y	Y register	(8 bit)
SR	status register [NV-BDIZC]	(8 bit)
SP	stack pointer	(8 bit)

Note: The status register (SR) is also known as the P register.
SR Flags (bit 7 to bit 0)
N	Negative
V	Overflow
-	ignored
B	Break
D	Decimal (use BCD for arithmetics)
I	Interrupt (IRQ disable)
Z	Zero
C	Carry
*/

#[derive(Debug, Clone, Copy)]
pub struct Registers {
    pub pc: u16,
    pub ac: u8,
    pub x: u8,
    pub y: u8,
    pub sr: u8,
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
