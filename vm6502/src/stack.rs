use core::fmt::{Debug, Formatter, Result};

/* https://www.masswerk.at/6502/6502_instruction_set.html
Processor Stack

LIFO, top-down, 8 bit range, 0x0100 - 0x01FF
*/    
#[derive(Clone)]
pub struct Stack<T> {
    pub data: Vec<T>,
}

trait StackInterface<T> {
    fn pop(&mut self) -> T;
    fn peek(&self) -> T;

    fn push(&mut self,  value: T);
}


impl Stack<u8> {
    pub fn new() -> Self {
        Stack {
            data: vec![0; 0xFF],
        }
    }
}

impl StackInterface<u8> for Stack<u8> {
    fn pop(&mut self) -> u8 {
        self.data.pop().unwrap()
    }

    fn peek(&self) -> u8 {
        self.data.last().unwrap().clone()
    }

    fn push(&mut self, value: u8) {
        self.data.push(value);
    }
}

impl Debug for Stack<u8> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Stack {{ data: {:?} }}", self.data)
    }
}