use hex::decode;
/// Higher level abstractions over vm module.
use std::time::{Duration, Instant};

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
    fn insert_program(&mut self, offset: u16, prog: &str);
    fn set_program(&mut self, offset: u16, prog: &str);

    /// TODO meta memory controller/allocator.
    /// Run the internally set program.
    fn run(&mut self, duration: Duration) -> u64;

    /// Fill the stack with ops.
    fn fill_stack(&mut self, ops: Vec<u8>);

    /// Reset machine state.
    fn reset(&mut self);
}

impl ProgramController for VirtualMachine {
    /// Insert a hex encoded string `prog` at heap offset `offset`.s
    fn insert_program(&mut self, offset: u16, prog: &str) {
        let offset = offset + self.heap_bounds.0 as u16;
        let decoded = if let Ok(d) = decode(prog) {
            d
        } else {
            panic!("Failed to decode program - it probably wasn't byte aligned or hex encoded.");
        };

        for (i, byte) in decoded.iter().enumerate() {
            self.flatmap[offset as usize + i] = *byte;
        }
    }

    // TODO: Higher level program allocator.
    /// Replaces and runs the program at `offset`.
    fn set_program(&mut self, offset: u16, prog: &str) {
        self.insert_program(offset, prog);
        self.registers.pc = offset as u16;
    }

    /// Run the internally set program for `duration` time, returning the number of cycles executed.
    fn run(&mut self, duration: Duration) -> u64 {
        let old_cycles = self.cycles;
        let start = Instant::now();
        while start.elapsed() < duration {
            self.step();
        }

        self.cycles - old_cycles
    }

    /// Resets the total machine state.
    fn reset(&mut self) {
        self.flatmap.iter_mut().for_each(|m| {
            *m = 0;
        });

        self.registers = Registers::new();
        self.cycles = 0;
    }

    // TODO: move to helpers? macro?
    /// Fill the stack with ops.
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
