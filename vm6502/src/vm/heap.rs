pub trait HeapInterface {
    fn alloc(&mut self);
    fn dealloc(&mut self);
}
