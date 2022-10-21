/// Frontend for vm6502.
use std::time::Duration;

use vm6502::prelude::*;

fn do_program(vm: &mut VirtualMachine, prog: &str, offset: u16) {
    vm.set_program(offset, prog);
    println!("Loaded program and set PC appropriately:\n{}\n", prog);

    println!("VM state before execution: {:?}\n", vm);

    println!("Running program...");
    let (cycles, time) = vm.run(Duration::from_millis(1000));

    println!("\nFinal state: {:?}", vm);
    println!("\tHalt state: {:?}", vm.halted);
    println!(
        "\tCycles: 0x{:X}\t\tTotal time: {:?},   {:.1}C/s",
        cycles,
        time,
        cycles as f64 / time.as_secs_f64()
    );
}

fn main() {
    let mut vm = VirtualMachine::new();

    do_program(&mut vm, "69016901690169016901690169016901690169016901690169016901690169016901690169016901690169016901690169016901690169016901", 0x0000);
}
