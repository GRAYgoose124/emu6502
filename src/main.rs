/// Frontend for vm6502.
///
///
use vm6502::prelude::*;

fn main() {
    let mut vm = VirtualMachine::new();

    let prog = "69F0";

    // vm.insert_program(vm.vheap_bounds.1 - (prog.len() / 2), prog);
    vm.insert_program(vm.vheap_bounds.0, prog);
    vm.registers.ac = 0x0F;

    println!("{:?}", vm);

    // Should execute 0x69 on 0x0F and 0xF0 == 0xFF
    vm.tick();

    println!("{:?}", vm);
}
