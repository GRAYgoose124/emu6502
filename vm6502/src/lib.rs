//! 6502 virtual machine implementation.
//!
//! This crate provides a virtual machine for the 6502 CPU.
//!
//! It is designed to be used as a library, with no frontend.
//!
//! The virtual machine is implemented as a struct, `VirtM`, which contains
//! all the state of the machine. The state is stored in a flat array, which is
//! indexed by the `Registers` struct.
//!
//#![deny(missing_docs)]

mod utils;
mod vm;

pub mod prelude {
    // Virtual machine implementation.
    pub use crate::vm::prelude::*;

    // Virtual machine utilities and macros.
    pub use crate::utils::prelude::*;
}
