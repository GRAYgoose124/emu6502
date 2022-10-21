/// Frontend for vm6502.
use std::time::Duration;

use vm6502::prelude::*;

fn main() {
    let mut vm = VirtualMachine::new();

    let prog = "69016901690169016901690169016901690169016901690169016901690169016901690169016901690169016901690169016901690169016901";
    vm.insert_program(vm.vheap_bounds.0 as u16, prog);

    let cycles = vm.run(Duration::from_micros(10000));
    println!("Cycles over 10000us: {}", cycles);
}
