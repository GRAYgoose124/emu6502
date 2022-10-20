use vm6502::prelude::*;

// TODO: Backend to test direct instruction matching without
// modifying machine state. API decisions
#[test]
fn test_vminstr_adc() {
    let mut vm = VirtualMachine::new();
    let offset = 0x0000;
    vm.insert_program(offset, "69ABEF");
    vm.registers.pc = offset as u16;

    vm.tick();

    assert_eq!(vm.addr_mode, Mode::Immediate);
    // assert_eq!(vm.registers.ac, 0x00);
}

#[test]
fn test_vm_instr_asl() {
    let mut vm = VirtualMachine::new();
    let prog = "0A";
    vm.registers.ac = 0xFF;

    vm.insert_program(vm.vheap_bounds.0, prog);
    vm.tick();
}
