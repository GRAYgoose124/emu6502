// TODO: Format heap here instead of VM mod?
// use std::fmt::{Debug, Formatter, Result};

use crate::prelude::*;

pub mod prelude {
    pub use crate::vm::heap::HeapInterface;
}

/// Provides a low level interface for accessing the heap.
///
/// It's simply a wrapper around the flatmap, using the internal [heap_bounds.0](VirtualMachine::heap_bounds) to index the heap.
pub trait HeapInterface {
    // Low level iinterface
    /// Returns the value at the heap address given.
    fn get_heap(&self, virt_addr: u16) -> u8;
    /// Sets the value at the heap address given.
    fn set_heap(&mut self, virt_addr: u16, byte: u8);

    // Mid level interface
    // return the bytes, 0xHH__ from the PC. More of a convenience/debug function.
    fn get_page_offset(&self) -> u8;
    // Add an the virt_addr to the high byte of the PC - it's a "magic" jump, bypassing modes.
    fn set_page_offset(&mut self, virt_addr: u8);

    fn bounds_check(&self, virt_addr: u16) -> bool;
}

impl HeapInterface for VirtualMachine {
    fn get_heap(&self, virt_addr: u16) -> u8 {
        #[cfg(feature = "check_heap_bounds")]
        if !self.bounds_check(self.registers.pc + virt_addr) {
            println!("Crossed virtual heap bounds!");
        } else if !self.bounds_check(self.registers.pc + virt_addr) {
            println!("Crossed heap bounds!");
        }

        self.flatmap[self.heap_bounds.0 + self.registers.pc as usize + virt_addr as usize]
    }

    fn set_heap(&mut self, virt_addr: u16, byte: u8) {
        #[cfg(feature = "check_heap_bounds")]
        if !self.bounds_check(self.registers.pc + virt_addr) {
            println!("Crossed virtual heap bounds!");
        } else if !self.bounds_check(self.registers.pc + virt_addr) {
            println!("Crossed heap bounds!");
        }

        self.flatmap[self.heap_bounds.0 + (self.registers.pc + virt_addr) as usize] = byte;
    }

    fn get_page_offset(&self) -> u8 {
        let virt_page = (self.registers.pc & 0xFF00) >> 8;

        virt_page as u8
    }

    fn set_page_offset(&mut self, virt_addr: u8) {
        let new_pc = (self.registers.pc & 0x00FF) | (virt_addr as u16) << 8;
        #[cfg(feature = "check_heap_bounds")]
        if !self.bounds_check(new_pc) {
            println!("Crossed virtual heap bounds!")
        };

        self.registers.pc = new_pc;
    }

    fn bounds_check(&self, virt_addr: u16) -> bool {
        if virt_addr < self.vheap_bounds.0 as u16 {
            #[cfg(feature = "passthrough_failure")]
            {
                panic!("Attempted to access heap before heap bounds!");
            }
            #[cfg(not(feature = "passthrough_failure"))]
            {
                false
            }
        } else if virt_addr > self.vheap_bounds.1 as u16 {
            #[cfg(feature = "passthrough_failure")]
            {
                panic!("Attempted to access heap after heap bounds!");
            }
            #[cfg(not(feature = "passthrough_failure"))]
            {
                false
            }
        } else {
            true
        }
    }
}

pub trait HeapController {
    // High level interface
    fn alloc(&mut self);
    fn dealloc(&mut self);
}
