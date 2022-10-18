pub mod prelude {
    pub use crate::stuff_program_at_end;
    pub use crate::{make_status, status};
}

#[macro_use]
mod vm_macros {
    #[macro_export]
    macro_rules! stuff_program_at_end {
        ($vm:expr, $prog:expr) => {
            let offset = $vm.vheap_bounds.1 - ($prog.len() / 2);
            $vm.insert_program(offset, $prog);
        };
    }
}

#[macro_use]
mod status_macros {
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
