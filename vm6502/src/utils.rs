pub mod prelude {
    pub use crate::utils::machine_arrays::prelude::*;

    pub use crate::utils::macros::*;
}

mod macros {
    pub use crate::utils::machine_data_macros::*;
    pub use crate::utils::status_macros::*;
}

#[macro_use]
mod vm_macros {
    #[macro_export]
    macro_rules! stuff_program_at_end {
        ($vm:expr, $prog:expr) => {
            let offset = ($vm.heap_bounds.1 - $prog.len());
            $vm.insert_program(offset as u16, $prog);
        };
    }

    #[macro_export]
    macro_rules! check_page_cross {
        ($vm:expr, $new_pc:expr) => {
            // We're comparing the page bytes, and don't care about the lower bytes.
            $new_pc & 0xFF00 != $vm.registers.pc & 0xFF00
        };
    }
}

#[macro_use]
mod status_macros {
    /// Convert Status to binary flag.
    ///  
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

    /// Make an arbitrary state from flags.
    ///
    /// # Example
    /// ```
    /// use vm6502::prelude::*;
    /// use vm6502::{make_status, status};
    ///
    /// assert_eq!(make_status!( Status::Negative, Status::Overflow ), 0b11000000)
    /// ```
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

/// This module contains statics, functions, and macros for creating the machine data.
pub mod machine_arrays {
    pub mod prelude {
        pub use crate::utils::machine_arrays::{
            valid_op,
            COMPLETE_OPCODE_TABLE,
            N_VALID_OPS,
            OP_MODES, //VALID_CYCLE_COUNTS,
            VALID_OPCODES,
        };
    }

    /// Total number of valid opcodes.
    pub const N_VALID_OPS: usize = 151;

