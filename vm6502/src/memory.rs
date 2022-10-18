use core::fmt::{Debug, Formatter, Result};

use bytes::BytesMut;
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Default)]
pub struct Memory {
    #[derivative(Default(value = "BytesMut::zeroed(0xFFFF)"))]
    pub flatmap: BytesMut,

    #[derivative(Default(value = "(0x0000, 0x0099)"))]
    pub zero_bounds: (usize, usize),
    #[derivative(Default(value = "(0x0100, 0x01FF)"))]
    pub stack_bounds: (usize, usize),
    #[derivative(Default(value = "(0x0200, 0xFFFF)"))]
    pub heap_bounds: (usize, usize),
}

impl Memory {
    pub fn new() -> Self {
        Memory::default()
    }
}

impl Debug for Memory {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Memory")
            .field("flatmap", &self.flatmap)
            .field("zero_bounds", &self.zero_bounds)
            .field("stack_bounds", &self.stack_bounds)
            .field("heap_bounds", &self.heap_bounds)
            .finish()
    }
}