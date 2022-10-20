// TODO: Format heap here instead of VM mod?
// use std::fmt::{Debug, Formatter, Result};

use crate::prelude::*;

pub mod prelude {
    pub use crate::vm::heap::HeapInterface;
}

pub trait HeapInterface {
    // Low level iinterface
    fn get_heap(&self, offset: usize) -> u8;
    fn set_heap(&mut self, offset: usize, byte: u8);
}

impl HeapInterface for VirtualMachine {
    fn get_heap(&self, offset: usize) -> u8 {
        // TODO: bounds checks
        self.flatmap[self.heap_bounds.0 + self.registers.pc as usize + offset]
    }

    fn set_heap(&mut self, offset: usize, byte: u8) {
        // TODO: bounds checks
        self.flatmap[self.heap_bounds.0 + self.registers.pc as usize + offset] = byte;
    }
}

pub trait HeapController {
    // High level interface
    fn alloc(&mut self);
    fn dealloc(&mut self);
}
