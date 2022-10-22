use vm6502::prelude::*;
use vm6502::status;

// TODO: The problem is likely in the core and functionality.
#[ignore]
#[test]
fn adc_imd() {
    let mut vm = VirtualMachine::new();
    vm.set_program(0x0000, "69F06901");

    vm.registers.ac = 0x0F;
    vm.step();
    assert_eq!(vm.registers.ac, 0xFF);

    vm.step();
    assert_eq!(vm.registers.sr & status!(Status::Carry), 1);
    assert_eq!(vm.registers.ac, 0x00);
}

// TODO: The problem is likely in the core and functionality.
#[ignore]
#[test]
fn and_imd() {
    let mut vm = VirtualMachine::new();
    vm.set_program(0x0000, "29FF29002900");
    vm.registers.ac = 0xFF;

    vm.step();
    assert_eq!(vm.registers.ac, 0xFF);

    vm.step();
    assert_eq!(vm.registers.ac, 0x00);
}

#[test]
fn asl_cover() {
    let mut vm = VirtualMachine::new();
    let prog = "0A0A0A0A0A0A0A0A0A";
    vm.set_program(0x0000, prog);
    vm.registers.ac = 0x01;

    for i in 1..8 {
        vm.step();
        eprintln!("i: {}, ac: {}", 1 << i, vm.registers.ac);

        assert_eq!(vm.registers.ac, 1 << i);
    }
}
