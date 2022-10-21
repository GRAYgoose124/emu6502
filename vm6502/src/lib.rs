//! 6502 virtual machine implementation.
//!
//! This crate provides a virtual machine for the 6502 CPU.
//!
//! It is designed to be used as a library, with no frontend.
//!
//! # Virtual machine
//! The virtual machine is implemented as a struct, `VirtualMachine`, which contains
//! all the state of the machine. The state is stored in a flat array, which is
//! indexed by the `Registers` struct.
//!
//! ## Specifications
//! ### Registers
//! THere are 8 status flags, stored in the [registers](crate::prelude::Registers).
//!
//! S/N:  The B flag is not actually set on the real 6502 status register, instead it's configured in memory when it's pushed and pulled. (Presumably during BRK)
//!
//! ##### Flags
//! ### Memory
//! The [VirtualMachine](crate::prelude::VirtualMachine) has a flat memory map which the stack and heap index into.
//!
//! ### Addressing modes
//! ### Instruction set
//! [Instructions](crate::prelude::Instructions)
//! ## Macros
//! Several macros are provided for more easily interacting with the machine and wielding opcodes.
//! [See more.](crate::utils)
//!
//! # !! In construction !!
//! Also provided is an [assembler](crate::assembler) and a [programmer](crate::program).
//#![deny(missing_docs)]

pub mod assembler;
pub mod program;
pub mod utils;
pub mod vm;

pub mod prelude {
    /// Virtual machine implementation prelude
    pub use crate::vm::prelude::*;

    // Virtual machine utilities and macros.
    pub use crate::program::prelude::*;

    pub use crate::utils::prelude::*;

    pub use crate::assembler::prelude::*;
}
