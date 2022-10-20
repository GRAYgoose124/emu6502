use std::fmt::{Debug, Formatter, Result};

use crate::prelude::*;
use crate::status;

pub mod prelude {
    pub use crate::vm::status::Status;
    pub use crate::vm::status::StatusInterface;
}

pub trait StatusInterface {
    fn flip_status(&mut self, flag: Status);

    fn set_status(&mut self, flag: Status, value: bool);
    fn get_status(&self, flag: Status) -> bool;

    fn reset_status(&mut self);
}

/// Implements a high level interface for accessing the status register.
///
/// This is the intended API access for frontends to use the VM.
///
impl StatusInterface for VirtualMachine {
    fn flip_status(&mut self, flag: Status) {
        let status = self.registers.sr;

        self.registers.sr = status ^ status!(flag);
    }

    fn set_status(&mut self, flag: Status, value: bool) {
        #[cfg(feature = "show_status_set")]
        println!("\t\t\tsetting status: {:?} to {}", flag, value);

        let status = self.registers.sr;

        if value {
            self.registers.sr = status | status!(flag);
        } else {
            self.registers.sr = status & !status!(flag);
        }

        #[cfg(feature = "show_status")]
        println!("\t\t\tNV-BDIZC\n\t\t\t{:08b}", self.registers.sr);
    }

    fn get_status(&self, flag: Status) -> bool {
        let status = self.registers.sr;

        #[cfg(feature = "show_status_get")]
        println!(
            "\t\t\tgetting status: {:?} = {}",
            flag,
            status & status!(flag) != 0
        );

        status & status!(flag) != 0
    }

    fn reset_status(&mut self) {
        self.registers.sr = 0x00;
    }
}

#[derive(PartialEq)]
pub enum Status {
    Negative,
    Overflow,
    Unused,
    Break, // Not Real - set when sr is pushed to stack.
    Decimal,
    Interrupt,
    Zero,
    Carry,
}

// TODO: set values to equal binary flags for easier usage.
impl From<u8> for Status {
    fn from(value: u8) -> Self {
        match value {
            64 => Status::Negative,
            32 => Status::Overflow,
            16 => Status::Unused,
            8 => Status::Break,
            4 => Status::Decimal,
            2 => Status::Interrupt,
            1 => Status::Zero,
            0 => Status::Carry,
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
            Status::Break => 4,
            Status::Decimal => 8,
            Status::Interrupt => 16,
            Status::Zero => 32,
            Status::Carry => 64,
        }
    }
}

impl Debug for Status {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Status::Negative => write!(f, "Negative"),
            Status::Overflow => write!(f, "Overflow"),
            Status::Unused => write!(f, "Unused"),
            Status::Break => write!(f, "Break"),
            Status::Decimal => write!(f, "Decimal"),
            Status::Interrupt => write!(f, "Interrupt"),
            Status::Zero => write!(f, "Zero"),
            Status::Carry => write!(f, "Carry"),
        }
    }
}
