use vm6502::prelude::*;

#[test]
fn test_vm_set_cc1_mode() {
    let mut vm = VirtM::new();
    vm.set_cc1_mode(0x00, 0x00);
    assert_eq!(vm.addr_mode, Mode::IndirectX);
}

#[test]
fn test_vm_match_instr_ora() {
    let mut vm = VirtM::new();
    vm.match_instr(0x01);
    assert_eq!(vm.addr_mode, Mode::IndirectX);
}
