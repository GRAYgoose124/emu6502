
#[test]
fn cmp_optable_nops_against_valid_ops() {
    let mut vm = VirtualMachine::new();

    for (i, op) in VALID_OPCODES.iter().enumerate() {
        vm.run_op(*op);

        assert_eq!(vm.addr_mode, COMPLETE_OPCODE_TABLE[i]);
    }
}