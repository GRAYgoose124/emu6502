/// Frontend for vm6502.
///
///
use vm6502::prelude::*;
use vm6502::status;

fn main() {
    let mut vm = VirtualMachine::new();

    let prog = "69F00A290069FF9002";

    // vm.insert_program(vm.vheap_bounds.1 - (prog.len() / 2), prog);
    vm.insert_program(vm.vheap_bounds.0, prog);
    vm.registers.ac = 0x0F;
    debug_assert_eq!(vm.registers.pc, 0x0000);
    debug_assert_eq!(vm.registers.ac, 0x0F);

    vm.tick();
    debug_assert_eq!(vm.registers.ac, 0xFF);

    vm.tick();
    debug_assert_eq!(vm.registers.ac, 0xFE);

    vm.tick();
    debug_assert_eq!(vm.registers.ac, 0x00);

    vm.registers.ac = 0x01;
    vm.tick();
    debug_assert_eq!(vm.registers.ac, 0x00);
    debug_assert_eq!(vm.registers.sr & status!(Status::Carry), 1);

    vm.registers.sr &= !status!(Status::Carry);
    vm.tick();

    vm.reset();
    vm.insert_program(vm.vheap_bounds.0, "90FF");
    vm.tick();
    assert_eq!(vm.registers.pc, 0xFF);
}
