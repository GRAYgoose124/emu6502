
pub mod prelude {
    pub use crate::utils::Status;
}

pub enum Status {
    Negative,
    Overflow,
    Unused,
    Break,
    Decimal,
    Interrupt,
    Zero,
    Carry,
}

#[macro_use]
mod status_macros {
    use super::*;
    #[macro_export]
    macro_rules! status {
        ($flag: expr) => {
            match $flag {
                Status::Negative => 0b10000000,
                Status::Overflow => 0b01000000,
                Status::Unused => 0,
                Status::Break => 0b00010000,
                Status::Decimal => 0b00001000,
                Status::Interrupt => 0b00000100,
                Status::Zero => 0b00000010,
                Status::Carry => 0b00000001,
            }
        };
    }

    #[macro_export]
    macro_rules! make_status {
        () => {
            0b00000000
        };
        ($($flag: expr),*) => {
            0b00000000 $(| status!($flag))*
        };
    }
}