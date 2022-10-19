use vm6502::prelude::*;

#[test]
fn test_cc0_mode() {
    let mut vm = VirtualMachine::new();
    let mode = vm.cc0_mode(0x00, 0x00);
    assert_eq!(mode, Mode::Implied);
}

#[test]
fn test_cc1_mode() {
    let mut vm = VirtualMachine::new();
    let addr_mode = vm.cc1_mode(0x00, 0x00);
    assert_eq!(addr_mode, Mode::IndirectX);
}

#[test]
fn test_cc2_mode() {
    let mut vm = VirtualMachine::new();
    let addr_mode = vm.cc2_mode(0x00, 0x00);
    assert_eq!(addr_mode, Mode::Implied);
}

#[test]
fn test_mode() {
    let mut vm = VirtualMachine::new();

    for i in 0..N_OPS {
        let mode = vm.mode(OPCODES[i]);
        eprintln!("{}, 0x{:02X}: {:?}, {:?}", i, OPCODES[i], mode, OP_MODES[i]);

        assert_eq!(mode, OP_MODES[i]);
    }
}

#[test]
fn test_run_op() {
    let mut vm = VirtualMachine::new();

    for i in 0..N_OPS {
        vm.run_op(OPCODES[i]);

        eprintln!(
            "{}, 0x{:02X}: {:?}, {:?}",
            i, OPCODES[i], vm.addr_mode, OP_MODES[i]
        );

        assert_eq!(vm.addr_mode, OP_MODES[i]);
    }
}
