//! 6502 virtual machine implementation.
//!
//! This crate provides a virtual machine for the 6502 CPU.
//!
//! It is designed to be used as a library, with no frontend.
//!
//! The virtual machine is implemented as a struct, `VirtualMachine`, which contains
//! all the state of the machine. The state is stored in a flat array, which is
//! indexed by the `Registers` struct.
//!
//! The `VirtualMachine` has a flap memory map which the stack and heap index into.
//! See more:
//!
//#![deny(missing_docs)]

mod assembler;
mod program;
mod utils;
mod vm;

pub mod prelude {
    // Virtual machine implementation.
    pub use crate::vm::prelude::*;

    // Virtual machine utilities and macros.
    pub use crate::program::prelude::*;
    pub use crate::utils::prelude::*;

    pub use crate::assembler::prelude::*;
}
