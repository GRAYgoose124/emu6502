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
    /// We set the size to 64k+1 to allow easy indexing.
    #[derivative(Default(value = "BytesMut::zeroed(0x10000)"))]
    pub flatmap: BytesMut,

    /// Machine zero page bounds.
    /// This is a tuple of (start, end) addresses.
    #[derivative(Default(value = "(0x0000, 0x0100)"))]
    pub zero_bounds: (usize, usize),

    /// Machine stack page bounds.
    /// The stack grows downwards from 0x01FF to 0x0100.
    #[derivative(Default(value = "(0x0100, 0x01FF)"))]
    pub stack_bounds: (usize, usize),

    /// Machine heap(dynamic memory) bounds.
    /// This is the only memory that can be dynamically allocated.
    /// Accessing memory outside of these bounds is undefined behavior.
    // TODO: FIX: #[derivative(Default(value = "(0x0200, 0xFFFF)"))]
    #[derivative(Default(value = "(0x0200, 0xFFFF)"))]
    pub heap_bounds: (usize, usize),

    #[derivative(Default(value = "(0x0000, 0xFFFF)"))]
    pub vheap_bounds: (usize, usize),

    /// Interrupt vector table bounds. Placed at end of heap.
    ///
    /// Three vectors are used: reset, irq, nmi. Each vector is 2 bytes.
    #[derivative(Default(value = "(0xFFFA, 0xFFFB)"))]
    pub interrupt_bounds: (usize, usize),
    #[derivative(Default(value = "(0xFFFC, 0xFFFD)"))]
    pub reset_bounds: (usize, usize),
    #[derivative(Default(value = "(0xFFFE, 0xFFFF)"))]
    pub irq_bounds: (usize, usize),

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

        let current_page = (self.get_page_offset() as u16) << 2;
        let page_bounds = (current_page as usize, (current_page + 0xFF) as usize);

        let mut pc_offset = self.heap_bounds.0 + self.registers.pc as usize;
        if pc_offset > self.heap_bounds.1 {
            println!("PC OUT OF BOUNDS");
            pc_offset = self.heap_bounds.1;
        }
        let pc_byte = &self.flatmap[pc_offset];
        write!(
            f,
            "VirtualMachine {{\n\tregisters:\n\t\t{:?}\n\tzero page:\n\t\t{}\n\tstack:\n\t\t{}\n\theap[..0xFF]:\n\t\t{}\n\tpage:\n\t\t{}\n{}\n}}",
            self.registers,
            hexfmt(&self.flatmap[..=0x0FF]),
            hexfmt(&self.flatmap[self.stack_bounds.0..=self.stack_bounds.1]),
            hexfmt(&self.flatmap[self.heap_bounds.0..=self.heap_bounds.0 + 0xFF]),
            hexfmt(&self.flatmap[page_bounds.0..=page_bounds.1]),
            format!("\n\t\tByte at PC: 0x{:02X}", pc_byte)
        )
    }
}
