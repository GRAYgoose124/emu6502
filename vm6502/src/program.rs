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
    /// Insert a hex encoded string `prog` at heap offset `offset` and set the PC to `offset`.
    fn set_program(&mut self, offset: u16, prog: &str);

    /// Set the interrupt vectors to the given values.
    fn set_interrupt_vectors(&mut self, nmi: u16, irq: u16, brk: u16);
    /// Set the interrupt vectors to the values: (0xFFFA, 0xFFFB), (0xFFFC, 0xFFFD), (0xFFFE, 0xFFFF)
    fn default_interrupt_vectors(&mut self);

    /// Run the internal program.
    fn execute(&mut self) -> u64;

    /// Run the internally set program at `offset` for `duration`.
    fn run(&mut self, duration: Duration) -> (u64, Duration);

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
        self.registers.pc = offset;
    }

    fn set_interrupt_vectors(&mut self, nmi: u16, irq: u16, brk: u16) {
        self.flatmap[self.interrupt_bounds.0] = (nmi & 0xFF) as u8;
        self.flatmap[self.interrupt_bounds.1] = (nmi >> 8) as u8;

        self.flatmap[self.reset_bounds.0] = (irq & 0xFF) as u8;
        self.flatmap[self.reset_bounds.1] = (irq >> 8) as u8;

        self.flatmap[self.irq_bounds.0] = (brk & 0xFF) as u8;
        self.flatmap[self.irq_bounds.1] = (brk >> 8) as u8;
    }

    fn default_interrupt_vectors(&mut self) {
        self.set_interrupt_vectors(0xFFFA, 0xFFFC, 0xFFFE);
    }

    /// Run the internally set program. Intended API for running programs.
    fn execute(&mut self) -> u64 {
        let old_cycles = self.cycles;

        while self.halted == false {
            self.step();
        }

        self.cycles - old_cycles
    }

    /// Run the internally set program for `duration` time, returning the number of cycles executed.
    fn run(&mut self, duration: Duration) -> (u64, Duration) {
        // Save cycles for delta.
        let old_cycles = self.cycles;

        let start = Instant::now();
        while start.elapsed() < duration && self.halted == false {
            self.step();

            if self.registers.pc == self.irq_bounds.0 as u16 {
                self.halted = true;
            }
        }

        (self.cycles - old_cycles, start.elapsed())
    }

    /// Resets the total machine state.
    fn reset(&mut self) {
        self.flatmap.iter_mut().for_each(|m| {
            *m = 0;
        });

        self.registers = Registers::new();
        self.cycles = 0;
        self.halted = false;
    }

    // TODO: move to helpers? macro?
    /// Fill the stack with ops.
    fn fill_stack(&mut self, ops: Vec<u8>) {
        for (i, byte) in ops.iter().enumerate() {
            if i > 0xFF {
                break;
            };

            self.registers.ac = *byte;
            self.push(self.registers.ac);
        }
    }
}
