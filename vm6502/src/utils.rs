pub mod prelude {
    pub use crate::stuff_program_at_end;
    pub use crate::utils::machine_arrays::{N_OPS, OPCODES, OP_MODES};
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

pub mod machine_arrays {
    pub mod prelude {
        pub use crate::utils::machine_arrays::{OPCODES, OP_MODES};
    }
    pub const N_OPS: usize = 151;

    // All valid ops in order, including all mode variants. TODO: move to utility.
    pub static OPCODES: [u8; N_OPS] = [
        0x00, 0x01, 0x05, 0x06, 0x08, 0x09, 0x0A, 0x0D, 0x0E, 0x10, 0x11, 0x15, 0x16, 0x18, 0x19,
        0x1D, 0x1E, 0x20, 0x21, 0x24, 0x25, 0x26, 0x28, 0x29, 0x2A, 0x2C, 0x2D, 0x2E, 0x30, 0x31,
        0x35, 0x36, 0x38, 0x39, 0x3D, 0x3E, 0x40, 0x41, 0x45, 0x46, 0x48, 0x49, 0x4A, 0x4C, 0x4D,
        0x4E, 0x50, 0x51, 0x55, 0x56, 0x58, 0x59, 0x5D, 0x5E, 0x60, 0x61, 0x65, 0x66, 0x68, 0x69,
        0x6A, 0x6C, 0x6D, 0x6E, 0x70, 0x71, 0x75, 0x76, 0x78, 0x79, 0x7D, 0x7E, 0x81, 0x84, 0x85,
        0x86, 0x88, 0x8A, 0x8C, 0x8D, 0x8E, 0x90, 0x91, 0x94, 0x95, 0x96, 0x98, 0x99, 0x9A, 0x9D,
        0xA0, 0xA1, 0xA2, 0xA4, 0xA5, 0xA6, 0xA8, 0xA9, 0xAA, 0xAC, 0xAD, 0xAE, 0xB0, 0xB1, 0xB4,
        0xB5, 0xB6, 0xB8, 0xB9, 0xBA, 0xBC, 0xBD, 0xBE, 0xC0, 0xC1, 0xC4, 0xC5, 0xC6, 0xC8, 0xC9,
        0xCA, 0xCC, 0xCD, 0xCE, 0xD0, 0xD1, 0xD5, 0xD6, 0xD8, 0xD9, 0xDD, 0xDE, 0xE0, 0xE1, 0xE4,
        0xE5, 0xE6, 0xE8, 0xE9, 0xEA, 0xEC, 0xED, 0xEE, 0xF0, 0xF1, 0xF5, 0xF6, 0xF8, 0xF9, 0xFD,
        0xFE,
    ];

    use crate::prelude::Mode;
    use crate::prelude::Mode::*;
    pub static OP_MODES: [Mode; N_OPS] = [
        Implied,
        IndirectX,
        ZeroPage,
        ZeroPage,
        Implied,
        Immediate,
        Accumulator,
        Absolute,
        Absolute,
        Relative,
        IndirectY,
        ZeroPageX,
        ZeroPageX,
        Implied,
        AbsoluteY,
        AbsoluteX,
        AbsoluteX,
        Absolute,
        IndirectX,
        ZeroPage,
        ZeroPage,
        ZeroPage,
        Implied,
        Immediate,
        Accumulator,
        Absolute,
        Absolute,
        Absolute,
        Relative,
        IndirectY,
        ZeroPageX,
        ZeroPageX,
        Implied,
        AbsoluteY,
        AbsoluteX,
        AbsoluteX,
        Implied,
        IndirectX,
        ZeroPage,
        ZeroPage,
        Implied,
        Immediate,
        Accumulator,
        Absolute,
        Absolute,
        Absolute,
        Relative,
        IndirectY,
        ZeroPageX,
        ZeroPageX,
        Implied,
        AbsoluteY,
        AbsoluteX,
        AbsoluteX,
        Implied,
        IndirectX,
        ZeroPage,
        ZeroPage,
        Implied,
        Immediate,
        Accumulator,
        Indirect,
        Absolute,
        Absolute, //61
        Relative,
        IndirectY,
        ZeroPageX,
        ZeroPageX,
        Implied,
        AbsoluteY,
        AbsoluteX,
        AbsoluteX,
        IndirectX,
        ZeroPage,
        ZeroPage,
        ZeroPage,
        Implied,
        Implied,
        Absolute,
        Absolute,
        Absolute,
        Relative,
        IndirectY,
        ZeroPageX,
        ZeroPageX,
        ZeroPageY,
        Implied,
        AbsoluteY,
        Implied,
        AbsoluteX,
        Immediate,
        IndirectX,
        Immediate,
        ZeroPage,
        ZeroPage,
        ZeroPage,
        Implied,
        Immediate,
        Implied,
        Absolute,
        Absolute,
        Absolute,
        Relative,
        IndirectY,
        ZeroPageX,
        ZeroPageX,
        ZeroPageY,
        Implied,
        AbsoluteY,
        Implied,
        AbsoluteX,
        AbsoluteX,
        AbsoluteY,
        Immediate,
        IndirectX,
        ZeroPage,
        ZeroPage,
        ZeroPage,
        Implied,
        Immediate,
        Implied,
        Absolute,
        Absolute,
        Absolute,
        Relative,
        IndirectY,
        ZeroPageX,
        ZeroPageX,
        Implied,
        AbsoluteY,
        AbsoluteX,
        AbsoluteX,
        Immediate,
        IndirectX,
        ZeroPage,
        ZeroPage,
        ZeroPage,
        Implied,
        Immediate,
        Implied,
        Absolute,
        Absolute,
        Absolute,
        Relative,
        IndirectY,
        ZeroPageX,
        ZeroPageX,
        Implied,
        AbsoluteY,
        AbsoluteX,
        AbsoluteX,
    ];
}
