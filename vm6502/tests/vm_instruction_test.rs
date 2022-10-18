use vm6502::prelude::*;

#[test]
fn test_vm_instruction_adc_acc() {
    let mut vm = VirtM::new();
    vm.registers.ac = 0x01;

    vm.set_mode(Mode::Accumulator);
    vm.adc();
    assert_eq!(vm.registers.ac, 0x02);

    println!("{:?}", vm);
}

#[test]
fn test_vm_instruction_ora_indx() {
    // INSTR = 0x01
    let mut vm = VirtM::new();
    vm.registers.ac = 0x00;
    vm.registers.x = 0x01;
    vm.flatmap[0x0001] = 0x01;

    vm.ora();
    assert_eq!(vm.registers.ac, 0x01);

    println!("{:?}", vm);
}

#[test]
fn test_vm_instruction_ora_indy() {
    // INSTR = 0x11
    let mut vm = VirtM::new();
    vm.registers.ac = 0x00;
    vm.registers.y = 0x01;
    vm.flatmap[0x0001] = 0x01;

    vm.ora();
    assert_eq!(vm.registers.ac, 0x01);

    println!("{:?}", vm);
}
