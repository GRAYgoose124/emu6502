use crate::prelude::*;

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

impl StatusInterface for VirtualMachine {
    fn flip_status(&mut self, flag: Status) {
        let status = self.registers.sr;

        self.registers.sr = status ^ status!(flag);
    }

    fn set_status(&mut self, flag: Status, value: bool) {
        let status = self.registers.sr;

        if value {
            self.registers.sr = status | status!(flag);
        } else {
            self.registers.sr = status & !status!(flag);
        }
    }

    fn get_status(&self, flag: Status) -> bool {
        let status = self.registers.sr;

        status & status!(flag) != 0
    }

    fn reset_status(&mut self) {
        self.registers.sr = 0x00;
    }
}

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
