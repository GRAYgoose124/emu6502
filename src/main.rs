/// Frontend for vm6502.
use std::time::Duration;

use vm6502::prelude::*;

fn main() {
    let mut vm = VirtualMachine::new();

    let prog = "69016901690169016901690169016901690169016901690169016901690169016901690169016901690169016901690169016901690169016901";

    println!("Loading program: {}", prog);
    vm.insert_program(vm.vheap_bounds.0 as u16, prog);

    println!("Running program...");
    let cycles = vm.run(Duration::from_micros(10000));

    println!("Final VM state: {:?}", vm);
    println!("Cycles over 10000us: 0x{:X}", cycles);
    println!("Halt state: {:?}", vm.halted);
}
