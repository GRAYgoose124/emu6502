use core::fmt::{Debug, Formatter, Result};

use bytes::BytesMut;
use derivative::Derivative;
use hex::decode;

use crate::prelude::*;

pub mod prelude {
    pub use crate::vm::StackInterface;
    pub use crate::vm::StatusInterface;
    pub use crate::vm::VirtM;
}

#[derive(Derivative)]
#[derivative(Default)]
pub struct VirtM {
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

impl VirtM {
    pub fn new() -> Self {
        VirtM::default()
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.addr_mode = mode;
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

pub trait StatusInterface {
    fn flip_status(&mut self, flag: Status);

    fn set_status(&mut self, flag: Status, value: bool);
    fn get_status(&self, flag: Status) -> bool;

    fn reset_status(&mut self);
}

impl StatusInterface for VirtM {
    fn flip_status(&mut self, flag: Status) {
        let status = self.registers.sr;

        self.registers.sr = status ^ status!(flag);
    }

    fn set_status(&mut self, flag: Status, value: bool) {
        let status = self.registers.sr;

        if value {
            self.registers.sr = status | status!(flag);
        } else {
            self.registers.sr = status & !status!(flag);
        }
    }

    fn get_status(&self, flag: Status) -> bool {
        let status = self.registers.sr;

        status & status!(flag) != 0
    }

    fn reset_status(&mut self) {
        self.registers.sr = 0x00;
    }
}

pub trait StackInterface {
    fn pop(&mut self);
    fn peek(&mut self); // Not congruent with spec.

    fn push(&mut self);
}

impl StackInterface for VirtM {
    fn pop(&mut self) {
        let value = self.flatmap[self.stack_bounds.1 - self.registers.sp as usize];
        self.registers.sp += 0x01;
        self.registers.ac = value; // VM internal side effect.
    }

    // Debug / Not Spec
    fn peek(&mut self) {
        let value = self.flatmap[self.stack_bounds.1 - self.registers.sp as usize];
        self.registers.ac = value; // VM internal side effect.
    }

    fn push(&mut self) {
        let value = self.registers.ac; // VM internal retrieval.
        self.flatmap[self.stack_bounds.1 - self.registers.sp as usize] = value;
        self.registers.sp -= 0x01;
    }
}

impl Debug for VirtM {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "VirtM {{ registers: {:?}, flatmap: {:?} }}",
            self.registers, self.flatmap
        )
    }
}

pub trait HeapInterface {
    fn alloc(&mut self);
    fn dealloc(&mut self);
}
