use core::fmt::{Debug, Formatter, Result};

use bytes::BytesMut;
use derivative::Derivative;

use crate::registers::Registers;
use crate::memory::Memory;

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
}

impl VirtM {
    pub fn new() -> Self {
        VirtM::default()
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