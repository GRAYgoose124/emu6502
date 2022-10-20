/// Frontend for vm6502.
///
///
use vm6502::prelude::*;

fn main() {
    let mut vm = VirtualMachine::new();

    let prog = "69F00A";

    // vm.insert_program(vm.vheap_bounds.1 - (prog.len() / 2), prog);
    vm.insert_program(vm.vheap_bounds.0, prog);
    vm.registers.ac = 0x0F;
    debug_assert_eq!(vm.registers.pc, 0x0000);
    debug_assert_eq!(vm.registers.ac, 0x0F);

    vm.tick();
    debug_assert_eq!(vm.registers.ac, 0xFF);

    vm.tick();
    debug_assert_eq!(vm.registers.ac, 0xFE);

    println!("{:?}", vm);
}
