use core::fmt::{Debug, Formatter, Result};

use bytes::BytesMut;
use derivative::Derivative;

use crate::prelude::*;

mod control;
mod heap;
mod instructions;
mod registers;
mod stack;
mod status;

/// Uses everything necessary for the full 6502 vm to run.
pub mod prelude {
    // Expose virtual machine.
    pub use crate::program::prelude::*;
    pub use crate::vm::VirtualMachine;

    // Virtual machine control functionality.
    pub use crate::vm::control::prelude::*;

    // Virtual machine instructions set.
    pub use crate::vm::instructions::prelude::*;

    pub use crate::vm::heap::prelude::*;
    pub use crate::vm::registers::prelude::*;
    pub use crate::vm::stack::prelude::*;
    pub use crate::vm::status::prelude::*;
}

/// The virtual machine implementation
///
/// This module provides VirtualMachine, a 6502 cpu vm.
/// It's API is intended to closely adhere to the 6502 specs.
///
/// See [Masswerk's 6502 Instruction Set](https://www.masswerk.at/6502/6502_instruction_set.html) for more info on the spec.
// TODO vstack and vheap so that you don't have to index yourself.
#[derive(Derivative)]
#[derivative(Default)]
pub struct VirtualMachine {
    /// Machine registers struct.
    #[derivative(Default(value = "Registers::new()"))]
    pub registers: Registers,
    /// The machine memory in a linear layout.
    #[derivative(Default(value = "BytesMut::zeroed(0xFFFF)"))]
    pub flatmap: BytesMut,

    /// Machine zero page bounds.
    #[derivative(Default(value = "(0x0000, 0x0099)"))]
    pub zero_bounds: (usize, usize),

    /// Machine stack page bounds.
    #[derivative(Default(value = "(0x0100, 0x01FF)"))]
    pub stack_bounds: (usize, usize),

    /// Machine heap(dynamic memory) bounds.
    #[derivative(Default(value = "(0x0200, 0xFFFF)"))]
    pub heap_bounds: (usize, usize),

    /// Virtual addressable heap memory access.
    #[derivative(Default(value = "(0x0000, 0xFDFF)"))]
    pub vheap_bounds: (usize, usize),

    /// Current mode state, this is generally set internally by [step](InstructionController::step).
    #[derivative(Default(value = "Mode::Absolute"))]
    pub addr_mode: Mode,

    /// The current cycle count of the vm. This is incremented by [step](InstructionController::step).
    #[derivative(Default(value = "0"))]
    pub cycles: u64,

    #[derivative(Default(value = "false"))]
    pub halted: bool,
}

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine::default()
    }
}

impl Debug for VirtualMachine {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let hexfmt = |s: &[u8]| -> String {
            hex::encode(s)
                .to_uppercase()
                .chars()
                .collect::<Vec<char>>()
                .chunks(64)
                .map(|c| c.iter().collect::<String>())
                .collect::<Vec<String>>()
                .join("\n\t\t")
        };

        let window = self.heap_bounds.0 + self.registers.pc as usize;

        write!(
            f,
            "VirtualMachine {{\n\tregisters:\n\t\t{:?}\n\tzero page:\n\t\t{}\n\tstack:\n\t\t{}\n\theap[..0xFF]:\n\t\t{}\n\twindow: {}\n}}",
            self.registers,
            hexfmt(&self.flatmap[..=0x0FF]),
            hexfmt(&self.flatmap[self.stack_bounds.0..=self.stack_bounds.1]),
            hexfmt(&self.flatmap[self.heap_bounds.0..=self.heap_bounds.0 + 0xFF]),
            format!("\n\t\tByte at PC: {:}", hexfmt(&self.flatmap[window-2..=window+2]))
        )
    }
}
