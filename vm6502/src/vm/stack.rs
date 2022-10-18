use crate::prelude::*;

pub mod prelude {
    pub use crate::vm::stack::StackInterface;
}

pub trait StackInterface {
    fn pop(&mut self);
    fn peek(&mut self); // Not congruent with spec.

    fn push(&mut self);
}

impl StackInterface for VirtM {
    fn pop(&mut self) {
        let value = self.flatmap[self.stack_bounds.1 - self.registers.sp as usize];

        if cfg!(debug_assertions) {
            println!("Popped value: {}. SP: {}", value, self.registers.sp);
        }

        if self.registers.sp > u8::MIN {
            self.registers.sp -= 1;
        } // TODO panic on underflow.
        self.registers.ac = value; // VM internal side effect.
    }

    // Debug / Not Spec
    fn peek(&mut self) {
        let value = self.flatmap[self.stack_bounds.1 - self.registers.sp as usize];
        self.registers.ac = value; // VM internal side effect.
    }

    fn push(&mut self) {
        self.flatmap[self.stack_bounds.1 - (self.registers.sp as usize)] = self.registers.ac;

        // TODO: Why is the retrieved value 0?
        if cfg!(debug_assertions) {
            println!(
                "Pushed {} to stack. SP: {}",
                self.flatmap[self.stack_bounds.1 - (self.registers.sp as usize)],
                self.registers.sp
            );
        }

        if self.registers.sp < u8::MAX {
            self.registers.sp += 1;
        } // TODO panic on overflow.
    }
}
