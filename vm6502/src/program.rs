/// Higher level abstractions over vm module.
///
use hex::decode;

use crate::vm::prelude::*;

pub mod prelude {
    pub use crate::program::ProgramController;
}
/// Abstraction layer over crate::vm::VirtualMachine
///
/// This trait is not strictly adhering to the VM hardware,
/// instead it acts as an abstraction layer to the user for
/// applying and using programs with the VM.
pub trait ProgramController {
    /// Insert a hex encoded string `prog` at heap offset `offset`.
    fn insert_program(&mut self, offset: usize, prog: &str);

    /// Fill the stack with ops.
    fn fill_stack(&mut self, ops: Vec<u8>);
}

impl ProgramController for VirtualMachine {
    fn insert_program(&mut self, offset: usize, prog: &str) {
        let offset = offset + self.heap_bounds.0;
        for (i, byte) in decode(prog).unwrap().iter().enumerate() {
            self.flatmap[offset + i] = *byte;
        }
    }

    fn fill_stack(&mut self, ops: Vec<u8>) {
        for (i, byte) in ops.iter().enumerate() {
            if i > 0xFF {
                break;
            };

            self.registers.ac = *byte;
            self.push();
        }
    }
}
