use rand::prelude::*;

mod memory;
mod registers;
mod stack;
mod vm;
mod utils;

pub mod prelude {
    pub use crate::memory::Memory;
    pub use crate::registers::Registers;
    
    pub use crate::vm::prelude::*;
    pub use crate::utils::prelude::*;
}
