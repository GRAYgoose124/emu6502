
pub mod prelude {
    pub use crate::utils::status_macros::Status;
}



#[macro_use]
mod status_macros {
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

    impl From<u8> for Status {
        fn from(value: u8) -> Self {
            match value {
                0 => Status::Negative,
                1 => Status::Overflow,
                2 => Status::Unused,
                3 => Status::Break,
                4 => Status::Decimal,
                5 => Status::Interrupt,
                6 => Status::Zero,
                7 => Status::Carry,
                _ => unreachable!(),
            }
        }
    }

    impl Into<u8> for Status {
        fn into(self) -> u8 {
            match self {
                Status::Negative => 0,
                Status::Overflow => 1,
                Status::Unused => 2,
                Status::Break => 3,
                Status::Decimal => 4,
                Status::Interrupt => 5,
                Status::Zero => 6,
                Status::Carry => 7,
            }
        }
    }

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