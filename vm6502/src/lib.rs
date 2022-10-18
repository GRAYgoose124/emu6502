mod control;
mod instructions;
mod registers;
mod utils;
mod vm;

pub mod prelude {
    pub use crate::registers::Registers;

    pub use crate::control::prelude::*;
    pub use crate::instructions::prelude::*;
    pub use crate::utils::prelude::*;
    pub use crate::vm::prelude::*;
}
