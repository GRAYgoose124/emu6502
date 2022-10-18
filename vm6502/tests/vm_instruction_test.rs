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