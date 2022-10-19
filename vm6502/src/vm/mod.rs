use core::fmt::{Debug, Formatter, Result};

use bytes::BytesMut;
use derivative::Derivative;
use hex::decode;

use crate::prelude::*;

mod control;
mod instructions;
mod registers;
mod stack;
mod status;

pub mod prelude {
    pub use crate::vm::registers::Registers;
    pub use crate::vm::VirtualMachine;

    // Virtual machine control functionality.
    pub use crate::vm::control::prelude::*;

    // Virtual machine instructions set.
    pub use crate::vm::instructions::prelude::*;

    pub use crate::vm::stack::prelude::*;
    pub use crate::vm::status::prelude::*;
}

#[derive(Derivative)]
#[derivative(Default)]
pub struct VirtualMachine {
    #[derivative(Default(value = "Registers::new()"))]
    pub registers: Registers,
    #[derivative(Default(value = "BytesMut::zeroed(0xFFFF)"))]
    pub flatmap: BytesMut,

    #[derivative(Default(value = "(0x0000, 0x0099)"))]
    pub zero_bounds: (usize, usize),
    #[derivative(Default(value = "(0x0100, 0x01FF)"))]
    pub stack_bounds: (usize, usize),

    #[derivative(Default(value = "(0x0200, 0xFFFF)"))]
    pub heap_bounds: (usize, usize),

    #[derivative(Default(value = "(0x0000, 0xFDFF)"))]
    pub vheap_bounds: (usize, usize),

    #[derivative(Default(value = "Mode::Absolute"))]
    pub addr_mode: Mode,
}

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine::default()
    }

    pub fn fetch(&mut self) -> u8 {
        match self.addr_mode {
            // OPC A
            Mode::Accumulator => self.registers.ac,
            // OPC $LLHH
            // operand is address $HHLL
            Mode::Absolute => {
                let ll = self.flatmap[self.registers.pc as usize + 1];
                let hh = self.flatmap[self.registers.pc as usize + 2];
                self.flatmap[(hh as usize) << 8 | ll as usize]
            }
            // OPC $LLHH,X
            Mode::AbsoluteX => {
                let ll = self.flatmap[self.registers.pc as usize + 1];
                let hh = self.flatmap[self.registers.pc as usize + 2];
                self.flatmap[(hh as usize) << 8 | ll as usize + self.registers.x as usize]
            }
            // OPC $LLHH,Y
            Mode::AbsoluteY => {
                let ll = self.flatmap[self.registers.pc as usize + 1];
                let hh = self.flatmap[self.registers.pc as usize + 2];
                self.flatmap[(hh as usize) << 8 | ll as usize + self.registers.y as usize]
            }
            // OPC #$BB
            Mode::Immediate => self.flatmap[self.registers.pc as usize + 1],
            // OPC
            Mode::Implied => 0,
            // OPC ($LLHH)
            Mode::Indirect => {
                let ll = self.flatmap[self.registers.pc as usize + 1];
                let hh = self.flatmap[self.registers.pc as usize + 2];
                let addr = (hh as usize) << 8 | ll as usize;
                self.flatmap[addr]
            }
            // OPC ($LL, X)
            // operand is zeropage address; effective address is word in (LL + X, LL + X + 1),
            // inc. without carry: C.w($00LL + X)
            Mode::IndirectX => {
                let ll = self.flatmap[(self.registers.pc + 1) as usize];
                let ell = self.flatmap[ll as usize + self.registers.x as usize];
                let ehh = self.flatmap[ll as usize + self.registers.x as usize + 1];
                let addr = (ehh as usize) << 8 | ell as usize;

                self.flatmap[addr]
            }
            // OPC ($LL), Y
            // operand is zeropage address; effective address is word in (LL, LL + 1)
            // incremented by Y with carry: C.w($00LL) + Y
            // TODO: check if this is correct.
            Mode::IndirectY => {
                let ll = self.flatmap[(self.registers.pc + 1) as usize];
                let ell = self.flatmap[ll as usize];
                let ehh = self.flatmap[ll as usize + 1];
                let addr = (ehh as usize) << 8 | ell as usize + self.registers.y as usize;

                self.flatmap[addr]
            }
            // OPC $BB
            Mode::Relative => {
                let bb = self.flatmap[self.registers.pc as usize + 1];
                // TODO: Write a test for this.
                self.flatmap[self.registers.pc.wrapping_add_signed((bb as i8).into()) as usize]
            }
            // OPC $LL
            Mode::ZeroPage => {
                let ll = self.flatmap[self.registers.pc as usize + 1];
                self.flatmap[ll as usize]
            }
            // OPC $LL, X
            Mode::ZeroPageX => {
                let ll = self.flatmap[self.registers.pc as usize + 1];
                self.flatmap[ll as usize + self.registers.x as usize]
            }
            // OPC $LL, Y
            Mode::ZeroPageY => {
                let ll = self.flatmap[self.registers.pc as usize + 1];
                self.flatmap[ll as usize + self.registers.y as usize]
            }
        }
    }

    pub fn insert_program(&mut self, offset: usize, prog: &str) {
        let offset = offset + self.heap_bounds.0;
        for (i, byte) in decode(prog).unwrap().iter().enumerate() {
            self.flatmap[offset + i] = *byte;
        }
    }
}

impl Debug for VirtualMachine {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "VirtualMachine {{ registers: {:?}, stack: {:?}, heap[..0x400..]: {:?} }}",
            self.registers,
            &self.flatmap[self.stack_bounds.0..self.stack_bounds.1],
            &self.flatmap[self.heap_bounds.0..0x400]
        )
    }
}

pub trait HeapInterface {
    fn alloc(&mut self);
    fn dealloc(&mut self);
}
