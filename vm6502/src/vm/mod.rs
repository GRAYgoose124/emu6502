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
            hex::encode(&self.flatmap[self.stack_bounds.0..self.stack_bounds.1]).to_uppercase(),
            hex::encode(&self.flatmap[self.heap_bounds.0..self.heap_bounds.0 + 0xFF]).to_uppercase()
        )
    }
}

pub trait HeapInterface {
    fn alloc(&mut self);
    fn dealloc(&mut self);
}
