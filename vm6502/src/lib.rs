use rand::prelude::*;

mod registers;
mod vm;
mod utils;

pub mod prelude {
    pub use crate::registers::Registers;

    pub use crate::vm::prelude::*;
    pub use crate::utils::prelude::*;
}
