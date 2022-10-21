use crate::prelude::*;

pub mod prelude {
    pub use crate::vm::stack::StackInterface;
}

pub trait StackInterface {
    fn pop(&mut self) -> u8;
    fn peek(&mut self) -> u8; // Not congruent with spec.

    fn push(&mut self, value: u8);
}

impl StackInterface for VirtualMachine {
    fn pop(&mut self) -> u8 {
        let value = self.flatmap[self.stack_bounds.1 - self.registers.sp as usize];

        if cfg!(debug_assertions) {
            println!("Popped value: {}. SP: {}", value, self.registers.sp);
        }

        if self.registers.sp > u8::MIN {
            self.registers.sp -= 1;
        } // TODO panic on underflow.
        value
    }

    // Debug / Not Spec
    fn peek(&mut self) -> u8 {
        let value = self.flatmap[self.stack_bounds.1 - self.registers.sp as usize];
        value
    }

    fn push(&mut self, value: u8) {
        self.flatmap[self.stack_bounds.1 - (self.registers.sp as usize)] = value;

        if self.registers.sp < u8::MAX {
            self.registers.sp += 1;
        } // TODO panic on overflow.
    }
}
