use core::fmt::{Debug, Formatter, Result};

use bytes::BytesMut;
use hex::decode;
use derivative::Derivative;

use crate::prelude::*;

pub mod prelude {
    pub use crate::vm::VirtM;
    pub use crate::vm::StackInterface;
    pub use crate::vm::StatusInterface;
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

    pub fn fetch(&mut self, addr: usize) -> u8 {
        match self.addr_mode {
            // OPC A
            Mode::Accumulator => self.registers.ac,
            // OPC $LLHH
            Mode::Absolute => self.flatmap[addr],
            _ => todo!(),
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
        self.registers.ac = value;  // VM internal side effect.
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
        write!(f, "VirtM {{ registers: {:?}, flatmap: {:?} }}", self.registers, self.flatmap)
    }
}

pub trait HeapInterface {
    fn aladdr(&mut self);
    fn dealaddr(&mut self);
}