    /// All valid ops in order, including all mode variants.
    pub static VALID_OPCODES: [u8; N_VALID_OPS] = [
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

    //The total number of cycles VALID_OPCODES[\n] should spend.
    //
    // This is mostly for debuggin purposes and is to be deprecated for internal use.
    //pub static VALID_CYCLE_COUNTS: [u8; N_VALID_OPS] = [];

    /// All opcodes and their names, as tuples in order.
    ///
    /// For example, 0x00 is "BRK", 0x01 is "ORA (indirect, X)".
    ///
    /// Ex. `COMPLETE_OPCODE_TABLE[0] == ("BRK", 0x00)`
    ///
    /// # Use valid_op to filter out invalid ops.
    /// ## Example:
    /// ```
    /// use vm6502::prelude::*;
    ///
    /// for (name, op) in COMPLETE_OPCODE_TABLE.iter().filter(|(_, op)| valid_op(*op)) {
    ///      println!("0x:{:02X} is {}", *op, name);
    /// }
    /// ```
    pub static COMPLETE_OPCODE_TABLE: [(&str, u8); 256] = [
        ("BRK", 0x00),
        ("ORA", 0x01),
        ("NOP", 0x02),
        ("NOP", 0x03),
        ("NOP", 0x04),
        ("ORA", 0x05),
        ("ASL", 0x06),
        ("NOP", 0x07),
        ("PHP", 0x08),
        ("ORA", 0x09),
        ("ASL", 0x0A),
        ("NOP", 0x0B),
        ("NOP", 0x0C),
        ("ORA", 0x0D),
        ("ASL", 0x0E),
        ("NOP", 0x0F),
        ("BPL", 0x10),
        ("ORA", 0x11),
        ("NOP", 0x12),
        ("NOP", 0x13),
        ("NOP", 0x14),
        ("ORA", 0x15),
        ("ASL", 0x16),
        ("NOP", 0x17),
        ("CLC", 0x18),
        ("ORA", 0x19),
        ("NOP", 0x1A),
        ("NOP", 0x1B),
        ("NOP", 0x1C),
        ("ORA", 0x1D),
        ("ASL", 0x1E),
        ("NOP", 0x1F),
        ("JSR", 0x20),
        ("AND", 0x21),
        ("NOP", 0x22),
        ("NOP", 0x23),
        ("BIT", 0x24),
        ("AND", 0x25),
        ("ROL", 0x26),
        ("NOP", 0x27),
        ("PLP", 0x28),
        ("AND", 0x29),
        ("ROL", 0x2A),
        ("NOP", 0x2B),
        ("BIT", 0x2C),
        ("AND", 0x2D),
        ("ROL", 0x2E),
        ("NOP", 0x2F),
        ("BMI", 0x30),
        ("AND", 0x31),
        ("NOP", 0x32),
        ("NOP", 0x33),
        ("NOP", 0x34),
        ("AND", 0x35),
        ("ROL", 0x36),
        ("NOP", 0x37),
        ("SEC", 0x38),
        ("AND", 0x39),
        ("NOP", 0x3A),
        ("NOP", 0x3B),
        ("NOP", 0x3C),
        ("AND", 0x3D),
        ("ROL", 0x3E),
        ("NOP", 0x3F),
        ("RTI", 0x40),
        ("EOR", 0x41),
        ("NOP", 0x42),
        ("NOP", 0x43),
        ("NOP", 0x44),
        ("EOR", 0x45),
        ("LSR", 0x46),
        ("NOP", 0x47),
        ("PHA", 0x48),
        ("EOR", 0x49),
        ("LSR", 0x4A),
        ("NOP", 0x4B),
        ("JMP", 0x4C),
        ("EOR", 0x4D),
        ("LSR", 0x4E),
        ("NOP", 0x4F),
        ("BVC", 0x50),
        ("EOR", 0x51),
        ("NOP", 0x52),
        ("NOP", 0x53),
        ("NOP", 0x54),
        ("EOR", 0x55),
        ("LSR", 0x56),
        ("NOP", 0x57),
        ("CLI", 0x58),
        ("EOR", 0x59),
        ("NOP", 0x5A),
        ("NOP", 0x5B),
        ("NOP", 0x5C),
        ("EOR", 0x5D),
        ("LSR", 0x5E),
        ("NOP", 0x5F),
        ("RTS", 0x60),
        ("ADC", 0x61),
        ("NOP", 0x62),
        ("NOP", 0x63),
        ("NOP", 0x64),
        ("ADC", 0x65),
        ("ROR", 0x66),
        ("NOP", 0x67),
        ("PLA", 0x68),
        ("ADC", 0x69),
        ("ROR", 0x6A),
        ("NOP", 0x6B),
        ("JMP", 0x6C),
        ("ADC", 0x6D),
        ("ROR", 0x6E),
        ("NOP", 0x6F),
        ("BVS", 0x70),
        ("ADC", 0x71),
        ("NOP", 0x72),
        ("NOP", 0x73),
        ("NOP", 0x74),
        ("ADC", 0x75),
        ("ROR", 0x76),
        ("NOP", 0x77),
        ("SEI", 0x78),
        ("ADC", 0x79),
        ("NOP", 0x7A),
        ("NOP", 0x7B),
        ("NOP", 0x7C),
        ("ADC", 0x7D),
        ("ROR", 0x7E),
        ("NOP", 0x7F),
        ("NOP", 0x80),
        ("STA", 0x81),
        ("NOP", 0x82),
        ("NOP", 0x83),
        ("STY", 0x84),
        ("STA", 0x85),
        ("STX", 0x86),
        ("NOP", 0x87),
        ("DEY", 0x88),
        ("NOP", 0x89),
        ("TXA", 0x8A),
        ("NOP", 0x8B),
        ("STY", 0x8C),
        ("STA", 0x8D),
        ("STX", 0x8E),
        ("NOP", 0x8F),
        ("BCC", 0x90),
        ("STA", 0x91),
        ("NOP", 0x92),
        ("NOP", 0x93),
        ("STY", 0x94),
        ("STA", 0x95),
        ("STX", 0x96),
        ("NOP", 0x97),
        ("TYA", 0x98),
        ("STA", 0x99),
        ("TXS", 0x9A),
        ("NOP", 0x9B),
        ("NOP", 0x9C),
        ("STA", 0x9D),
        ("NOP", 0x9E),
        ("NOP", 0x9F),
        ("LDY", 0xA0),
        ("LDA", 0xA1),
        ("LDX", 0xA2),
        ("NOP", 0xA3),
        ("LDY", 0xA4),
        ("LDA", 0xA5),
        ("LDX", 0xA6),
        ("NOP", 0xA7),
        ("TAY", 0xA8),
        ("LDA", 0xA9),
        ("TAX", 0xAA),
        ("NOP", 0xAB),
        ("LDY", 0xAC),
        ("LDA", 0xAD),
        ("LDX", 0xAE),
        ("NOP", 0xAF),
        ("BCS", 0xB0),
        ("LDA", 0xB1),
        ("NOP", 0xB2),
        ("NOP", 0xB3),
        ("LDY", 0xB4),
        ("LDA", 0xB5),
        ("LDX", 0xB6),
        ("NOP", 0xB7),
        ("CLV", 0xB8),
        ("LDA", 0xB9),
        ("TSX", 0xBA),
        ("NOP", 0xBB),
        ("LDY", 0xBC),
        ("LDA", 0xBD),
        ("LDX", 0xBE),
        ("NOP", 0xBF),
        ("CPY", 0xC0),
        ("CMP", 0xC1),
        ("NOP", 0xC2),
        ("NOP", 0xC3),
        ("CPY", 0xC4),
        ("CMP", 0xC5),
        ("DEC", 0xC6),
        ("NOP", 0xC7),
        ("INY", 0xC8),
        ("CMP", 0xC9),
        ("DEX", 0xCA),
        ("NOP", 0xCB),
        ("CPY", 0xCC),
        ("CMP", 0xCD),
        ("DEC", 0xCE),
        ("NOP", 0xCF),
        ("BNE", 0xD0),
        ("CMP", 0xD1),
        ("NOP", 0xD2),
        ("NOP", 0xD3),
        ("NOP", 0xD4),
        ("CMP", 0xD5),
        ("DEC", 0xD6),
        ("NOP", 0xD7),
        ("CLD", 0xD8),
        ("CMP", 0xD9),
        ("NOP", 0xDA),
        ("NOP", 0xDB),
        ("NOP", 0xDC),
        ("CMP", 0xDD),
        ("DEC", 0xDE),
        ("NOP", 0xDF),
        ("CPX", 0xE0),
        ("SBC", 0xE1),
        ("NOP", 0xE2),
        ("NOP", 0xE3),
        ("CPX", 0xE4),
        ("SBC", 0xE5),
        ("INC", 0xE6),
        ("NOP", 0xE7),
        ("INX", 0xE8),
        ("SBC", 0xE9),
        ("NOP", 0xEA),
        ("NOP", 0xEB),
        ("CPX", 0xEC),
        ("SBC", 0xED),
        ("INC", 0xEE),
        ("NOP", 0xEF),
        ("BEQ", 0xF0),
        ("SBC", 0xF1),
        ("NOP", 0xF2),
        ("NOP", 0xF3),
        ("NOP", 0xF4),
        ("SBC", 0xF5),
        ("INC", 0xF6),
        ("NOP", 0xF7),
        ("SED", 0xF8),
        ("SBC", 0xF9),
        ("NOP", 0xFA),
        ("NOP", 0xFB),
        ("NOP", 0xFC),
        ("SBC", 0xFD),
        ("INC", 0xFE),
        ("NOP", 0xFF),
    ];

    pub fn valid_op(op: u8) -> bool {
        for i in 0..N_VALID_OPS {
            if VALID_OPCODES[i] == op {
                return true;
            }
        }
        return false;
    }

    use crate::prelude::Mode;
    use crate::prelude::Mode::*;
    /// The expected modes for the VALID_OPCODES.
    pub static OP_MODES: [Mode; N_VALID_OPS] = [
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
        Absolute,
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

#[macro_use]
pub mod machine_data_macros {
    /// Macro to get the name of an opcode from its value.
    /// # Example
    /// ```
    /// use vm6502::opcode_name;
    /// use vm6502::prelude::COMPLETE_OPCODE_TABLE;
    ///
    /// assert_eq!(opcode_name!(0x69), "ADC");
    /// ```
    /// # Panics
    /// Panics if the opcode is not valid.
    #[macro_export]
    macro_rules! opcode_name {
        ($op:expr) => {
            COMPLETE_OPCODE_TABLE[$op as usize].0
        };
    }

    /// Macro to get the value of an opcode from its name.
    /// # Example
    /// ```
    /// use vm6502::opcode;
    /// use vm6502::prelude::COMPLETE_OPCODE_TABLE;
    ///
    /// for op in opcode!("ADC") {
    ///    match op {
    ///       0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => assert!(true),
    ///      _ => assert!(false)
    /// }
    /// }
    #[macro_export]
    macro_rules! opcode {
        ($op:expr) => {
            COMPLETE_OPCODE_TABLE
                .iter()
                .filter_map(|(name, op)| if name == &$op { Some(*op) } else { None })
                .collect::<Vec<u8>>()
        };
    }
}
