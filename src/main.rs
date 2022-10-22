//! Frontend for vm6502.
//!
//! This crate provides a user-end accessible binary to use the virtual machine.
//!
//! It is meant for demonstration purposes towards using the [](vm6502) crate.
use std::time::Duration;

use vm6502::prelude::*;

// Run the given program on the given vitual machine.
fn do_program(vm: &mut VirtualMachine, offset: u16, prog: &str) {
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

    do_program(&mut vm, 0x0000, "00");
    vm.reset();
}
