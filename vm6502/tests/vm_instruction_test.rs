use vm6502::prelude::*;
use vm6502::status;

// TODO: Backend to test direct instruction matching without
// modifying machine state. API decisions
#[test]
fn test_vm_instr_adc_imd() {
    let mut vm = VirtualMachine::new();
    let offset = 0x0000;
    vm.insert_program(offset, "69F06901");
    vm.registers.pc = offset as u16;
    vm.registers.ac = 0x0F;
    vm.tick();

    assert_eq!(vm.registers.ac, 0xFF);

    vm.tick();
    assert_eq!(vm.registers.sr & status!(Status::Carry), 1);
    assert_eq!(vm.registers.ac, 0x00);
}

#[test]
fn test_vm_instr_and_imd() {
    let mut vm = VirtualMachine::new();
    let offset = 0x0000;
    vm.insert_program(offset, "29FF2900");
    vm.registers.pc = offset as u16;
    vm.registers.ac = 0x0F;
    vm.tick();

    assert_eq!(vm.registers.ac, 0x0F);
    assert_eq!(vm.registers.sr & status!(Status::Zero), 0);

    vm.tick();
    assert_eq!(vm.registers.ac, 0x00);
    eprintln!("sr: 0x{:08b}", vm.registers.sr);
    assert_eq!(
        vm.registers.sr & status!(Status::Zero),
        status!(Status::Zero)
    );
}

#[test]
fn test_vm_instr_asl() {
    let mut vm = VirtualMachine::new();
    let prog = "0A0A";
    vm.registers.ac = 0xFF;

    vm.insert_program(vm.vheap_bounds.0, prog);
    vm.tick();
    assert_eq!(vm.registers.ac, 0xFE);
    assert_eq!(vm.registers.sr & status!(Status::Carry), 1);

    vm.tick();
    assert_eq!(vm.registers.ac, 0xFC);
    assert_eq!(vm.registers.sr & status!(Status::Carry), 1);
}

#[test]
fn test_vm_instr_cover_asl() {
    let mut vm = VirtualMachine::new();
    let prog = "0A0A0A0A0A0A0A0A0A";
    vm.insert_program(vm.vheap_bounds.0, prog);
    vm.registers.ac = 0x01;

    for i in 1..8 {
        vm.tick();
        eprintln!("i: {}, ac: {}", 1 << i, vm.registers.ac);

        assert_eq!(vm.registers.ac, 1 << i);
    }
}

#[test]
fn test_vm_instr_bcc() {
    let mut vm = VirtualMachine::new();
    let prog = "90F0";
    vm.insert_program(vm.vheap_bounds.0, prog);
    vm.registers.sr &= status!(Status::Carry);

    vm.tick();
    assert_eq!(vm.registers.pc, 0xF0);
}
