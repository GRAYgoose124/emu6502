use vm6502::prelude::*;

/// TODO: MORE TESTCASES!

#[test]
fn test_mode() {
    let mut vm = VirtualMachine::new();

    for (i, op) in VALID_OPCODES.iter().enumerate() {
        let mode = vm.mode(*op);

        assert_eq!(mode, OP_MODES[i]);
    }
}

// TODO:
//fn test_step() {}
